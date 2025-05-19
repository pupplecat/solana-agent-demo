use solana_sdk::pubkey::Pubkey;

// Detauls of a token amount
pub struct TokenAmount {
    pub amount: u64,
    pub decimals: u8,
}

/// Details of a token account.
#[derive(Debug, Clone)]
pub struct TokenAccountDetails {
    pub pubkey: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub decimals: u8,
}

/// Details of a transaction.
#[derive(Debug, Clone)]
pub struct TransactionDetails {
    pub status: String,
    pub slot: u64,
    pub block_time: Option<i64>,
}
