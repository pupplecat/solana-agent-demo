use std::sync::Arc;

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

/// Requests an airdrop of SOL to the wallet on devnet.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `args` - The arguments containing the SOL to request.
///
/// # Returns
/// A `Result` containing the transaction signature or an error if the airdrop fails.
#[yart::rig_tool(description = "Request SOL airdrop for devnet")]
async fn request_airdrop(ctx: Arc<SolanaRpcClient>, args: AirdropArgs) -> Result<AirdropResponse> {
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
