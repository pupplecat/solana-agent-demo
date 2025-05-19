---
theme: seriph
layout: cover
title: "Solana Agent Demo: Empowering LLMs with Tools"
presenter: true
fonts:
  sans: 'Inter'
  serif: 'Merriweather'
  mono: 'Fira Code'
transition: slide-up
favicon: './assets/solana_logo.png'
background: './assets/cover.png'
---

# Solana Agent Demo: Empowering LLMs with Tools

<div class="text-center mt-8">
  <p class="text-xl text-neutral-300 mt-4">A Terminal-Based Solana Wallet Manager with Gemini AI</p>
  <p class="text-sm text-neutral-400">Presented by pupplecat | Solana Enthusiast | May 19, 2025</p>
</div>

<div class="abs-br m-6">
  <img src="./assets/solana_logo.png" alt="Solana Agent Logo" width="100">
</div>

---
layout: center
title: Why Provide Tools to Agents (LLMs)?
transition: slide-up
---

# Why Provide Tools to Agents (LLMs)?

- **Enhanced Functionality**: LLMs excel at language but lack direct access to external systems (e.g., blockchain).
- **Actionable Responses**: Tools enable agents to perform tasks like checking balances or transferring tokens.
- **Contextual Precision**: Tools provide real-time data, improving response accuracy (e.g., wallet address lookup).
- **User Empowerment**: Combines natural language with actionable outcomes, simplifying complex workflows.

```text

                        Question  ───────────►┌───────┐      Preamble:
                                              │  LLM  │
                        Response  ◄...........└─┬───▲─┘        You are a Solana assistant...
                                                │   :
                                                │   :
                                     json-schema│   :        Tools:
                                              ┌─▼───:─┐
                                              │ AGENT │        - Get Balance
                                              └───────┘        - Create Mint
                                                               - Transfer
                                                               - ...
```

---
layout: default
title: "Introducing the Rig Framework"
transition: slide-up
---

# Introducing the Rig Framework

- **What is Rig?**: A Rust framework for building LLM agents with external tools ([0xPlaygrounds/rig](https://github.com/0xPlaygrounds/rig)).
- **Key Features**:
  - **Agent Builder**: Configure agents with preambles, tools, and models (e.g., Gemini 2.0 Flash).
  - **Tool Integration**: Seamless connection to external APIs or services.
  - **Multi-Turn Context**: Supports conversational memory for better user interactions.

```text

                                        ┌──────────────────────┐
                                        │ rig-core             │
                               1.       │                      │          Preamble:
                  Question  ───────────►│    ┌─────────────┐   │
                                        │    │     LLM     │   │            You are a Solana assistant...
                  Response  ◄...........│    └─┬─▲─┬─▲─┬─▲─┘   │
                               3.       │      │ : │ : │ :     │ 2.
                                        │      │ : │ : │ :   multi-turn   Tools:
                                        │      │ : │ : │ :     │
                                        │    ┌─▼─:─▼─:─▼─:─┐   │            - Get Balance
                                        │    │    AGENT    │   │            - Create Mint
                                        │    └─────────────┘   │            - Transfer
                                        │                      │            - ...
                                        └──────────────────────┘
```

---
layout: default
title: "multi-turn"
transition: slide-up
---

# Rig: multi-turn

```text


               multi-turn
            ┌─────────────┐          Question:   How much SOL do I have? If none, help request 3 airdrop.
            │     LLM     │
            └─┬─▲─┬─▲─┬─▲─┘    Internal Query:   - get_balance
              │ : │ : │ :                        - request_airdrop
 json-schema  │ : │ : │ :
              │ : │ : │ :
            ┌─▼─:─▼─:─▼─:─┐
            │    AGENT    │          Question:   Help creat a mint with 6 decimal point, mint 500 tokens to my wallet
            └─────────────┘                      , and then transfer 250 to wallet Cjbt...

                               Internal Query:   - create_mint
                                                 - mint_to
                                                 - transfer


```

---
layout: two-cols-header
title: Implementing Tools in Solana Agent Demo
transition: slide-up
---

# Implementing Tools in Solana Agent Demo

::left::

- **Tool Setup**:
  - Built using `solana_tools` crate.
  - Examples: `GetWalletAddress`, `GetTokenAccounts`, `Transfer`.
- **Integration**:
  - Tools are attached to the Gemini agent via `rig`.
  - Agent uses tools to fetch data or execute actions.
- **Challenges**:
  - Ensuring tool responses are user-friendly.
  - Managing Solana RPC connections.

::right::

## Code Snippet: Tool Attachment

```rust
let solana_agent = gemini_client
    .agent(gemini::completion::GEMINI_2_0_FLASH)
    .preamble("You are a Solana assistant...")
    .tool(GetWalletAddress::new(solana_rpc_client.clone()))
    .tool(GetTokenAccounts::new(solana_rpc_client.clone()))
    .tool(Transfer::new(solana_rpc_client.clone()))
    .build();
```

---
layout: default
title: "Tool Implementation"
transition: slide-up
---

# Tool example: yart

**yart**: yet another rig tool

```rust
#[yart::rig_tool(description = "Mint tokens to a wallet's associated token account")]
async fn mint_to(ctx: Arc<SolanaRpcClient>, args: MintToArgs) -> Result<MintToResponse> {
    let to_wallet = Pubkey::from_str(&args.to_wallet)
        .map_err(|e| anyhow::anyhow!("Invalid wallet pubkey: {}", e))?;
    let mint = Pubkey::from_str(&args.mint_pubkey)
        .map_err(|e| anyhow::anyhow!("Invalid mint pubkey: {}", e))?;
    if args.amount <= 0.0 {
        return Err(anyhow::anyhow!("Mint amount must be positive").into());
    }

    let signature = ctx.mint_to(&mint, &to_wallet, args.amount).await?;

    Ok(MintToResponse {
        signature: signature.to_string(),
    })
}
```

---
layout: default
title: "Tool Implementation"
transition: slide-up
---

# Tool Arguments

```rust
/// Arguments for the `mint_to` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct MintToArgs {
    #[schemars(description = "Recipient wallet public key")]
    to_wallet: String,
    #[schemars(description = "Mint public key")]
    mint_pubkey: String,
    #[schemars(description = "Amount to mint (in UI units, e.g., 100.5 tokens)")]
    amount: f64,
}

/// Response for the `mint_to` action.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct MintToResponse {
    #[schemars(description = "Transaction signature")]
    signature: String,
}
```


---
layout: image-right
image: ./assets/terminal-screenshot.png
transition: slide-up
---

# Live Demo: Interact with Solana Agent!

1. **Launch the Terminal UI**: Run the `agent_chat` example.
2. **Check Wallet Address**: "what is my wallet address?"
3. **List Token Accounts**: "help me get all token accounts"
4. **Transfer Tokens**: "I want to transfer 50 of [mint] to [wallet]"
5. **Check Balance**: "what is my remain balance?" (context-aware)

[pupplecat/solana-agent-demo](https://github.com/pupplecat/solana-agent-demo)


---
layout: default
title: Achievements and Future
transition: slide-up
---

# Achievements and Future

- **Achievements**:
  - Seamless integration of LLM with Solana tools via `rig`.
  - Context-aware responses (e.g., token inference, balance formatting).
  - Polished terminal UI with `ratatui` (word wrapping, input at bottom).
- **Potential**:
  - Expand to web-based interfaces for broader access.
  - Inspire other blockchain-AI integrations.
- **Future Work**:
  - Add more tools (e.g., NFT management, Trading agent).
  - Multi agent with MCP (Model Context Protocol)

---
layout: center
transition: slide-up
---

# Wrap-Up & Questions

- **Summary**: Solana Agent Demo showcases the power of LLMs with tools!
- **Call to Action**: Open-source under MIT License, try the demo on GitHub.
- **Q&A Invite**: Any questions? I’m happy to dive deeper!

<div class="mt-6 text-center">
  <span class="text-gray-600">Thank you!</span>
</div>