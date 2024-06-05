use std::fmt;

use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Crosschain global config
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct Bridge {
    /// Token mint
    pub token: Pubkey,

    /// Index
    pub index: u64,

    /// Sender
    pub sender: Pubkey,

    /// Recipient
    pub recipient: String,

    /// Amount
    pub amount: u64,

    /// Fee
    pub fee: u64,

    /// Destination
    pub destination: u32,
}

impl fmt::Display for Bridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(to_vec(&self).unwrap()))
    }
}
