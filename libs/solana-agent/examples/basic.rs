use std::{env, sync::Arc};

use anyhow::Result;
use tracing::info;

use rig::{
    agent::PromptRequest,
    providers::gemini::{self, Client as GeminiClient},
};
use solana_agent::utils::load_keypair;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_tools::{
    actions::{
        CreateAssociatedTokenAccountRig, CreateMintRig, GetBalanceRig, GetBlockhashRig,
        GetTokenAccountsRig, GetTransactionRig, GetWalletAddressRig, MintToRig, RequestAirdropRig,
        TransferRig,
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
    let max_tokens = env::var("MAX_TOKENS")
        .unwrap_or("1048".to_string())
        .parse::<u64>()
        .map_err(|_| anyhow::anyhow!("Invalid MAX_TOKENS value"))?;

    // Create Gemini client
    let wallet_keypair = load_keypair(&wallet_keypair_path)?;
    let rpc_client = RpcClient::new(rpc_url);
    let gemini_client = GeminiClient::from_env();

    let solana_rpc_client = Arc::new(SolanaRpcClient::new(rpc_client, wallet_keypair));

    // Create agent with a single context prompt and tools
    let solana_agent = gemini_client
        .agent(gemini::completion::GEMINI_2_0_FLASH)
        .preamble("You are a Solana assistant that helps users manage their wallet on the Solana blockchain. You can check balances, request airdrops, transfer SOL or tokens, create token mints, mint tokens, create associated token accounts, and fetch transaction or blockhash details. Use the provided tools to perform these actions accurately.")
        .max_tokens(max_tokens)
        .tool(CreateAssociatedTokenAccountRig::new(solana_rpc_client.clone()))
        .tool(CreateMintRig::new(solana_rpc_client.clone()))
        .tool(GetBalanceRig::new(solana_rpc_client.clone()))
        .tool(GetBlockhashRig::new(solana_rpc_client.clone()))
        .tool(GetTokenAccountsRig::new(solana_rpc_client.clone()))
        .tool(GetTransactionRig::new(solana_rpc_client.clone()))
        .tool(GetWalletAddressRig::new(solana_rpc_client.clone()))
        .tool(MintToRig::new(solana_rpc_client.clone()))
        .tool(RequestAirdropRig::new(solana_rpc_client.clone()))
        .tool(TransferRig::new(solana_rpc_client.clone()))
        .build();

    info!("Solana agent initialized");

    // Prompt the agent and verify responses
    let prompts = vec!["What is my wallet address"];

    for prompt in prompts {
        println!("Prompt: {}", prompt);
        let ret = PromptRequest::new(&solana_agent, prompt)
            .multi_turn(5)
            .await?;
        println!("Gemini Agent: {}", ret);
    }

    Ok(())
}
