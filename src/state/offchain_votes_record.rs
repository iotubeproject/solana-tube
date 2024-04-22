//! Proposal Vote Record Account

use {
    borsh::{maybestd::io::Write, BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        account_info::AccountInfo, clock::UnixTimestamp, program_error::ProgramError,
        program_pack::IsInitialized, pubkey::Pubkey,
    },
    spl_governance::{
        state::{
            enums::{GovernanceAccountType, VoteThreshold},
            proposal::OptionVoteResult,
            vote_record::Vote,
        },
        PROGRAM_AUTHORITY_SEED,
    },
    spl_governance_tools::account::{get_account_data, AccountMaxSize},
};

pub const HASH_BYTES: usize = 32;

/// Proposal VoteRecord
#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct OffchainVotesRecord {
    /// Governance account type
    pub account_type: GovernanceAccountType,

    /// Record ID
    pub record_id: [u8; HASH_BYTES],

    /// Proposal account
    pub proposal: Pubkey,

    /// The users who casted this votes
    /// This is the Governing Token Owner who deposited governing tokens into
    /// the Realm
    pub governing_token_owners: Vec<Pubkey>,

    /// The weights of the users casting the vote
    pub voter_weights: Vec<u64>,

    /// Voters' votes
    pub votes: Vec<Vote>,

    /// The max vote weight for the Governing Token mint at the time Proposal
    /// was decided.
    /// It's used to show correct vote results for historical proposals in
    /// cases when the mint supply or max weight source changed after vote was
    /// completed.
    pub max_vote_weight: u64,

    /// The vote threshold at the time Proposal was decided
    /// It's used to show correct vote results for historical proposals in cases
    /// when the threshold was changed for governance config after vote was
    /// completed.
    /// TODO: Use this field to override the threshold from parent Governance
    /// (only higher value possible)
    pub vote_threshold: VoteThreshold,

    /// Vote result for the option
    pub vote_result: OptionVoteResult,

    /// Unique record index within it's parent Proposal
    pub vote_record_index: u64,

    /// Previous vote record account
    pub prev_vote_record_account: Option<Pubkey>,

    /// Voting completed at flag
    pub voting_completed_at: UnixTimestamp,
}

impl AccountMaxSize for OffchainVotesRecord {
    fn get_max_size(&self) -> Option<usize> {
        Some(
            1 + 32
                + 32
                + 4
                + self.governing_token_owners.len() * 32
                + 4
                + self.voter_weights.len() * 8
                + 4
                + self.votes.len() * 24
                + 8
                + 2
                + 1
                + 8
                + 32
                + 8,
        )
    }
}

impl IsInitialized for OffchainVotesRecord {
    fn is_initialized(&self) -> bool {
        self.account_type == GovernanceAccountType::VoteRecordV2
    }
}
impl OffchainVotesRecord {
    /// Serializes account into the target buffer
    pub fn serialize<W: Write>(self, writer: W) -> Result<(), ProgramError> {
        borsh::to_writer(writer, &self)?;
        Ok(())
    }
}

// Also use this func to confirm if the account is initialized
/// Deserializes VoteRecord account and checks owner program
pub fn get_offchain_votes_record_data(
    program_id: &Pubkey,
    offchain_votes_record_info: &AccountInfo,
) -> Result<OffchainVotesRecord, ProgramError> {
    get_account_data::<OffchainVotesRecord>(program_id, offchain_votes_record_info)
}

pub fn get_offchain_votes_record_data_for_proposal(
    program_id: &Pubkey,
    offchain_votes_record_info: &AccountInfo,
    proposal: &Pubkey,
) -> Result<OffchainVotesRecord, ProgramError> {
    let offchain_votes_record_data =
        get_offchain_votes_record_data(program_id, offchain_votes_record_info)?;
    if offchain_votes_record_data.proposal != *proposal {
        return Err(ProgramError::InvalidArgument);
    }
    Ok(offchain_votes_record_data)
}

/// Returns VoteRecord PDA seeds
pub fn get_offchain_votes_record_address_seeds<'a>(
    proposal: &'a Pubkey,
    record_id: &'a [u8; HASH_BYTES],
) -> [&'a [u8]; 3] {
    [
        PROGRAM_AUTHORITY_SEED,
        proposal.as_ref(),
        record_id.as_ref(),
    ]
}

/// Returns VoteRecord PDA address
pub fn get_offchain_votes_record_address<'a>(
    program_id: &Pubkey,
    proposal: &'a Pubkey,
    record_id: &'a [u8; HASH_BYTES],
) -> Pubkey {
    Pubkey::find_program_address(
        &get_offchain_votes_record_address_seeds(proposal, record_id),
        program_id,
    )
    .0
}
