use std::path::Path;

use anyhow::{Context, Result};
use solana_sdk::signature::Keypair;

// Load the keypair from the default Solana CLI keypair path (~/.config/solana/id.json)
// This enables using the same wallet as the Solana CLI tools
pub fn load_keypair<P: AsRef<Path>>(path: P) -> Result<Keypair> {
    // Convert the path to a Path reference
    let keypair_path = path.as_ref();

    // Read the keypair file directly into bytes using serde_json
    // The keypair file is a JSON array of bytes
    let file = std::fs::File::open(&keypair_path)
        .context(format!("Could not open keypair file at {:?}", keypair_path))?;
    let keypair_bytes: Vec<u8> =
        serde_json::from_reader(file).context("Failed to parse keypair file as JSON")?;

    // Create keypair from the loaded bytes
    // This converts the byte array into a keypair
    let keypair =
        Keypair::from_bytes(&keypair_bytes).context("Failed to create Keypair from bytes")?;

    Ok(keypair)
}
