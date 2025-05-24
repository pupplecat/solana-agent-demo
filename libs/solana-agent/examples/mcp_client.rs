use std::env;

use anyhow::Result;
use mcp_core::{client::ClientBuilder, transport::ClientSseTransportBuilder};

use rig::providers::gemini::{self, Client as GeminiClient};
use solana_agent::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let max_tokens = env::var("MAX_TOKENS")
        .unwrap_or("1048".to_string())
        .parse::<u64>()
        .map_err(|_| anyhow::anyhow!("Invalid MAX_TOKENS value"))?;

    // Create the MCP client
    let mcp_client = ClientBuilder::new(
        ClientSseTransportBuilder::new("http://127.0.0.1:3000/sse".to_string()).build(),
    )
    .build();
    // Start the MCP client
    mcp_client.open().await?;

    let init_res = mcp_client.initialize().await?;
    tracing::debug!("Initialized: {:?}", init_res);

    let tools_list_res = mcp_client.list_tools(None, None).await?;
    tracing::debug!("Tools: {:?}", tools_list_res);

    tracing::info!("Building RIG agent");
    let gemini_client = GeminiClient::from_env();
    let mut agent_builder = gemini_client.agent(gemini::completion::GEMINI_2_0_FLASH).preamble("You are a Solana assistant that helps users manage their wallet on the Solana blockchain. You can check balances, request airdrops, transfer SOL or tokens, create token mints, mint tokens, create associated token accounts, and fetch transaction or blockhash details. Use the provided tools to perform these actions accurately. When a user asks about their 'remaining balance' or 'balance' without specifying a token, infer the most recently mentioned token mint from the conversation history (e.g., from a transfer or token account query) and check the balance for that token. If no token context is available, ask for clarification. When reporting token balances, convert the raw amount to a user-friendly format by dividing the amount by 10 raised to the number of decimals (provided by the mint), and round the result to 2 decimal places for readability (e.g., a raw balance of 949999950 with 6 decimals becomes 949.99).")
        .max_tokens(max_tokens);

    // Add MCP tools to the agent
    agent_builder = tools_list_res
        .tools
        .into_iter()
        .fold(agent_builder, |builder, tool| {
            builder.mcp_tool(tool, mcp_client.clone().into())
        });
    let agent = agent_builder.build();

    color_eyre::install().map_err(|e| anyhow::anyhow!("Failed to initialize color_eyre: {}", e))?;
    let terminal = ratatui::init();
    let app_result = App::new(agent).run(terminal).await;
    ratatui::restore();

    app_result.map_err(|e| anyhow::anyhow!("Application failed: {}", e))?;

    Ok(())
}
