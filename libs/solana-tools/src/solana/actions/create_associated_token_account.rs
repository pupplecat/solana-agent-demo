use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use mcp_core::types::{TextContent, ToolResponseContent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use solana_sdk::pubkey::Pubkey;

use crate::solana::solana_rpc_client::SolanaRpcClient;

/// Arguments for the `create_associated_token_account` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateAssociatedTokenAccountArgs {
    #[schemars(description = "Wallet public key")]
    wallet: String,
    #[schemars(description = "Mint public key")]
    mint_pubkey: String,
}

/// Response for the `create_associated_token_account` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateAssociatedTokenAccountResponse {
    #[schemars(description = "Associated token account public key")]
    ata_pubkey: String,
    #[schemars(description = "Transaction signature")]
    signature: String,
}

impl Into<Vec<ToolResponseContent>> for CreateAssociatedTokenAccountResponse {
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

/// Creates an associated token account for a wallet and mint.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `args` - The arguments containing the wallet and mint pubkey.
///
/// # Returns
/// A `Result` containing the ATA pubkey and transaction signature or an error if the operation fails.
async fn create_associated_token_account_inner(
    ctx: Arc<SolanaRpcClient>,
    args: CreateAssociatedTokenAccountArgs,
) -> Result<CreateAssociatedTokenAccountResponse> {
    let mint = Pubkey::from_str(&args.mint_pubkey)
        .map_err(|e| anyhow::anyhow!("Invalid mint pubkey: {}", e))?;
    let wallet = Pubkey::from_str(&args.wallet)
        .map_err(|e| anyhow::anyhow!("Invalid wallet pubkey: {}", e))?;

    let (signature, ata_pubkey) = ctx.create_associated_token_account(&wallet, &mint).await?;

    Ok(CreateAssociatedTokenAccountResponse {
        signature: signature.to_string(),
        ata_pubkey: ata_pubkey.to_string(),
    })
}

#[yart::rig_tool(description = "Create an associated token account for a mint")]
async fn create_associated_token_account_rig(
    ctx: Arc<SolanaRpcClient>,
    args: CreateAssociatedTokenAccountArgs,
) -> Result<CreateAssociatedTokenAccountResponse> {
    create_associated_token_account_inner(ctx, args).await
}

#[yart::mcp_tool(description = "Create an associated token account for a mint")]
async fn create_associated_token_account_mcp(
    ctx: Arc<SolanaRpcClient>,
    args: CreateAssociatedTokenAccountArgs,
) -> Result<CreateAssociatedTokenAccountResponse> {
    create_associated_token_account_inner(ctx, args).await
}
