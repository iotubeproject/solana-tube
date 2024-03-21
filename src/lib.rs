pub mod entrypoint;
pub mod error;
pub mod instruction;
// pub mod processor;

// Export current sdk types for downstream users building with a different sdk
// version
pub use solana_program;

/// Seed prefix for Governance  PDAs
/// Note: This prefix is used for the initial set of PDAs and shouldn't be used
/// for any new accounts All new PDAs should use a unique prefix to guarantee
/// uniqueness for each account
pub const PROGRAM_AUTHORITY_SEED: &[u8] = b"governance";
