use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use mcp_core::types::{TextContent, ToolResponseContent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use solana_sdk::pubkey::Pubkey;

use crate::solana::solana_rpc_client::SolanaRpcClient;

/// Arguments for the `mint_to` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct MintToArgs {
    #[schemars(description = "Recipient wallet public key")]
    to_wallet: String,
    #[schemars(description = "Mint public key")]
    mint_pubkey: String,
    #[schemars(description = "Amount to mint (in UI units, e.g., 100.5 tokens)")]
    amount: f64,
}

/// Response for the `mint_to` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct MintToResponse {
    #[schemars(description = "Transaction signature")]
    signature: String,
}

impl Into<Vec<ToolResponseContent>> for MintToResponse {
    fn into(self) -> Vec<ToolResponseContent> {
        let content =
            to_value(self).map_or(format!("Serializing response error"), |f| f.to_string());

        vec![ToolResponseContent::Text(TextContent {
            content_type: "text".to_string(),
            text: content,
            annotations: None,
        })]
    }
}

/// Mints tokens to a specified wallet's associated token account.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `args` - The arguments containing the recipient wallet, mint, and amount.
///
/// # Returns
/// A `Result` containing the transaction signature or an error if the operation fails.
async fn mint_to_inner(ctx: Arc<SolanaRpcClient>, args: MintToArgs) -> Result<MintToResponse> {
    let to_wallet = Pubkey::from_str(&args.to_wallet)
        .map_err(|e| anyhow::anyhow!("Invalid wallet pubkey: {}", e))?;
    let mint = Pubkey::from_str(&args.mint_pubkey)
        .map_err(|e| anyhow::anyhow!("Invalid mint pubkey: {}", e))?;
    if args.amount <= 0.0 {
        return Err(anyhow::anyhow!("Mint amount must be positive").into());
    }

    let signature = ctx.mint_to(&mint, &to_wallet, args.amount).await?;

    Ok(MintToResponse {
        signature: signature.to_string(),
    })
}

#[yart::rig_tool(description = "Mint tokens to a wallet's associated token account")]
async fn mint_to_rig(ctx: Arc<SolanaRpcClient>, args: MintToArgs) -> Result<MintToResponse> {
    mint_to_inner(ctx, args).await
}

#[yart::mcp_tool(description = "Mint tokens to a wallet's associated token account")]
async fn mint_to_mcp(ctx: Arc<SolanaRpcClient>, args: MintToArgs) -> Result<MintToResponse> {
    mint_to_inner(ctx, args).await
}
