use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::solana::solana_rpc_client::SolanaRpcClient;

/// Arguments for the `transfer` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TransferArgs {
    #[schemars(description = "Recipient's public key")]
    to_wallet: String,
    #[schemars(description = "Amount to transfer (in UI units, e.g., 1.5 SOL or tokens)")]
    amount: f64,
    #[schemars(description = "Solana public key of mint account, null for SOL")]
    mint_pubkey: Option<String>,
}

/// Response for the `transfer` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TransferResponse {
    #[schemars(description = "Transaction signature")]
    signature: String,
}

/// Transfers SOL or SPL tokens to another wallet.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `args` - The arguments containing the recipient, amount, and optional mint pubkey.
///
/// # Returns
/// A `Result` containing the transaction signature or an error if the transfer fails.
#[yart::rig_tool(description = "Transfer SOL or SPL token to another wallet")]
async fn transfer(ctx: Arc<SolanaRpcClient>, args: TransferArgs) -> Result<TransferResponse> {
    let to_wallet = Pubkey::from_str(&args.to_wallet)
        .map_err(|e| anyhow::anyhow!("Invalid recipient pubkey: {}", e))?;
    if args.amount <= 0.0 {
        return Err(anyhow::anyhow!("Transfer amount must be positive").into());
    }

    let mint_pubkey = match args.mint_pubkey {
        Some(m) => {
            Some(Pubkey::from_str(&m).map_err(|e| anyhow::anyhow!("Invalid mint pubkey: {}", e))?)
        }
        None => None,
    };

    let signature = ctx.transfer(&to_wallet, args.amount, mint_pubkey).await?;
    Ok(TransferResponse {
        signature: signature.to_string(),
    })
}
