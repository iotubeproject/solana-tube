//! Program state processor

use {
    super::signature::secp256k1::{secp256k1_verify, Data},
    crate::state::proposal::{get_proposal_data_for_governance, OptionVoteResult},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        instruction::Instruction,
        msg,
        program::invoke_signed,
        program_error::ProgramError,
        program_pack::*,
        pubkey::Pubkey,
        sysvar::Sysvar,
    },
    spl_governance::state::{
        enums::{ProposalState, TransactionExecutionStatus},
        governance::get_governance_data,
        native_treasury::get_native_treasury_address_seeds,
        proposal_transaction::get_proposal_transaction_data_for_proposal,
    },
    spl_token::{
        state::{Account, Mint},
        ID as TOKEN_PROGRAM_ID,
    },
};

/// Processes ExecuteTransaction instruction
pub fn process_execute_transaction(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let instructions_sysvar_account = next_account_info(account_info_iter)?; // -1
    let governance_info = next_account_info(account_info_iter)?; // 0
    let proposal_info = next_account_info(account_info_iter)?; // 1
    let proposal_transaction_info = next_account_info(account_info_iter)?; // 2

    let clock = Clock::get()?;

    let governance_data = get_governance_data(program_id, governance_info)?;

    // TODO: this is a temp verification
    let msgs = secp256k1_verify(&instructions_sysvar_account)?;
    msg!("secp256k1_verify: {:?}", msgs);

    let mut proposal_data =
        get_proposal_data_for_governance(program_id, proposal_info, governance_info.key)?;

    let mut proposal_transaction_data = get_proposal_transaction_data_for_proposal(
        program_id,
        proposal_transaction_info,
        proposal_info.key,
    )?;

    // Proposal is always executed in the POC
    // TODO: Enable Proposal after POC

    // proposal_data
    //     .assert_can_execute_transaction(&proposal_transaction_data, clock.unix_timestamp)?;

    // Execute instruction with Governance PDA as signer
    let instructions = proposal_transaction_data
        .instructions
        .iter()
        .map(Instruction::from);

    // In the current implementation accounts for all instructions are passed to
    // each instruction invocation. This is an overhead but shouldn't be a
    // showstopper because if we can invoke the parent instruction with that many
    // accounts then we should also be able to invoke all the nested ones
    // TODO: Optimize the invocation to split the provided accounts for each
    // individual instruction
    let instruction_account_infos = account_info_iter.as_slice();

    let mut signers_seeds: Vec<&[&[u8]]> = vec![];

    // Sign the transaction using the governance PDA
    let mut governance_seeds = governance_data.get_governance_address_seeds()?.to_vec();
    let (_, bump_seed) = Pubkey::find_program_address(&governance_seeds, program_id);
    let bump = &[bump_seed];
    governance_seeds.push(bump);

    signers_seeds.push(&governance_seeds[..]);

    // TODO: Enable Proposal after POC
    // proposal_data.executing_at = Some(clock.unix_timestamp);
    // proposal_data.state = ProposalState::Executing;

    for instruction in instructions {
        invoke_signed(&instruction, instruction_account_infos, &signers_seeds[..])?;
    }

    // Update proposal and instruction accounts

    // TODO: Enable Proposal after POC
    // let option = &mut proposal_data.options[proposal_transaction_data.option_index as usize];
    // option.transactions_executed_count = option.transactions_executed_count.checked_add(1).unwrap();
    // proposal_data.closed_at = Some(clock.unix_timestamp);
    // proposal_data.state = ProposalState::Completed;
    // proposal_data.serialize(&mut proposal_info.data.borrow_mut()[..])?;

    proposal_transaction_data.executed_at = Some(clock.unix_timestamp);
    proposal_transaction_data.execution_status = TransactionExecutionStatus::Success;
    proposal_transaction_data.serialize(&mut proposal_transaction_info.data.borrow_mut()[..])?;

    Ok(())
}
