use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::solana::solana_rpc_client::SolanaRpcClient;

/// Arguments for the `get_token_accounts` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetTokenAccountsArgs {
    #[schemars(description = "Optional mint public key to filter by")]
    mint_pubkey: Option<String>,
}

/// Details of a token account.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TokenAccount {
    #[schemars(description = "Token account public key")]
    pubkey: String,
    #[schemars(description = "Mint public key")]
    mint: String,
    #[schemars(description = "Token balance")]
    amount: String,
    #[schemars(description = "Number of decimal places", with = "i32")]
    decimal: u8,
}

/// Response for the `get_token_accounts` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetTokenAccountsResponse {
    #[schemars(description = "List of token accounts")]
    accounts: Vec<TokenAccount>,
}

/// Retrieves all token accounts owned by the wallet, optionally filtered by mint.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `args` - The arguments containing an optional mint pubkey.
///
/// # Returns
/// A `Result` containing a list of token accounts or an error if the query fails.
#[yart::rig_tool(description = "Get token accounts owned by the wallet")]
async fn get_token_accounts(
    ctx: Arc<SolanaRpcClient>,
    args: GetTokenAccountsArgs,
) -> Result<GetTokenAccountsResponse> {
    let mint_pubkey = match args.mint_pubkey {
        Some(m) => {
            Some(Pubkey::from_str(&m).map_err(|e| anyhow::anyhow!("Invalid mint pubkey: {}", e))?)
        }
        None => None,
    };

    let token_accounts = ctx.get_token_accounts(mint_pubkey).await?;
    Ok(GetTokenAccountsResponse {
        accounts: token_accounts
            .iter()
            .map(|a| TokenAccount {
                pubkey: a.pubkey.to_string(),
                mint: a.mint.to_string(),
                amount: a.amount.to_string(),
                decimal: a.decimals,
            })
            .collect(),
    })
}
