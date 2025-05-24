use std::env;

use anyhow::Result;
use mcp_core::{client::ClientBuilder, transport::ClientSseTransportBuilder};

use rig::{
    completion::Prompt,
    providers::gemini::{self, Client as GeminiClient},
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

    let mcp_endpoint =
        env::var("MCP_ENDPOINT").map_err(|_| anyhow::anyhow!("MCP Endpoint not set"))?;

    // Create the MCP client
    let mcp_client =
        ClientBuilder::new(ClientSseTransportBuilder::new(mcp_endpoint).build()).build();
    // Start the MCP client
    mcp_client.open().await?;

    let init_res = mcp_client.initialize().await?;
    tracing::debug!("Initialized: {:?}", init_res);

    let tools_list_res = mcp_client.list_tools(None, None).await?;
    tracing::debug!("Tools: {:?}", tools_list_res);

    tracing::info!("Building RIG agent");
    let gemini_client = GeminiClient::from_env();
    let mut agent_builder = gemini_client.agent(gemini::completion::GEMINI_2_0_FLASH);

    // Add MCP tools to the agent
    agent_builder = tools_list_res
        .tools
        .into_iter()
        .fold(agent_builder, |builder, tool| {
            builder.mcp_tool(tool, mcp_client.clone().into())
        });
    let agent = agent_builder.build();

    tracing::info!("Prompting RIG agent");
    let response = agent.prompt("What is my wallet address").await?;
    tracing::info!("Agent response: {:?}", response);

    Ok(())
}
