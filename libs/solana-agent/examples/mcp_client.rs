use std::{env, time::Duration};

use anyhow::Result;
use mcp_core::{client::ClientBuilder, transport::ClientSseTransportBuilder};

use rig::providers::gemini::{self, Client as GeminiClient};
use solana_agent::app::App;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let mcp_endpoint =
        env::var("MCP_ENDPOINT").map_err(|_| anyhow::anyhow!("MCP Endpoint not set"))?;
    let max_tokens = env::var("MAX_TOKENS")
        .unwrap_or("1048".to_string())
        .parse::<u64>()
        .map_err(|_| anyhow::anyhow!("Invalid MAX_TOKENS value"))?;

    // Create the MCP client
    let mcp_client =
        ClientBuilder::new(ClientSseTransportBuilder::new(mcp_endpoint).build()).build();

    // Retry logic for MCP client connection
    let max_attempts = 5;
    let mut attempt = 1;
    let base_delay = Duration::from_secs(2);

    loop {
        match mcp_client.open().await {
            Ok(_) => {
                println!(
                    "Successfully connected to MCP server on attempt {}",
                    attempt
                );
                break;
            }
            Err(e) => {
                if attempt >= max_attempts {
                    return Err(anyhow::anyhow!(
                        "Failed to connect to MCP server after {} attempts: {}",
                        max_attempts,
                        e
                    ));
                }
                let delay = base_delay * 2u32.pow(attempt - 1); // Exponential backoff: 2s, 4s, 8s, ...
                println!(
                    "Failed to connect to MCP server on attempt {}: {}. Retrying in {:.1}s...",
                    attempt,
                    e,
                    delay.as_secs_f32()
                );
                sleep(delay).await;
                attempt += 1;
            }
        }
    }

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
