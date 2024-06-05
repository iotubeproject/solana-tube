use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the cToken program.
#[derive(Clone, Debug, PartialEq, Eq, Error, FromPrimitive)]
pub enum HelloError {
    /// Invalid program address
    #[error("Invalid program address generated from bump seed and key")]
    InvalidProgramAddress,
}
impl From<HelloError> for ProgramError {
    fn from(e: HelloError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for HelloError {
    fn type_of() -> &'static str {
        "HelloError"
    }
}

impl PrintProgramError for HelloError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            HelloError::InvalidProgramAddress => {
                msg!("Error: Invalid program address generated from bump seed and key")
            }
        }
    }
}
