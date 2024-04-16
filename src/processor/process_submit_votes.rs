//! Program state processor

use {
    super::signature::secp256k1::{secp256k1_verify, Data},
    crate::{
        processor::message_protocol::{dummy::DummyProtocol, MessageParser},
        state::{
            enums::GovernanceAddinAccountType,
            offchain_votes_record::{
                get_offchain_votes_record_address, get_offchain_votes_record_address_seeds,
                OffchainVotesRecord,
            },
            proposal::{get_proposal_data_for_governance, OptionVoteResult},
            record_transaction::{get_record_transaction_address_seeds, RecordTransaction},
        },
    },
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
        rent::Rent,
        sysvar::Sysvar,
    },
    spl_governance::{
        error::GovernanceError,
        state::{
            enums::{GovernanceAccountType, ProposalState, TransactionExecutionStatus},
            governance::get_governance_data,
            native_treasury::get_native_treasury_address_seeds,
            proposal_transaction::get_proposal_transaction_data_for_proposal,
        },
    },
    spl_governance_tools::account::create_and_serialize_account_signed,
    spl_token::{
        instruction::TokenInstruction,
        state::{Account, Mint},
        ID as TOKEN_PROGRAM_ID,
    },
};

/// Processes SubmitVotes instruction
pub fn process_submit_votes(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let instructions_sysvar_account = next_account_info(account_info_iter)?; // -1
    let governance_info = next_account_info(account_info_iter)?; // 0
    let proposal_info = next_account_info(account_info_iter)?; // 1
    let proposal_transaction_info = next_account_info(account_info_iter)?; // 2

    let offchain_votes_record_info = next_account_info(account_info_iter)?; // 3
    let record_transaction_info = next_account_info(account_info_iter)?; // 3

    let payer_info = next_account_info(account_info_iter)?; // 4
    let system_info = next_account_info(account_info_iter)?; // 5

    let clock = Clock::get()?;

    // Step0: Prevent resubmision of votes
    if !offchain_votes_record_info.data_is_empty() {
        return Err(GovernanceError::VoteAlreadyExists.into());
    }
    if !record_transaction_info.data_is_empty() {
        return Err(GovernanceError::TransactionAlreadyExists.into());
    }
    let mut proposal_data =
        get_proposal_data_for_governance(program_id, proposal_info, governance_info.key)?;
    proposal_data
        .assert_is_final_state()
        .map_err(|_| GovernanceError::ProposalIsNotExecutable)?;

    // Step1: Validate the signatures and extract validated msgs
    let msgs = secp256k1_verify(&instructions_sysvar_account)?;
    msg!("secp256k1_verify: {:?}", msgs);

    // Step2: Tally the votes(see impl in cast_votes)
    // check known pubkey and its msg data

    // Step3: Generate record_id from msg (protocol related)
    let message_parser = DummyProtocol::new(&msgs[0].message);
    let record_id = message_parser.record_id()?;
    msg!("record_id: {:?}", record_id);

    // Step4: Create new vote record account and update proposal
    let offchain_votes_record_data = OffchainVotesRecord {
        account_type: GovernanceAccountType::VoteRecordV2,
        record_id: record_id,
        proposal: *proposal_info.key,
        governing_token_owners: vec![],
        voter_weights: vec![],
        votes: vec![],
        vote_result: OptionVoteResult::Succeeded,
        // TODO: fix this
        vote_record_index: 0,
        // TODO: fix this
        prev_vote_record_account: *offchain_votes_record_info.key,
        recorded_at: clock.unix_timestamp,
    };
    create_and_serialize_account_signed::<OffchainVotesRecord>(
        payer_info,
        offchain_votes_record_info,
        &offchain_votes_record_data,
        &get_offchain_votes_record_address_seeds(proposal_info.key, &record_id),
        program_id,
        system_info,
        &Rent::get()?,
        0,
    )?;
    // TODO: update proposal and its state

    // Step5: Generate record transaction from proposal transaction and msg (protocol related)
    let proposal_transaction_data = get_proposal_transaction_data_for_proposal(
        program_id,
        proposal_transaction_info,
        proposal_info.key,
    )?;

    let record_instruction =
        message_parser.instructions_from_proposal(&proposal_transaction_data.instructions)?;

    let record_transaction_data = RecordTransaction {
        account_type: GovernanceAddinAccountType::RecordTransaction,
        proposal: *proposal_info.key,
        offchain_votes_record: *offchain_votes_record_info.key,
        proposal_transaction: *proposal_transaction_info.key,
        instructions: record_instruction,
        executed_at: None,
        execution_status: TransactionExecutionStatus::None,
    };
    create_and_serialize_account_signed::<RecordTransaction>(
        payer_info,
        record_transaction_info,
        &record_transaction_data,
        &get_record_transaction_address_seeds(proposal_info.key, &offchain_votes_record_info.key),
        program_id,
        system_info,
        &Rent::get()?,
        0,
    )?;

    Ok(())
}
