use std::sync::Arc;

use anyhow::Result;
use mcp_core::types::{TextContent, ToolResponseContent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::to_value;

use crate::solana::solana_rpc_client::SolanaRpcClient;

/// Arguments for the `get_wallet_address` action (empty).
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetWalletAddressArgs {}

/// Response for the `get_wallet_address` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetWalletAddressResponse {
    #[schemars(description = "Wallet public key")]
    address: String,
}

impl Into<Vec<ToolResponseContent>> for GetWalletAddressResponse {
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

/// Retrieves the wallet's public key as a string.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `_args` - Empty arguments (unused).
///
/// # Returns
/// A `Result` containing the wallet's public key as a string or an error.
async fn get_wallet_address_inner(
    ctx: Arc<SolanaRpcClient>,
    _args: GetWalletAddressArgs,
) -> Result<GetWalletAddressResponse> {
    let pubkey = ctx.get_wallet_pubkey();
    Ok(GetWalletAddressResponse {
        address: pubkey.to_string(),
    })
}

#[yart::rig_tool(description = "Get wallet public key")]
async fn get_wallet_address_rig(
    ctx: Arc<SolanaRpcClient>,
    args: GetWalletAddressArgs,
) -> Result<GetWalletAddressResponse> {
    get_wallet_address_inner(ctx, args).await
}

#[yart::mcp_tool(description = "Get wallet public key")]
async fn get_wallet_address_mcp(
    ctx: Arc<SolanaRpcClient>,
    args: GetWalletAddressArgs,
) -> Result<GetWalletAddressResponse> {
    get_wallet_address_inner(ctx, args).await
}
