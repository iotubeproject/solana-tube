//! State enumerations

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Defines all Governance accounts types
#[derive(Clone, Debug, Default, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum GovernanceAddinAccountType {
    /// Default uninitialized account state
    #[default]
    Uninitialized,

    /// RecordTransaction account which holds instructions to execute for
    RecordTransaction,
}
