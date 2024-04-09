//! Program processor

mod process_create_governance;
mod process_create_proposal;
mod process_create_realm;
mod process_deposit_governing_tokens;
mod process_execute_transaction;
mod process_insert_transaction;
mod signature;

use {
    borsh::BorshDeserialize,
    process_create_governance::*,
    process_create_proposal::*,
    process_create_realm::*,
    process_deposit_governing_tokens::*,
    process_execute_transaction::*,
    process_insert_transaction::*,
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

        GovernanceInstruction::DepositGoverningTokens { amount } => {
            process_deposit_governing_tokens(program_id, accounts, amount)
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

        GovernanceInstruction::InsertTransaction {
            option_index,
            index,
            hold_up_time: _,
            instructions,
        } => process_insert_transaction(program_id, accounts, option_index, index, instructions),

        GovernanceInstruction::ExecuteTransaction {} => {
            process_execute_transaction(program_id, accounts)
        }

        _ => {
            msg!("Instruction not available");
            Err(GovernanceError::InvalidInstruction.into())
        }
    }
}
