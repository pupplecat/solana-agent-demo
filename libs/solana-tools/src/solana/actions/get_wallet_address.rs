use std::sync::Arc;

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

/// Retrieves the wallet's public key as a string.
///
/// # Arguments
/// * `ctx` - The Solana RPC client context.
/// * `_args` - Empty arguments (unused).
///
/// # Returns
/// A `Result` containing the wallet's public key as a string or an error.
#[yart::rig_tool(description = "Get wallet public key")]
async fn get_wallet_address(
    ctx: Arc<SolanaRpcClient>,
    _args: GetWalletAddressArgs,
) -> Result<GetWalletAddressResponse> {
    let pubkey = ctx.get_wallet_pubkey();
    Ok(GetWalletAddressResponse {
        address: pubkey.to_string(),
    })
}
