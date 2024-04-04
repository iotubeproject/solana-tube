pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

// Export current sdk types for downstream users building with a different sdk
// version
pub use solana_program;

pub const PROGRAM_AUTHORITY_SEED: &[u8] = b"governance";
