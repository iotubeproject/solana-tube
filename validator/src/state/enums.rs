//! State enumerations

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Defines all Governance addin accounts types
#[derive(Clone, Debug, Default, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum GovernanceAddinAccountType {
    /// Default uninitialized account state
    #[default]
    Uninitialized,

    /// RecordTransaction account which holds instructions to execute for
    RecordTransaction,

    /// OffchainVotesRecord account which holds the offchain votes casted by the voters
    OffchainVotesRecord,
}
