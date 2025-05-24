# Use the official Rust image as the base
FROM rust:1.86 AS builder

# Set working directory
WORKDIR /usr/src/solana-agent-demo

# Copy the project files
COPY . .

# Build the examples
RUN cargo build --release --example agent_chat
RUN cargo build --release --example mcp_server
RUN cargo build --release --example mcp_client

# Use a smaller base image for the final container
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built example binary from the builder stage
# The binary will be in target/release/examples/
COPY --from=builder /usr/src/solana-agent-demo/target/release/examples/agent_chat /usr/local/bin/agent_chat
COPY --from=builder /usr/src/solana-agent-demo/target/release/examples/mcp_server /usr/local/bin/mcp_server
COPY --from=builder /usr/src/solana-agent-demo/target/release/examples/mcp_client /usr/local/bin/mcp_client

# Set the working directory
WORKDIR /app

# Command to run the agent_chat example
CMD ["agent_chat"]