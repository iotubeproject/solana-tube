use {
    crate::state::{
        offchain_votes_record::get_offchain_votes_record_data_for_proposal,
        proposal::get_proposal_data_for_governance,
        record_transaction::get_record_transaction_data_for_votes_record,
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        instruction::Instruction,
        program::invoke_signed,
        pubkey::Pubkey,
        sysvar::Sysvar,
    },
    spl_governance::{
        error::GovernanceError,
        state::{
            enums::{ProposalState, TransactionExecutionStatus},
            governance::get_governance_data,
        },
    },
};
pub fn process_execute_transaction(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let governance_info = next_account_info(account_info_iter)?; // 0
    let proposal_info = next_account_info(account_info_iter)?; // 1
    let vote_record_info = next_account_info(account_info_iter)?; // 2
    let record_transaction_info = next_account_info(account_info_iter)?; // 3
    let clock = Clock::get()?;
    let governance_data = get_governance_data(program_id, governance_info)?;
    let proposal_data =
        get_proposal_data_for_governance(program_id, proposal_info, governance_info.key)?;
    let offchain_votes_record_data = get_offchain_votes_record_data_for_proposal(
        program_id,
        vote_record_info,
        proposal_info.key,
    )?;
    let mut record_transaction_data = get_record_transaction_data_for_votes_record(
        program_id,
        record_transaction_info,
        vote_record_info.key,
    )?;
    if proposal_data.state != ProposalState::Voting {
        return Err(GovernanceError::InvalidStateCannotExecuteTransaction.into());
    }
    record_transaction_data.assert_can_execute_transaction(&offchain_votes_record_data)?;
    let instructions = record_transaction_data
        .instructions
        .iter()
        .map(Instruction::from);
    let instruction_account_infos = account_info_iter.as_slice();
    let mut signers_seeds: Vec<&[&[u8]]> = vec![];
    let mut governance_seeds = governance_data.get_governance_address_seeds()?.to_vec();
    let (_, bump_seed) = Pubkey::find_program_address(&governance_seeds, program_id);
    let bump = &[bump_seed];
    governance_seeds.push(bump);
    signers_seeds.push(&governance_seeds[..]);
    for instruction in instructions {
        invoke_signed(&instruction, instruction_account_infos, &signers_seeds[..])?;
    }
    record_transaction_data.executed_at = Some(clock.unix_timestamp);
    record_transaction_data.execution_status = TransactionExecutionStatus::Success;
    record_transaction_data.serialize(&mut record_transaction_info.data.borrow_mut()[..])?;
    Ok(())
}
