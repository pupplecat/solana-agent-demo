use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use mcp_core::types::{TextContent, ToolResponseContent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use solana_sdk::signature::Signature;

use crate::solana::solana_rpc_client::SolanaRpcClient;

/// Arguments for the `get_transaction` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetTransactionArgs {
    #[schemars(description = "Transaction signature")]
    signature: String,
}

/// Response for the `get_transaction` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetTransactionResponse {
    #[schemars(description = "Transaction status (e.g., Confirmed, Failed)")]
    status: String,
    #[schemars(description = "Slot number")]
    slot: String,
    #[schemars(description = "Block time (Unix timestamp), null if unavailable")]
    block_time: Option<String>,
}

impl Into<Vec<ToolResponseContent>> for GetTransactionResponse {
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

/// Retrieves details of a transaction by its signature.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `args` - The arguments containing the transaction signature.
///
/// # Returns
/// A `Result` containing the transaction details or an error if the query fails.
async fn get_transaction_inner(
    ctx: Arc<SolanaRpcClient>,
    args: GetTransactionArgs,
) -> Result<GetTransactionResponse> {
    let signature = Signature::from_str(&args.signature)
        .map_err(|e| anyhow::anyhow!("Invalid signature: {}", e))?;

    let tx = ctx.get_transaction(&signature).await?;
    Ok(GetTransactionResponse {
        status: tx.status,
        slot: tx.slot.to_string(),
        block_time: tx.block_time.map_or(None, |b| Some(b.to_string())),
    })
}

#[yart::rig_tool(description = "Get transaction status and details")]
async fn get_transaction_rig(
    ctx: Arc<SolanaRpcClient>,
    args: GetTransactionArgs,
) -> Result<GetTransactionResponse> {
    get_transaction_inner(ctx, args).await
}

#[yart::mcp_tool(description = "Get transaction status and details")]
async fn get_transaction_mcp(
    ctx: Arc<SolanaRpcClient>,
    args: GetTransactionArgs,
) -> Result<GetTransactionResponse> {
    get_transaction_inner(ctx, args).await
}
