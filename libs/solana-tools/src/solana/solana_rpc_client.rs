use std::time::Duration;
use std::{collections::HashSet, str::FromStr};

use anyhow::Result;
use solana_account_decoder::{UiAccountData, parse_token::UiTokenAccount};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    hash::Hash,
    instruction::Instruction,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use solana_transaction_status_client_types::UiTransactionEncoding;
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token::{
    instruction::{initialize_mint, mint_to},
    state::Mint,
    ui_amount_to_amount,
};
use tokio::time::{sleep, timeout};

use crate::solana::types::TokenAmount;

use super::types::{TokenAccountDetails, TransactionDetails};

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
            let (_, token_program_id) = self.get_mint_account(&mint).await?;
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
            decimals: balance.1,
        })
    }

    /// Requests an airdrop of SOL to the wallet.
    ///
    /// # Arguments
    /// * `amount` - The amount of SOL to request.
    ///
    /// # Returns
    /// A `Result` containing the transaction signature or an error if the airdrop fails or times out.
    pub async fn request_airdrop(&self, amount: f64) -> Result<Signature> {
        if amount <= 0.0 {
            return Err(anyhow::anyhow!("Airdrop amount must be positive"));
        }

        let lamports = ui_amount_to_amount(amount, 9);
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

    /// Creates a new SPL token mint.
    ///
    /// # Arguments
    /// * `decimals` - The number of decimal places for the token.
    /// * `mint_authority` - Optional public key of the mint authority. Defaults to the wallet's pubkey.
    ///
    /// # Returns
    /// A `Result` containing the transaction signature and the mint pubkey, or an error if the operation fails.
    pub async fn create_mint(
        &self,
        decimals: u8,
        mint_authority: Option<Pubkey>,
    ) -> Result<(Signature, Pubkey)> {
        let wallet_pubkey = self.get_wallet_pubkey();
        let mint_authority = mint_authority.unwrap_or(wallet_pubkey);
        let mint_keypair = Keypair::new();
        let mint_pubkey = mint_keypair.pubkey();
        let rent_exempt_balance = self
            .rpc_client
            .get_minimum_balance_for_rent_exemption(Mint::LEN)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get rent-exempt balance: {}", e))?;

        let instructions = vec![
            // Create account for the mint
            system_instruction::create_account(
                &wallet_pubkey,
                &mint_pubkey,
                rent_exempt_balance,
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            // Initialize the mint
            initialize_mint(
                &spl_token::id(),
                &mint_pubkey,
                &mint_authority,
                None, // No freeze authority
                decimals,
            )?,
        ];

        let signature = self
            .process_instructions(&instructions, &vec![&mint_keypair], None)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create mint {}: {}", mint_pubkey, e))?;

        Ok((signature, mint_pubkey))
    }

    /// Mints tokens to a specified wallet's associated token account.
    ///
    /// # Arguments
    /// * `mint` - The public key of the token mint.
    /// * `to_wallet` - The public key of the recipient wallet.
    /// * `amount` - The amount of tokens to mint (in UI amount, e.g., 100.5 tokens).
    ///
    /// # Returns
    /// A `Result` containing the transaction signature, or an error if the operation fails.
    pub async fn mint_to(
        &self,
        mint: &Pubkey,
        to_wallet: &Pubkey,
        amount: f64,
    ) -> Result<Signature> {
        if amount <= 0.0 {
            return Err(anyhow::anyhow!("Mint amount must be positive"));
        }

        let (mint_account, token_program_id) = self.get_mint_account(mint).await?;
        let wallet_pubkey = self.get_wallet_pubkey();
        let to_token_account =
            get_associated_token_address_with_program_id(to_wallet, mint, &token_program_id);

        let mut instructions = if self
            .rpc_client
            .get_account(&to_token_account)
            .await
            .is_err()
        {
            vec![create_associated_token_account(
                &wallet_pubkey,
                to_wallet,
                mint,
                &token_program_id,
            )]
        } else {
            vec![]
        };

        let amount_lamports = ui_amount_to_amount(amount, mint_account.decimals);
        instructions.push(mint_to(
            &token_program_id,
            mint,
            &to_token_account,
            &wallet_pubkey,
            &[],
            amount_lamports,
        )?);

        let signature = self
            .process_instructions(&instructions, &vec![], None)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to mint to {}: {}", to_token_account, e))?;

        Ok(signature)
    }

    /// Creates an associated token account for a wallet and mint.
    ///
    /// # Arguments
    /// * `wallet` - The public key of the wallet to create the ATA for.
    /// * `mint` - The public key of the token mint.
    ///
    /// # Returns
    /// A `Result` containing the transaction signature and the ATA pubkey, or an error if the operation fails.
    pub async fn create_associated_token_account(
        &self,
        wallet: &Pubkey,
        mint: &Pubkey,
    ) -> Result<(Signature, Pubkey)> {
        let account = self.rpc_client.get_account(mint).await?;
        let token_program_id = account.owner;
        let ata_pubkey =
            get_associated_token_address_with_program_id(wallet, mint, &token_program_id);
        let wallet_pubkey = self.get_wallet_pubkey();

        if self.rpc_client.get_account(&ata_pubkey).await.is_ok() {
            return Err(anyhow::anyhow!(
                "Associated token account {} already exists",
                ata_pubkey
            ));
        }

        let instruction =
            create_associated_token_account(&wallet_pubkey, wallet, mint, &token_program_id);

        let signature = self
            .process_instruction(instruction, &vec![], None)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create ATA {}: {}", ata_pubkey, e))?;

        Ok((signature, ata_pubkey))
    }

    /// Retrieves the latest blockhash and its last valid block height.
    ///
    /// # Returns
    /// A `Result` containing the blockhash and last valid block height, or an error if the query fails.
    pub async fn get_block_hash(&self) -> Result<(Hash, u64)> {
        let (blockhash, last_valid_block_height) = self
            .rpc_client
            .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get blockhash: {}", e))?;

        Ok((blockhash, last_valid_block_height))
    }

    /// Retrieves all token accounts owned by the wallet, optionally filtered by mint.
    ///
    /// # Arguments
    /// * `mint_pubkey` - Optional public key of the token mint to filter by.
    ///
    /// # Returns
    /// A `Result` containing a vector of token account details, or an error if the query fails.
    pub async fn get_token_accounts(
        &self,
        mint_pubkey: Option<Pubkey>,
    ) -> Result<Vec<TokenAccountDetails>> {
        let wallet_pubkey = self.get_wallet_pubkey();
        let token_accounts = if let Some(mint) = mint_pubkey {
            self.rpc_client
                .get_token_accounts_by_owner(&wallet_pubkey, TokenAccountsFilter::Mint(mint))
                .await
                .map_err(|e| {
                    anyhow::anyhow!("Failed to get token accounts for mint {}: {}", mint, e)
                })?
        } else {
            self.rpc_client
                .get_token_accounts_by_owner(
                    &wallet_pubkey,
                    TokenAccountsFilter::ProgramId(spl_token::id()),
                )
                .await
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Failed to get token accounts for wallet {}: {}",
                        wallet_pubkey,
                        e
                    )
                })?
        };

        let mut details = vec![];
        for account in token_accounts {
            let UiAccountData::Json(json_data) = &account.account.data else {
                return Err(anyhow::anyhow!("non-JSON token account data returned"));
            };

            let info = json_data
                .parsed
                .get("info")
                .ok_or(anyhow::anyhow!("missing 'info' field"))?;
            let token_account = serde_json::from_value::<UiTokenAccount>(info.clone())?;

            let amount = token_account
                .token_amount
                .amount
                .parse()
                .map_err(|e| anyhow::anyhow!("Failed to parse token amount: {}", e))?;
            let mint = Pubkey::from_str(&token_account.mint)
                .map_err(|e| anyhow::anyhow!("Failed to parse mint pubkey: {}", e))?;
            let (mint_account, _) = self.get_mint_account(&mint).await?;

            details.push(TokenAccountDetails {
                pubkey: Pubkey::from_str(&account.pubkey)
                    .map_err(|e| anyhow::anyhow!("Failed to parse token amount: {}", e))?,
                mint,
                amount,
                decimals: mint_account.decimals,
            });
        }

        Ok(details)
    }

    /// Retrieves details of a transaction by its signature.
    ///
    /// # Arguments
    /// * `signature` - The transaction signature.
    ///
    /// # Returns
    /// A `Result` containing the transaction details, or an error if the query fails.
    pub async fn get_transaction(&self, signature: &Signature) -> Result<TransactionDetails> {
        let transaction = self
            .rpc_client
            .get_transaction_with_config(
                signature,
                solana_client::rpc_config::RpcTransactionConfig {
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                    encoding: Some(UiTransactionEncoding::Json),
                },
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get transaction {}: {}", signature, e))?;

        let status = match transaction.transaction.meta {
            Some(meta) => {
                if meta.err.is_some() {
                    "Failed".to_string()
                } else {
                    "Confirmed".to_string()
                }
            }
            None => "Unknown".to_string(),
        };

        Ok(TransactionDetails {
            status,
            slot: transaction.slot,
            block_time: transaction.block_time,
        })
    }
}
