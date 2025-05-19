use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::solana::solana_rpc_client::SolanaRpcClient;

/// Arguments for the `get_balance` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetBalanceArgs {
    #[schemars(description = "Solana public key of mint account, null for SOL")]
    mint_pubkey: Option<String>,
}

/// Response for the `get_balance` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetBalanceResponse {
    #[schemars(description = "Balance amount")]
    amount: String,
    #[schemars(description = "Number of decimal places", with = "i32")]
    decimal: u8,
}

/// Retrieves the wallet's balance for SOL or an SPL token.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `args` - The arguments containing an optional mint pubkey.
///
/// # Returns
/// A `Result` containing the balance as a `GetBalanceResponse` or an error if the query fails.
#[yart::rig_tool(description = "Get wallet balance for SOL or SPL token")]
async fn get_balance(
    ctx: Arc<SolanaRpcClient>,
    args: GetBalanceArgs,
) -> Result<GetBalanceResponse> {
    let mint_pubkey = match args.mint_pubkey {
        Some(m) => {
            Some(Pubkey::from_str(&m).map_err(|e| anyhow::anyhow!("Invalid mint pubkey: {}", e))?)
        }
        None => None,
    };

    let token_amount = ctx.get_balance(mint_pubkey).await?;
    Ok(GetBalanceResponse {
        amount: token_amount.amount.to_string(),
        decimal: token_amount.decimals,
    })
}
