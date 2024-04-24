//! Program state processor

use {
    crate::{
        processor::{
            message_protocol::{iotube::IoTubeProtocol, MessageParser},
            signature::ed25519::ed25519_verify,
        },
        state::{
            enums::GovernanceAddinAccountType,
            offchain_votes_record::{get_offchain_votes_record_address_seeds, OffchainVotesRecord},
            proposal::{
                get_min_vote_threshold_weight, get_proposal_data_for_governance_and_governing_mint,
                ProposalV2,
            },
            record_transaction::{get_record_transaction_address_seeds, RecordTransaction},
        },
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        sysvar::Sysvar,
    },
    spl_governance::{
        error::GovernanceError,
        state::{
            enums::{ProposalState, TransactionExecutionStatus, VoteThreshold, VoteTipping},
            governance::get_governance_data_for_realm,
            proposal::{OptionVoteResult, VoteType},
            proposal_transaction::get_proposal_transaction_data_for_proposal,
            realm::get_realm_data_for_governing_token_mint,
            token_owner_record::get_token_owner_record_data_for_realm_and_governing_mint,
            vote_record::{Vote, VoteKind},
        },
    },
    spl_governance_tools::account::create_and_serialize_account_signed,
};

/// Processes SubmitVotes instruction
pub fn process_submit_votes(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let instructions_sysvar_account = next_account_info(account_info_iter)?; // 0
    let realm_info = next_account_info(account_info_iter)?; // 1
    let vote_governing_token_mint_info = next_account_info(account_info_iter)?; // 2
    let governance_info = next_account_info(account_info_iter)?; // 3
    let proposal_info = next_account_info(account_info_iter)?; // 4
    let proposal_transaction_info = next_account_info(account_info_iter)?; // 5
    let offchain_votes_record_info = next_account_info(account_info_iter)?; // 6
    let record_transaction_info = next_account_info(account_info_iter)?; // 7
    let payer_info = next_account_info(account_info_iter)?; // 8
    let system_info = next_account_info(account_info_iter)?; // 9

    let clock = Clock::get()?;

    // Step0: Prevent resubmision of votes
    if !offchain_votes_record_info.data_is_empty() {
        return Err(GovernanceError::VoteAlreadyExists.into());
    }
    if !record_transaction_info.data_is_empty() {
        return Err(GovernanceError::TransactionAlreadyExists.into());
    }

    // Step1: Validate the signatures and extract validated msgs
    let raw_data = ed25519_verify(&instructions_sysvar_account)?;
    msg!("ed25519_verify: {:?}", raw_data);
    let msgs = raw_data
        .iter()
        .map(|data| &data.message)
        .collect::<Vec<_>>();
    let message_parser = IoTubeProtocol::new(&msgs);

    // Step2: Tally the votes
    let votes_auth = raw_data.iter().map(|data| data.pubkey).collect::<Vec<_>>();
    let votes = message_parser.votes()?;

    let mut proposal_data = get_proposal_data_for_governance_and_governing_mint(
        program_id,
        proposal_info,
        governance_info.key,
        &vote_governing_token_mint_info.key,
    )?;

    if proposal_data.state == ProposalState::Draft {
        proposal_data.state = ProposalState::Voting;
    }

    let (voter_weights, max_vote_weight, vote_threshold, vote_result) = tally_offchain_votes(
        program_id,
        realm_info,
        vote_governing_token_mint_info,
        governance_info,
        &proposal_data,
        account_info_iter.as_slice(), // 10
        &votes_auth,
        &votes,
    )?;

    if vote_result == OptionVoteResult::None {
        msg!("Insufficient votes from offchain");
        msg!("voter_weights: {:?}", voter_weights);
        msg!("max_vote_weight: {:?}", max_vote_weight);
        msg!("vote_threshold: {:?}", vote_threshold);
        msg!("vote_result: {:?}", vote_result);
        return Err(ProgramError::InvalidInstructionData);
    }

    // Step3: Generate record_id from msg (protocol related)
    let record_id = message_parser.record_id()?;

    // Step4: Create new vote record account and update proposal
    let offchain_votes_record_data = OffchainVotesRecord {
        account_type: GovernanceAddinAccountType::OffchainVotesRecord,
        record_id: record_id,
        proposal: *proposal_info.key,
        governing_token_owners: votes_auth,
        voter_weights: voter_weights,
        votes: votes,
        max_vote_weight,
        vote_threshold,
        vote_result: vote_result.clone(),
        vote_record_index: proposal_data.offchain_votes_record.vote_records_count,
        prev_vote_record_account: proposal_data.offchain_votes_record.last_vote_record_account,
        voting_completed_at: clock.unix_timestamp,
    };
    proposal_data.offchain_votes_record.vote_records_count = proposal_data
        .offchain_votes_record
        .vote_records_count
        .checked_add(1)
        .unwrap();
    proposal_data.offchain_votes_record.last_vote_record_account =
        Some(*offchain_votes_record_info.key);
    msg!(
        "offchain_votes_record_data: {:?}",
        offchain_votes_record_data
    );
    msg!("proposal_data: {:?}", proposal_data);
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
    proposal_data.serialize(&mut proposal_info.data.borrow_mut()[..])?;

    // Step5: If succeeded, Generate record transaction from proposal transaction and msg (protocol related)
    if vote_result != OptionVoteResult::Succeeded {
        msg!("Vote failed, vote_result: {:?}", vote_result);
        return Ok(());
    }
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
    msg!("record_transaction_data: {:?}", record_transaction_data);
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

fn tally_offchain_votes(
    program_id: &Pubkey,
    realm_info: &AccountInfo,
    vote_governing_token_mint_info: &AccountInfo,
    governance_info: &AccountInfo,
    proposal_data: &ProposalV2,
    voters_token_owner_record_infos: &[AccountInfo],
    votes_authorites: &Vec<Pubkey>,
    votes: &Vec<Vote>,
) -> Result<(Vec<u64>, u64, VoteThreshold, OptionVoteResult), ProgramError> {
    let realm_data = get_realm_data_for_governing_token_mint(
        program_id,
        realm_info,
        vote_governing_token_mint_info.key,
    )?;
    let governance_data =
        get_governance_data_for_realm(program_id, governance_info, realm_info.key)?;

    // For simplicity we are assuming no veto votes
    let vote_kind = VoteKind::Electorate;

    // assert proposal can be voted
    if proposal_data.state != ProposalState::Voting {
        return Err(GovernanceError::InvalidStateCannotVote.into());
    }

    votes
        .iter()
        .try_for_each(|vote| proposal_data.assert_valid_vote(vote))?;

    let voters_token_owner_records = voters_token_owner_record_infos
        .iter()
        .map(|token_owner_record_info| {
            get_token_owner_record_data_for_realm_and_governing_mint(
                program_id,
                token_owner_record_info,
                &governance_data.realm,
                vote_governing_token_mint_info.key,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    msg!(
        "voters_token_owner_records: {:?}",
        voters_token_owner_records
    );
    msg!("votes_authorites: {:?}", votes_authorites);

    if voters_token_owner_records.len() != votes_authorites.len()
        || !voters_token_owner_records
            .iter()
            .map(|record| &record.governing_token_owner)
            .eq(votes_authorites)
    {
        return Err(GovernanceError::GoverningTokenOwnerMustSign.into());
    }

    let votes_weights = voters_token_owner_records
        .iter()
        .map(|record| record.governing_token_deposit_amount)
        .collect::<Vec<_>>();

    // Calculate Proposal voting weights
    let mut yes_vote_weight = 0u64;
    let mut deny_vote_weight = 0u64;

    for (vote, weight) in votes.iter().zip(votes_weights.clone()) {
        match vote {
            Vote::Approve(choices) => {
                // Only support single choice voting for now
                if choices.len() != 1 {
                    return Err(GovernanceError::InvalidInstruction.into());
                }
                yes_vote_weight = yes_vote_weight
                    .checked_add(choices[0].get_choice_weight(weight)?)
                    .unwrap();
            }
            Vote::Deny => deny_vote_weight = deny_vote_weight.checked_add(weight).unwrap(),
            Vote::Abstain | Vote::Veto => {
                return Err(GovernanceError::NotSupportedVoteType.into());
            }
        }
    }

    // Tip proposal
    let max_voter_weight = proposal_data.resolve_max_voter_weight(
        &realm_data,
        vote_governing_token_mint_info,
        &vote_kind,
    )?;

    let vote_threshold = governance_data.resolve_vote_threshold(
        &realm_data,
        vote_governing_token_mint_info.key,
        &vote_kind,
    )?;

    let votes_result = tip_vote(
        &proposal_data,
        max_voter_weight,
        &vote_threshold,
        governance_data.get_vote_tipping(&realm_data, vote_governing_token_mint_info.key)?,
        yes_vote_weight,
        deny_vote_weight,
    )?;

    return Ok((
        votes_weights,
        max_voter_weight,
        vote_threshold,
        votes_result,
    ));
}

fn tip_vote(
    proposal: &ProposalV2,
    max_voter_weight: u64,
    vote_threshold: &VoteThreshold,
    vote_tipping: &VoteTipping,
    yes_vote_weight: u64,
    deny_vote_weight: u64,
) -> Result<OptionVoteResult, ProgramError> {
    let min_vote_threshold_weight =
        get_min_vote_threshold_weight(vote_threshold, max_voter_weight)?;

    if proposal.vote_type != VoteType::SingleChoice {
        return Err(GovernanceError::InvalidInstruction.into());
    }

    match vote_tipping {
        VoteTipping::Disabled => {}
        VoteTipping::Strict => {
            if yes_vote_weight >= min_vote_threshold_weight
                && yes_vote_weight > (max_voter_weight.saturating_sub(yes_vote_weight))
            {
                return Ok(OptionVoteResult::Succeeded);
            }
        }
        VoteTipping::Early => {
            if yes_vote_weight >= min_vote_threshold_weight && yes_vote_weight > deny_vote_weight {
                return Ok(OptionVoteResult::Succeeded);
            }
        }
    }

    // If vote tipping isn't disabled entirely, allow a vote to complete as
    // "defeated" if there is no possible way of reaching majority or the
    // min_vote_threshold_weight for another option. This tipping is always
    // strict, there's no equivalent to "early" tipping for deny votes.
    if *vote_tipping != VoteTipping::Disabled
        && (deny_vote_weight > (max_voter_weight.saturating_sub(min_vote_threshold_weight))
            || deny_vote_weight >= (max_voter_weight.saturating_sub(deny_vote_weight)))
    {
        return Ok(OptionVoteResult::Defeated);
    }

    Ok(OptionVoteResult::None)
}
