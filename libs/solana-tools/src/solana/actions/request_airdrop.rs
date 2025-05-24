use std::sync::Arc;

use anyhow::Result;
use mcp_core::types::{TextContent, ToolResponseContent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::to_value;

use crate::solana::solana_rpc_client::SolanaRpcClient;

/// Arguments for the `request_airdrop` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AirdropArgs {
    #[schemars(description = "Amount of SOL to request (maximum 5 SOL)")]
    amount: f64,
}

/// Response for the `request_airdrop` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AirdropResponse {
    #[schemars(description = "Transaction signature")]
    signature: String,
}

impl Into<Vec<ToolResponseContent>> for AirdropResponse {
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

/// Requests an airdrop of SOL to the wallet on devnet.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `args` - The arguments containing the SOL to request.
///
/// # Returns
/// A `Result` containing the transaction signature or an error if the airdrop fails.
async fn request_airdrop_inner(
    ctx: Arc<SolanaRpcClient>,
    args: AirdropArgs,
) -> Result<AirdropResponse> {
    if args.amount <= 0.0 {
        return Err(anyhow::anyhow!("Airdrop amount must be positive").into());
    }
    if args.amount > 5.0 {
        return Err(anyhow::anyhow!("Airdrop amount exceeds maximum of 5 SOL").into());
    }

    let signature = ctx.request_airdrop(args.amount).await?;
    Ok(AirdropResponse {
        signature: signature.to_string(),
    })
}

#[yart::rig_tool(description = "Request SOL airdrop for devnet")]
async fn request_airdrop_rig(
    ctx: Arc<SolanaRpcClient>,
    args: AirdropArgs,
) -> Result<AirdropResponse> {
    request_airdrop_inner(ctx, args).await
}

#[yart::mcp_tool(description = "Request SOL airdrop for devnet")]
async fn request_airdrop_mcp(
    ctx: Arc<SolanaRpcClient>,
    args: AirdropArgs,
) -> Result<AirdropResponse> {
    request_airdrop_inner(ctx, args).await
}
