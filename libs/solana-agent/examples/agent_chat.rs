use std::{env, sync::Arc};

use anyhow::Result;

use rig::providers::gemini::{self, Client as GeminiClient};
use solana_agent::{app::App, utils::load_keypair};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_tools::{
    actions::{
        CreateAssociatedTokenAccountRig, CreateMintRig, GetBalanceRig, GetBlockhashRig,
        GetTokenAccountsRig, GetTransactionRig, GetWalletAddressRig, MintToRig, RequestAirdropRig,
        TransferRig,
    },
    solana_rpc_client::SolanaRpcClient,
};

#[tokio::main]
async fn main() -> Result<()> {
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
        .preamble("You are a Solana assistant that helps users manage their wallet on the Solana blockchain. You can check balances, request airdrops, transfer SOL or tokens, create token mints, mint tokens, create associated token accounts, and fetch transaction or blockhash details. Use the provided tools to perform these actions accurately. When a user asks about their 'remaining balance' or 'balance' without specifying a token, infer the most recently mentioned token mint from the conversation history (e.g., from a transfer or token account query) and check the balance for that token. If no token context is available, ask for clarification. When reporting token balances, convert the raw amount to a user-friendly format by dividing the amount by 10 raised to the number of decimals (provided by the mint), and round the result to 2 decimal places for readability (e.g., a raw balance of 949999950 with 6 decimals becomes 949.99).")
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

    color_eyre::install().map_err(|e| anyhow::anyhow!("Failed to initialize color_eyre: {}", e))?;
    let terminal = ratatui::init();
    let app_result = App::new(solana_agent).run(terminal).await;
    ratatui::restore();

    app_result.map_err(|e| anyhow::anyhow!("Application failed: {}", e))?;

    Ok(())
}
