//! Proposal Vote Record Account

use {
    crate::state::proposal::OptionVoteResult,
    borsh::{maybestd::io::Write, BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        account_info::AccountInfo, clock::UnixTimestamp, program_error::ProgramError,
        program_pack::IsInitialized, pubkey::Pubkey,
    },
    spl_governance::{
        error::GovernanceError,
        state::{
            enums::GovernanceAccountType, proposal::ProposalV2, realm::RealmV2,
            token_owner_record::TokenOwnerRecordV2, vote_record::Vote,
        },
        PROGRAM_AUTHORITY_SEED,
    },
    spl_governance_tools::account::{get_account_data, AccountMaxSize},
};

/// Proposal VoteRecord
#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct OffchainVotesRecord {
    /// Governance account type
    pub account_type: GovernanceAccountType,

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

    /// Vote result for the option
    pub vote_result: OptionVoteResult,

    /// Unique record index within it's parent Proposal
    pub vote_record_index: u64,

    /// Previous vote record account
    pub prev_vote_record_account: Pubkey,

    /// Recorded at flag
    pub recorded_at: UnixTimestamp,
}

impl AccountMaxSize for OffchainVotesRecord {
    fn get_max_size(&self) -> Option<usize> {
        Some(
            1 + 32
                + 4
                + self.governing_token_owners.len() * 32
                + 4
                + self.voter_weights.len() * 8
                + 4
                + self.votes.len() * 24
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

/// Deserializes VoteRecord account and checks owner program
pub fn get_offchain_votes_record_data(
    program_id: &Pubkey,
    offchain_votes_record_info: &AccountInfo,
) -> Result<OffchainVotesRecord, ProgramError> {
    get_account_data::<OffchainVotesRecord>(program_id, offchain_votes_record_info)
}

/// Deserializes VoteRecord and checks it belongs to the provided Proposal and
/// TokenOwnerRecord
pub fn get_vote_record_data_for_proposal_and_token_owner_record(
    program_id: &Pubkey,
    vote_record_info: &AccountInfo,
    realm_data: &RealmV2,
    proposal: &Pubkey,
    proposal_data: &ProposalV2,
    token_owner_record_data: &TokenOwnerRecordV2,
) -> Result<VoteRecordV2, ProgramError> {
    let vote_record_data = get_vote_record_data(program_id, vote_record_info)?;

    if vote_record_data.proposal != *proposal {
        return Err(GovernanceError::InvalidProposalForVoterRecord.into());
    }

    if vote_record_data.governing_token_owner != token_owner_record_data.governing_token_owner {
        return Err(GovernanceError::InvalidGoverningTokenOwnerForVoteRecord.into());
    }

    // Assert governing_token_mint between Proposal and TokenOwnerRecord match for
    // the deserialized VoteRecord For Approve, Deny and Abstain votes
    // Proposal.governing_token_mint must equal
    // TokenOwnerRecord.governing_token_mint For Veto vote it must be the
    // governing_token_mint of the opposite voting population
    let proposal_governing_token_mint = realm_data.get_proposal_governing_token_mint_for_vote(
        &token_owner_record_data.governing_token_mint,
        &get_vote_kind(&vote_record_data.vote),
    )?;

    if proposal_data.governing_token_mint != proposal_governing_token_mint {
        return Err(GovernanceError::InvalidGoverningMintForProposal.into());
    }

    Ok(vote_record_data)
}

/// Returns VoteRecord PDA seeds
pub fn get_vote_record_address_seeds<'a>(
    proposal: &'a Pubkey,
    token_owner_record: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [
        PROGRAM_AUTHORITY_SEED,
        proposal.as_ref(),
        token_owner_record.as_ref(),
    ]
}

/// Returns VoteRecord PDA address
pub fn get_vote_record_address<'a>(
    program_id: &Pubkey,
    proposal: &'a Pubkey,
    token_owner_record: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_vote_record_address_seeds(proposal, token_owner_record),
        program_id,
    )
    .0
}

#[cfg(test)]
mod test {

    use {super::*, solana_program::clock::Epoch};

    #[test]
    fn test_vote_record_v1_to_v2_serialisation_roundtrip() {
        // Arrange

        let vote_record_v1_source = VoteRecordV1 {
            account_type: GovernanceAccountType::VoteRecordV1,
            proposal: Pubkey::new_unique(),
            governing_token_owner: Pubkey::new_unique(),
            is_relinquished: true,
            vote_weight: VoteWeightV1::Yes(120),
        };

        let mut account_data = vec![];
        borsh::to_writer(&mut account_data, &vote_record_v1_source).unwrap();

        let program_id = Pubkey::new_unique();

        let info_key = Pubkey::new_unique();
        let mut lamports = 10u64;

        let account_info = AccountInfo::new(
            &info_key,
            false,
            false,
            &mut lamports,
            &mut account_data[..],
            &program_id,
            false,
            Epoch::default(),
        );

        // Act

        let vote_record_v2 = get_vote_record_data(&program_id, &account_info).unwrap();
        vote_record_v2
            .serialize(&mut account_info.data.borrow_mut()[..])
            .unwrap();

        // Assert

        let vote_record_v1_target =
            get_account_data::<VoteRecordV1>(&program_id, &account_info).unwrap();

        assert_eq!(vote_record_v1_source, vote_record_v1_target)
    }

    #[test]
    fn test_get_choice_weight_with_invalid_weight_percentage_error() {
        // Arrange
        let vote_choice = VoteChoice {
            rank: 0,
            weight_percentage: 127,
        };

        // Act
        let result = vote_choice.get_choice_weight(100);

        // Assert
        assert_eq!(
            Err(GovernanceError::InvalidVoteChoiceWeightPercentage.into()),
            result
        );
    }

    #[test]
    fn test_get_choice_weight() {
        // Arrange
        let vote_choice = VoteChoice {
            rank: 0,
            weight_percentage: 100,
        };

        // Act
        let result = vote_choice.get_choice_weight(100);

        // Assert
        assert_eq!(Ok(100_u64), result);

        // Arrange
        let vote_choice = VoteChoice {
            rank: 0,
            weight_percentage: 0,
        };

        // Act
        let result = vote_choice.get_choice_weight(100);

        // Assert
        assert_eq!(Ok(0_u64), result);

        // Arrange
        let vote_choice = VoteChoice {
            rank: 0,
            weight_percentage: 33,
        };

        // Act
        let result = vote_choice.get_choice_weight(100);

        // Assert
        assert_eq!(Ok(33_u64), result);

        // Arrange
        let vote_choice = VoteChoice {
            rank: 0,
            weight_percentage: 34,
        };

        // Act
        let result = vote_choice.get_choice_weight(100);

        // Assert
        assert_eq!(Ok(34_u64), result);

        // Arrange
        let vote_choice = VoteChoice {
            rank: 0,
            weight_percentage: 50,
        };

        // Act
        let result = vote_choice.get_choice_weight(u64::MAX);

        // Assert
        assert_eq!(Ok(u64::MAX / 2), result);
    }
}
