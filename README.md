# Solana Agent Demo

A terminal-based Solana wallet management interface powered by a Gemini AI agent. This project allows users to interact with their Solana wallet, manage tokens, request airdrops, transfer funds, and more, all through a conversational UI.

## Overview

This demo showcases a Rust application using the `ratatui` library for a terminal UI and the `rig` framework to integrate a Gemini AI agent. The agent assists with Solana blockchain operations, leveraging tools like token account management, balance checks, and transactions. The interface supports word-wrapped messages, context-aware responses, and a user-friendly layout with the input box at the bottom.

## Prerequisites

- Rust (latest stable version)
- Solana CLI (for wallet setup)
- An API key for Gemini (from Google AI Studio)
- Environment variable management (e.g., `direnv` or manual `.env` file)

## Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/pupplecat/solana-agent-demo.git
   cd solana-agent-demo
   ```

2. **Install Dependencies**

   ```bash
   cargo build
   ```

3. **Set Up Environment Variables**
   Create a `.env` file in the project root with the following:

   ```env
   WALLET_KEYPAIR=/path/to/your/wallet.json
   RPC_URL=https://api.devnet.solana.com
   GEMINI_API_KEY=your_gemini_api_key
   MAX_TOKENS=1048
   ```

   - Replace `/path/to/your/wallet.json` with the path to your Solana keypair file.
   - Obtain the `GEMINI_API_KEY` from Google AI Studio and set `MAX_TOKENS` as needed.
MAX_TOKENS=1048
4. **Initialize Solana Wallet**
   Ensure your wallet is configured with the Solana CLI:

   ```bash
   solana-keygen new -o ./keys/wallet.json
   ```

   Export your keypair as a JSON file and update the `WALLET_KEYPAIR` path.

## Usage

1. **Run the Application**

   ```bash
   cargo run --example agent_chat
   ```

2. **Interact with the UI**
   - Press `e` to enter editing mode and type commands (e.g., "what is my wallet address?", "help me get all token accounts").
   - Press `Enter` to submit a command.
   - Press `Esc` to exit editing mode or cancel processing.
   - Use `↑/↓` to scroll through message history.
   - Press `q` to quit the application.

3. **Example Commands**
   - "what is my wallet address?" - Displays your Solana wallet address.
   - "How much SOL do I have? If none, help request 3 airdrop" - Check SOL balance, and request airdrop if balance is zero
   - "help me get all token accounts" - Lists token accounts and their balances.
   - "I want to transfer 50 of [mint] to [wallet]" - Initiates a token transfer.
   - "what is my remain balance?" - Checks the remaining balance of the last-mentioned token.

## Features

- **Conversational AI**: Uses Gemini to interpret and respond to commands.
- **Token Management**: Supports checking balances, transferring tokens, and minting.
- **User-Friendly Formatting**: Converts raw token balances (e.g., 949999950) to readable formats (e.g., 949.99) based on decimals.
- **Context Awareness**: Infers token context from previous commands (e.g., after a transfer).
- **Terminal UI**: Word-wrapped messages, scrollable history, and input at the bottom.

## Troubleshooting

- **Compilation Errors**: Ensure all dependencies are installed and Rust is up to date (`rustup update`).
- **API Key Issues**: Verify your `GEMINI_API_KEY` is valid and has sufficient quota.
- **Wallet Errors**: Check your `WALLET_KEYPAIR` file path and Solana RPC connection.
- **Long Messages**: If messages don't wrap, resize the terminal or report a bug.

## Contributing

Feel free to submit issues or pull requests on the GitHub repository. Contributions to enhance the UI, add new tools, or improve agent responses are welcome!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Resources

- [Solana Documentation](https://docs.solana.com/)
- [Gemini API](https://ai.google.dev/gemini-api)
- [ratatui GitHub](https://github.com/ratatui/ratatui)
- [rig Framework](https://github.com/0xPlaygrounds/rig)

## Acknowledgments

Thanks to the Solana, ratatui, and rig communities for their tools and support.
