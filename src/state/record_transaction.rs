//! ProposalTransaction Account

use {
    crate::state::enums::AddinGovernanceAccountType,
    borsh::{maybestd::io::Write, BorshDeserialize, BorshSchema, BorshSerialize},
    core::panic,
    solana_program::{
        account_info::AccountInfo,
        clock::UnixTimestamp,
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        program_pack::IsInitialized,
        pubkey::Pubkey,
    },
    spl_governance::{
        error::GovernanceError,
        state::{
            enums::{GovernanceAccountType, TransactionExecutionStatus},
            legacy::ProposalInstructionV1,
            proposal_transaction::InstructionData,
        },
        PROGRAM_AUTHORITY_SEED,
    },
    spl_governance_tools::account::{get_account_data, get_account_type, AccountMaxSize},
};

/// Account for an instruction to be executed for Proposal
#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct RecordTransaction {
    /// Governance Account type
    pub account_type: AddinGovernanceAccountType,

    /// The Proposal the instruction belongs to
    pub proposal: Pubkey,

    /// The Offchain Votes Record the instruction belongs to
    pub offchain_votes_record: Pubkey,

    /// The Proposal Transaction the instruction created from
    pub proposal_transaction: Pubkey,

    /// Instructions to execute
    /// The instructions will be signed by Governance PDA the Proposal belongs
    /// to
    // For example for ProgramGovernance the instruction to upgrade program will be signed by
    // ProgramGovernance PDA All instructions will be executed within a single transaction
    pub instructions: Vec<InstructionData>,

    /// Executed at flag
    pub executed_at: Option<UnixTimestamp>,

    /// Instruction execution status
    pub execution_status: TransactionExecutionStatus,
}

impl AccountMaxSize for RecordTransaction {
    fn get_max_size(&self) -> Option<usize> {
        let instructions_size = self
            .instructions
            .iter()
            .map(|i| i.accounts.len() * 34 + i.data.len() + 40)
            .sum::<usize>();

        Some(instructions_size + 111)
    }
}

impl IsInitialized for RecordTransaction {
    fn is_initialized(&self) -> bool {
        self.account_type == AddinGovernanceAccountType::RecordTransaction
    }
}

impl RecordTransaction {
    /// Serializes account into the target buffer
    pub fn serialize<W: Write>(self, writer: W) -> Result<(), ProgramError> {
        borsh::to_writer(writer, &self)?;
        Ok(())
    }
}

/// Returns RecordTransaction PDA seeds
pub fn get_record_transaction_address_seeds<'a>(
    proposal: &'a Pubkey,
    offchain_votes_record: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [
        PROGRAM_AUTHORITY_SEED,
        proposal.as_ref(),
        offchain_votes_record.as_ref(),
    ]
}

/// Returns RecordTransaction PDA address
pub fn get_record_transaction_address<'a>(
    program_id: &Pubkey,
    proposal: &'a Pubkey,
    offchain_votes_record: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_record_transaction_address_seeds(proposal, offchain_votes_record),
        program_id,
    )
    .0
}

/// Deserializes ProposalTransaction account and checks owner program
pub fn get_record_transaction_data(
    program_id: &Pubkey,
    record_transaction_info: &AccountInfo,
) -> Result<RecordTransaction, ProgramError> {
    get_account_data::<RecordTransaction>(program_id, record_transaction_info)
}

///  Deserializes and returns ProposalTransaction account and checks it belongs
/// to the given Proposal
pub fn get_proposal_transaction_data_for_proposal(
    program_id: &Pubkey,
    record_transaction_info: &AccountInfo,
    proposal: &Pubkey,
    offchain_votes_record: &Pubkey,
) -> Result<RecordTransaction, ProgramError> {
    let record_transaction_data = get_record_transaction_data(program_id, record_transaction_info)?;

    if record_transaction_data.proposal != *proposal {
        return Err(GovernanceError::InvalidProposalForProposalTransaction.into());
    }

    if record_transaction_data.offchain_votes_record != *offchain_votes_record {
        return Err(GovernanceError::InvalidProposalForProposalTransaction.into());
    }

    Ok(record_transaction_data)
}

#[cfg(test)]
mod test {

    use {super::*, spl_governance::state::proposal_transaction::AccountMetaData};

    fn create_test_account_meta_data() -> AccountMetaData {
        AccountMetaData {
            pubkey: Pubkey::new_unique(),
            is_signer: true,
            is_writable: false,
        }
    }

    fn create_test_instruction_data() -> Vec<InstructionData> {
        vec![InstructionData {
            program_id: Pubkey::new_unique(),
            accounts: vec![
                create_test_account_meta_data(),
                create_test_account_meta_data(),
                create_test_account_meta_data(),
            ],
            data: vec![1, 2, 3],
        }]
    }

    fn create_test_record_transaction() -> RecordTransaction {
        RecordTransaction {
            account_type: AddinGovernanceAccountType::RecordTransaction,
            proposal: Pubkey::new_unique(),
            offchain_votes_record: Pubkey::new_unique(),
            proposal_transaction: Pubkey::new_unique(),
            instructions: create_test_instruction_data(),
            executed_at: Some(100),
            execution_status: TransactionExecutionStatus::Success,
        }
    }

    #[test]
    fn test_record_transaction_max_size() {
        // Arrange
        let record_transaction = create_test_record_transaction();
        let size = borsh::to_vec(&record_transaction).unwrap().len();

        // Act, Assert
        assert_eq!(record_transaction.get_max_size(), Some(size));
    }

    #[test]
    fn test_empty_record_transaction_max_size() {
        // Arrange
        let mut record_transaction = create_test_record_transaction();
        record_transaction.instructions[0].data = vec![];
        record_transaction.instructions[0].accounts = vec![];

        let size = borsh::to_vec(&record_transaction).unwrap().len();

        // Act, Assert
        assert_eq!(record_transaction.get_max_size(), Some(size));
    }
}
