use std::sync::Arc;

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::solana::solana_rpc_client::SolanaRpcClient;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetWalletAddressArgs {}

#[yart::rig_tool(description = "Get wallet address")]
async fn get_wallet_address(
    ctx: Arc<SolanaRpcClient>,
    _args: GetWalletAddressArgs,
) -> Result<String> {
    let pubkey = ctx.get_wallet_pubkey();
    Ok(pubkey.to_string())
}
