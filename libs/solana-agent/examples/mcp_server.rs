use std::{env, sync::Arc};

use anyhow::Result;
use mcp_core::{
    server::Server,
    transport::ServerSseTransport,
    types::{ProtocolVersion, ServerCapabilities, ToolCapabilities},
};

use solana_agent::utils::load_keypair;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_tools::{
    actions::{
        CreateAssociatedTokenAccountMcp, CreateMintMcp, GetBalanceMcp, GetBlockhashMcp,
        GetTokenAccountsMcp, GetTransactionMcp, GetWalletAddressMcp, MintToMcp, RequestAirdropMcp,
        TransferMcp,
    },
    solana_rpc_client::SolanaRpcClient,
};

// Setup function to initialize tracing only once
pub fn init_tracing() {
    // Use a static Once to ensure initialization happens once
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_target(false)
            .init();
    });
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    dotenvy::dotenv().ok();

    let wallet_keypair_path = env::var("WALLET_KEYPAIR")
        .map_err(|_| anyhow::anyhow!("WALLET_KEYPAIR environment variable not set"))?;
    let rpc_url =
        env::var("RPC_URL").map_err(|_| anyhow::anyhow!("RPC_URL environment variable not set"))?;

    // Create Gemini client
    let wallet_keypair = load_keypair(&wallet_keypair_path)?;
    let rpc_client = RpcClient::new(rpc_url);

    let solana_rpc_client = Arc::new(SolanaRpcClient::new(rpc_client, wallet_keypair));

    // Create the MCP server
    let mcp_server_protocol = Server::builder(
        "add".to_string(),
        "1.0".to_string(),
        ProtocolVersion::V2025_03_26,
    )
    .set_capabilities(ServerCapabilities {
        tools: Some(ToolCapabilities::default()),
        ..Default::default()
    })
    .register_tool(
        CreateAssociatedTokenAccountMcp::tool(),
        CreateAssociatedTokenAccountMcp::call(solana_rpc_client.clone()),
    )
    .register_tool(
        CreateMintMcp::tool(),
        CreateMintMcp::call(solana_rpc_client.clone()),
    )
    .register_tool(
        GetBalanceMcp::tool(),
        GetBalanceMcp::call(solana_rpc_client.clone()),
    )
    .register_tool(
        GetBlockhashMcp::tool(),
        GetBlockhashMcp::call(solana_rpc_client.clone()),
    )
    .register_tool(
        GetTokenAccountsMcp::tool(),
        GetTokenAccountsMcp::call(solana_rpc_client.clone()),
    )
    .register_tool(
        GetTransactionMcp::tool(),
        GetTransactionMcp::call(solana_rpc_client.clone()),
    )
    .register_tool(
        GetWalletAddressMcp::tool(),
        GetWalletAddressMcp::call(solana_rpc_client.clone()),
    )
    .register_tool(
        MintToMcp::tool(),
        MintToMcp::call(solana_rpc_client.clone()),
    )
    .register_tool(
        RequestAirdropMcp::tool(),
        RequestAirdropMcp::call(solana_rpc_client.clone()),
    )
    .register_tool(
        TransferMcp::tool(),
        TransferMcp::call(solana_rpc_client.clone()),
    )
    .build();

    let mcp_server_transport =
        ServerSseTransport::new("0.0.0.0".to_string(), 3000, mcp_server_protocol);

    let server_result = Server::start(mcp_server_transport).await;

    server_result.map_err(|e| anyhow::anyhow!("Application failed: {}", e))?;

    Ok(())
}
