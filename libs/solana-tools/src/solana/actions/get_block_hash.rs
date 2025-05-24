use std::sync::Arc;

use anyhow::Result;
use mcp_core::types::{TextContent, ToolResponseContent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::to_value;

use crate::solana::solana_rpc_client::SolanaRpcClient;

/// Arguments for the `get_blockhash` action (empty).
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetBlockhashArgs {}

/// Response for the `get_blockhash` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetBlockhashResponse {
    #[schemars(description = "Latest blockhash")]
    blockhash: String,
    #[schemars(description = "Last valid block height")]
    last_valid_block_height: String,
}

impl Into<Vec<ToolResponseContent>> for GetBlockhashResponse {
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

/// Retrieves the latest blockhash and its last valid block height.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `_args` - Empty arguments (unused).
///
/// # Returns
/// A `Result` containing the blockhash and last valid block height or an error if the query fails.
async fn get_blockhash_inner(
    ctx: Arc<SolanaRpcClient>,
    _args: GetBlockhashArgs,
) -> Result<GetBlockhashResponse> {
    let (hash, block_height) = ctx.get_block_hash().await?;
    Ok(GetBlockhashResponse {
        blockhash: hash.to_string(),
        last_valid_block_height: block_height.to_string(),
    })
}

#[yart::rig_tool(description = "Get the latest blockhash")]

async fn get_blockhash_rig(
    ctx: Arc<SolanaRpcClient>,
    args: GetBlockhashArgs,
) -> Result<GetBlockhashResponse> {
    get_blockhash_inner(ctx, args).await
}

#[yart::mcp_tool(description = "Get the latest blockhash")]
async fn get_blockhash_mcp(
    ctx: Arc<SolanaRpcClient>,
    args: GetBlockhashArgs,
) -> Result<GetBlockhashResponse> {
    get_blockhash_inner(ctx, args).await
}
