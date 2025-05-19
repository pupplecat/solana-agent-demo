use std::collections::HashSet;
use std::time::Duration;

use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token::{state::Mint, ui_amount_to_amount};
use tokio::time::{sleep, timeout};

use crate::solana::types::TokenAmount;

/// A client for interacting with the Solana network using an RPC endpoint and a wallet keypair.

pub struct SolanaRpcClient {
    pub rpc_client: RpcClient,
    pub wallet: Keypair,
}

impl SolanaRpcClient {
    /// Creates a new `SolanaRpcClient` with the provided RPC client and wallet keypair.
    pub fn new(rpc_client: RpcClient, wallet: Keypair) -> Self {
        Self { rpc_client, wallet }
    }

    /// Returns the public key of the wallet associated with this client.
    pub fn get_wallet_pubkey(&self) -> Pubkey {
        self.wallet.pubkey()
    }

    /// Processes a single instruction on the Solana network.
    ///
    /// # Arguments
    /// * `instruction` - The instruction to process.
    /// * `signers` - A vector of keypairs that will sign the transaction.
    /// * `payer` - Optional pubkey of the account paying for transaction fees. Defaults to the wallet's pubkey.
    ///
    /// # Returns
    /// A `Result` containing the transaction signature or an error if the transaction fails.
    pub async fn process_instruction(
        &self,
        instruction: Instruction,
        signers: &Vec<&Keypair>,
        payer: Option<&Pubkey>,
    ) -> Result<Signature> {
        self.process_instructions(&[instruction], signers, payer)
            .await
    }

    /// Processes multiple instructions in a single transaction on the Solana network.
    ///
    /// # Arguments
    /// * `instructions` - A slice of instructions to process.
    /// * `signers` - A vector of keypairs that will sign the transaction.
    /// * `payer` - Optional pubkey of the account paying for transaction fees. Defaults to the wallet's pubkey.
    ///
    /// # Returns
    /// A `Result` containing the transaction signature or an error if the transaction fails.
    pub async fn process_instructions(
        &self,
        instructions: &[Instruction],
        signers: &Vec<&Keypair>,
        payer: Option<&Pubkey>,
    ) -> Result<Signature> {
        // Get recent blockhash
        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get recent blockhash: {}", e))?;

        let wallet_pubkey = self.get_wallet_pubkey();
        let actual_payer = payer.unwrap_or(&wallet_pubkey);

        // Deduplicate signers and ensure wallet is included if it's the payer
        let mut unique_signers: Vec<&Keypair> = vec![];
        if actual_payer == &wallet_pubkey {
            unique_signers.push(&self.wallet);
        }
        let signer_pubkeys: HashSet<Pubkey> = unique_signers.iter().map(|s| s.pubkey()).collect();
        for signer in signers {
            if !signer_pubkeys.contains(&signer.pubkey()) {
                unique_signers.push(signer);
            }
        }

        // Create and sign transaction
        let tx = Transaction::new_signed_with_payer(
            instructions,
            Some(actual_payer),
            &unique_signers,
            recent_blockhash,
        );

        // Send and confirm transaction
        let signature = self
            .rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send transaction: {}", e))?;

        Ok(signature)
    }

    /// Retrieves and unpacks an account's data into a specified type.
    ///
    /// # Arguments
    /// * `pubkey` - The public key of the account to query.
    ///
    /// # Returns
    /// A `Result` containing the unpacked account data or an error if the account doesn't exist or unpacking fails.
    pub async fn get_packed_account<T: Pack + IsInitialized>(&self, pubkey: &Pubkey) -> Result<T> {
        let account = self
            .rpc_client
            .get_account(pubkey)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get account {}: {}", pubkey, e))?;

        T::unpack(&account.data[..])
            .map_err(|e| anyhow::anyhow!("Failed to unpack account {}: {}", pubkey, e))
    }

    /// Retrieves and unpacks a mint account's data.
    ///
    /// # Arguments
    /// * `mint_pubkey` - The public key of the mint account.
    ///
    /// # Returns
    /// A `Result` containing the mint data and its program ID, or an error if the account doesn't exist or unpacking fails.
    pub async fn get_mint_account(&self, mint_pubkey: &Pubkey) -> Result<(Mint, Pubkey)> {
        let account = self
            .rpc_client
            .get_account(mint_pubkey)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get mint account {}: {}", mint_pubkey, e))?;
        let mint = Mint::unpack(&account.data[..])
            .map_err(|e| anyhow::anyhow!("Failed to unpack mint account {}: {}", mint_pubkey, e))?;
        Ok((mint, account.owner))
    }

    /// Retrieves the balance of the wallet for native SOL or a specific SPL token.
    ///
    /// # Arguments
    /// * `mint_pubkey` - Optional public key of the token mint. If `None`, returns the SOL balance.
    ///
    /// # Returns
    /// A `Result` containing the balance as a `TokenAmount` or an error if the query fails.
    pub async fn get_balance(&self, mint_pubkey: Option<Pubkey>) -> Result<TokenAmount> {
        let wallet_pubkey = self.get_wallet_pubkey();
        let balance = if let Some(mint) = mint_pubkey {
            let (mint_account, token_program_id) = self.get_mint_account(&mint).await?;
            let token_account_pubkey = get_associated_token_address_with_program_id(
                &wallet_pubkey,
                &mint,
                &token_program_id,
            );

            let balance = self
                .rpc_client
                .get_token_account(&token_account_pubkey)
                .await
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Failed to get token account {}: {}",
                        token_account_pubkey,
                        e
                    )
                })?;

            if let Some(ta) = balance {
                (ta.token_amount.amount.parse()?, ta.token_amount.decimals)
            } else {
                return Err(anyhow::anyhow!(
                    "Token account {} does not exist",
                    token_account_pubkey
                ));
            }
        } else {
            let balance = self
                .rpc_client
                .get_balance(&wallet_pubkey)
                .await
                .map_err(|e| {
                    anyhow::anyhow!("Failed to get SOL balance for {}: {}", wallet_pubkey, e)
                })?;
            (balance, 9)
        };

        Ok(TokenAmount {
            amount: balance.0,
            decimal: balance.1,
        })
    }

    /// Requests an airdrop of SOL to the wallet.
    ///
    /// # Arguments
    /// * `lamports` - The amount of lamports to request.
    ///
    /// # Returns
    /// A `Result` containing the transaction signature or an error if the airdrop fails or times out.
    pub async fn request_airdrop(&self, lamports: u64) -> Result<Signature> {
        let wallet_pubkey = self.get_wallet_pubkey();
        let signature = self
            .rpc_client
            .request_airdrop(&wallet_pubkey, lamports)
            .await
            .map_err(|e| {
                anyhow::anyhow!("Failed to request airdrop for {}: {}", wallet_pubkey, e)
            })?;

        let confirmed = timeout(Duration::from_secs(30), async {
            loop {
                let confirmed = self
                    .rpc_client
                    .confirm_transaction(&signature)
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to confirm airdrop: {}", e));
                if confirmed.is_ok() && confirmed.unwrap() {
                    break true;
                }
                sleep(Duration::from_millis(500)).await;
            }
        })
        .await
        .map_err(|_| {
            anyhow::anyhow!("Airdrop confirmation timed out for signature {}", signature)
        })?;

        if !confirmed {
            return Err(anyhow::anyhow!(
                "Airdrop failed to confirm for signature {}",
                signature
            ));
        }

        Ok(signature)
    }

    /// Transfers SOL or SPL tokens to another wallet.
    ///
    /// # Arguments
    /// * `to_wallet_pubkey` - The recipient's public key.
    /// * `amount` - The amount to transfer (in UI amount, e.g., 1.5 SOL or tokens).
    /// * `mint_pubkey` - Optional public key of the token mint. If `None`, transfers SOL.
    ///
    /// # Returns
    /// A `Result` containing the transaction signature or an error if the transfer fails.
    pub async fn transfer(
        &self,
        to_wallet_pubkey: &Pubkey,
        amount: f64,
        mint_pubkey: Option<Pubkey>,
    ) -> Result<Signature> {
        if amount <= 0.0 {
            return Err(anyhow::anyhow!("Transfer amount must be positive"));
        }

        let signature = if let Some(mint) = mint_pubkey {
            let (mint_account, token_program_id) = self.get_mint_account(&mint).await?;
            let from_wallet_pubkey = self.get_wallet_pubkey();
            let token_account_pubkey = get_associated_token_address_with_program_id(
                &from_wallet_pubkey,
                &mint,
                &token_program_id,
            );

            // Check source balance
            let source_balance = self.get_balance(Some(mint)).await?;
            let amount_lamports = ui_amount_to_amount(amount, mint_account.decimals);
            if source_balance.amount < amount_lamports {
                return Err(anyhow::anyhow!(
                    "Insufficient balance: {} available, {} required",
                    source_balance.amount,
                    amount_lamports
                ));
            }

            let to_token_account_pubkey = get_associated_token_address_with_program_id(
                to_wallet_pubkey,
                &mint,
                &token_program_id,
            );

            let mut ixs = if self
                .rpc_client
                .get_account(&to_token_account_pubkey)
                .await
                .is_err()
            {
                vec![create_associated_token_account(
                    &from_wallet_pubkey,
                    to_wallet_pubkey,
                    &mint,
                    &token_program_id,
                )]
            } else {
                vec![]
            };

            ixs.push(spl_token::instruction::transfer(
                &token_program_id,
                &token_account_pubkey,
                &to_token_account_pubkey,
                &from_wallet_pubkey,
                &[],
                amount_lamports,
            )?);

            self.process_instructions(&ixs, &vec![], None).await?
        } else {
            let lamports = ui_amount_to_amount(amount, 9);
            let ix =
                system_instruction::transfer(&self.get_wallet_pubkey(), to_wallet_pubkey, lamports);

            self.process_instruction(ix, &vec![], None).await?
        };

        Ok(signature)
    }
}
