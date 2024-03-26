//! Program processor

mod process_create_mint_governance;
mod process_execute_transaction;

use {
    process_execute_transaction::*,
    solana_program::{
        account_info::AccountInfo, borsh0_10::try_from_slice_unchecked, entrypoint::ProgramResult,
        msg, program_error::ProgramError, pubkey::Pubkey,
    },
    spl_governance::{error::GovernanceError, instruction::GovernanceInstruction},
};

/// Processes an instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    msg!("VERSION:{:?}", env!("CARGO_PKG_VERSION"));

    // Use try_from_slice_unchecked to support forward compatibility of newer UI
    // with older program
    let instruction: GovernanceInstruction =
        try_from_slice_unchecked(input).map_err(|_| ProgramError::InvalidInstructionData)?;

    msg!("GOVERNANCE-INSTRUCTION: {:?}", instruction);

    match instruction {
        GovernanceInstruction::ExecuteTransaction {} => {
            process_execute_transaction(program_id, accounts)
        }
        _ => {
            msg!("Instruction not available");
            Err(GovernanceError::InvalidInstruction.into())
        }
    }
}
