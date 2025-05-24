use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use mcp_core::types::{TextContent, ToolResponseContent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use solana_sdk::pubkey::Pubkey;

use crate::solana::solana_rpc_client::SolanaRpcClient;

/// Arguments for the `create_mint` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateMintArgs {
    #[schemars(description = "Number of decimal places (0-255)", with = "i32")]
    decimals: u8,
    #[schemars(description = "Optional mint authority public key")]
    authority: Option<String>,
}

/// Response for the `create_mint` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateMintResponse {
    #[schemars(description = "Public key of the created mint")]
    mint_pubkey: String,
    #[schemars(description = "Transaction signature")]
    signature: String,
}

impl Into<Vec<ToolResponseContent>> for CreateMintResponse {
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

/// Creates a new SPL token mint.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `args` - The arguments containing the decimals and optional authority.
///
/// # Returns
/// A `Result` containing the mint pubkey and transaction signature or an error if the operation fails.
async fn create_mint_inner(
    ctx: Arc<SolanaRpcClient>,
    args: CreateMintArgs,
) -> Result<CreateMintResponse> {
    let authority = match args.authority {
        Some(m) => Some(
            Pubkey::from_str(&m).map_err(|e| anyhow::anyhow!("Invalid mint authority: {}", e))?,
        ),
        None => None,
    };

    let (signature, mint_pubkey) = ctx.create_mint(args.decimals, authority).await?;

    Ok(CreateMintResponse {
        signature: signature.to_string(),
        mint_pubkey: mint_pubkey.to_string(),
    })
}

#[yart::rig_tool(description = "Create a new SPL token mint")]
async fn create_mint_rig(
    ctx: Arc<SolanaRpcClient>,
    args: CreateMintArgs,
) -> Result<CreateMintResponse> {
    create_mint_inner(ctx, args).await
}

#[yart::mcp_tool(description = "Create a new SPL token mint")]
async fn create_mint_mcp(
    ctx: Arc<SolanaRpcClient>,
    args: CreateMintArgs,
) -> Result<CreateMintResponse> {
    create_mint_inner(ctx, args).await
}
