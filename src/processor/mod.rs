//! Program processor

mod process_create_governance;
mod process_create_proposal;
mod process_create_realm;
mod process_execute_transaction;
mod signature;

use {
    borsh::BorshDeserialize,
    process_create_governance::*,
    process_create_proposal::*,
    process_create_realm::*,
    process_execute_transaction::*,
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
        pubkey::Pubkey,
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

    let instruction = GovernanceInstruction::try_from_slice(input).map_err(|_| {
        msg!("Failed to deserialize instruction data{:?}", input);
        ProgramError::InvalidInstructionData
    })?;

    msg!("GOVERNANCE-INSTRUCTION: {:?}", instruction);

    match instruction {
        GovernanceInstruction::CreateRealm { name, config_args } => {
            process_create_realm(program_id, accounts, name, config_args)
        }

        GovernanceInstruction::CreateGovernance { config } => {
            process_create_governance(program_id, accounts, config)
        }

        GovernanceInstruction::CreateProposal {
            name,
            description_link,
            vote_type: proposal_type,
            options,
            use_deny_option,
            proposal_seed,
        } => process_create_proposal(
            program_id,
            accounts,
            name,
            description_link,
            proposal_type,
            options,
            use_deny_option,
            proposal_seed,
        ),

        GovernanceInstruction::ExecuteTransaction {} => {
            process_execute_transaction(program_id, accounts)
        }

        _ => {
            msg!("Instruction not available");
            Err(GovernanceError::InvalidInstruction.into())
        }
    }
}
