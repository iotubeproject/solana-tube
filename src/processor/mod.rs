//! Program processor

mod message_protocol;
mod process_create_governance;
mod process_create_proposal;
mod process_create_realm;
mod process_deposit_governing_tokens;
mod process_execute_transaction;
mod process_insert_transaction;
mod process_submit_votes;
mod signature;

use {
    crate::instruction::GovernanceAddinInstruction,
    borsh::BorshDeserialize,
    process_create_governance::*,
    process_create_proposal::*,
    process_create_realm::*,
    process_deposit_governing_tokens::*,
    process_execute_transaction::*,
    process_insert_transaction::*,
    process_submit_votes::*,
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
    let instruction = match GovernanceInstruction::try_from_slice(input) {
        Ok(ins) => ins,
        Err(_) => {
            if let Ok(ins_addin) = GovernanceAddinInstruction::try_from_slice(input) {
                match ins_addin {
                    GovernanceAddinInstruction::SubmitVotes {} => {
                        return process_submit_votes(program_id, accounts)
                    }
                }
            }
            msg!("Failed to deserialize instruction data{:?}", input);
            return Err(ProgramError::InvalidInstructionData);
        }
    };

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
