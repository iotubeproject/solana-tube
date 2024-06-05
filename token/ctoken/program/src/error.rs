use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the cToken program.
#[derive(Clone, Debug, PartialEq, Eq, Error, FromPrimitive)]
pub enum CTokenError {
    /// Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,
    /// The account cannot be initialized because it is already being used.
    #[error("cToken account already in use")]
    AlreadyInUse,
    /// Invalid program address
    #[error("Invalid program address generated from bump seed and key")]
    InvalidProgramAddress,
    /// Invalid program id
    #[error("The provided token program does not match the token program expected by the cToken")]
    IncorrectTokenProgramId,
    /// The deserialization of the account returned something besides
    /// State::Mint.
    #[error("Deserialized account is not an SPL Token mint")]
    ExpectedMint,
    /// The deserialization of the account returned something besides
    /// State::Account.
    #[error("Deserialized account is not an SPL Token account")]
    ExpectedAccount,
    /// Invalid token account
    #[error("Invalid token account")]
    InvalidToken,
    /// Invalid input
    #[error("Invalid token account")]
    InvalidInput,
    /// Invalid owner
    #[error("Invalid cToken owner")]
    InvalidOwner,
    /// Invalid config
    #[error("Invalid config")]
    InvalidConfig,
    /// Invalid owner
    #[error("Invalid mint")]
    InvalidMint,
    /// Invalid Authority
    #[error("Invalid mint")]
    InvalidAuthority,
    /// Invalid Invalid Fee Collector
    #[error("Invalid fee collector")]
    InvalidFeeCollector,
    /// Invalid Amount
    #[error("Invalid amount")]
    InvalidAmount,
}
impl From<CTokenError> for ProgramError {
    fn from(e: CTokenError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for CTokenError {
    fn type_of() -> &'static str {
        "CTokenError"
    }
}

impl PrintProgramError for CTokenError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            CTokenError::InvalidInstruction => msg!("Error: Invalid instruction"),
            CTokenError::AlreadyInUse => msg!("Error: Swap account already in use"),
            CTokenError::InvalidProgramAddress => {
                msg!("Error: Invalid program address generated from bump seed and key")
            }
            CTokenError::IncorrectTokenProgramId => {
                msg!("Error: The provided token program does not match the token program expected by the cToken")
            }
            CTokenError::ExpectedMint => {
                msg!("Error: Deserialized account is not an SPL Token mint")
            }
            CTokenError::ExpectedAccount => {
                msg!("Error: Deserialized account is not an SPL Token account")
            }
            CTokenError::InvalidToken => {
                msg!("Error: Invalid token account")
            }
            CTokenError::InvalidInput => {
                msg!("Error: Invalid input")
            }
            CTokenError::InvalidOwner => {
                msg!("Error: Invalid cToken owner")
            }
            CTokenError::InvalidConfig => {
                msg!("Error: Invalid config")
            }
            CTokenError::InvalidMint => {
                msg!("Error: Invalid mint")
            }
            CTokenError::InvalidAuthority => {
                msg!("Error: Invalid authority")
            }
            CTokenError::InvalidFeeCollector => {
                msg!("Error: Invalid fee collector")
            }
            CTokenError::InvalidAmount => {
                msg!("Error: Invalid amount")
            }
        }
    }
}
