use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Crosschain global config
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct Config {
    /// Initialized state.
    pub is_initialized: bool,

    /// Owner
    pub owner: Pubkey,

    /// Authority for cToken
    pub authority: Pubkey,

    /// Bridge fee
    pub fee: u64,

    /// Fee collector
    pub fee_collector: Pubkey,
}

/// Crosschain Token
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct CToken {
    /// Initialized state.
    pub is_initialized: bool,

    /// Bump seed used in program address.
    /// The program address is created deterministically with the bump seed,
    /// cToken program id, and cToken account pubkey. This program address has
    /// authority over the cToken's token account and token mint.
    pub bump_seed: u8,

    /// Program ID of the tokens being exchanged.
    pub token_program_id: Pubkey,

    /// CToken config
    pub config: Pubkey,

    /// Token account for cToken bridge
    pub token: Pubkey,
    /// Mint information for token
    pub token_mint: Pubkey,

    /// Bridge chain id
    /// 0: solana
    /// 4689: IoTeX
    pub destination: u32,

    /// Index for bridge instruction
    pub index: u64,

    /// Max amount for bridge
    pub max: u64,

    /// Min amount for bridge
    pub min: u64,
}
