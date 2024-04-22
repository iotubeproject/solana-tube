//! Program instructions

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Instructions supported by the Governance program
#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum GovernanceAddinInstruction {
    // TODO update doc
    /// Creates Governance Realm account which aggregates governances for given
    /// Community Mint and optional Council Mint
    ///
    /// 0. `[writable]` Governance Realm account.
    ///     * PDA seeds:['governance',name]
    /// 1. `[]` Realm authority
    /// 2. `[]` Community Token Mint
    /// 3. `[writable]` Community Token Holding account.
    ///     * PDA seeds: ['governance',realm,community_mint]
    ///     The account will be created with the Realm PDA as its owner
    /// 4. `[signer]` Payer
    /// 5. `[]` System
    /// 6. `[]` SPL Token
    /// 7. `[]` Sysvar Rent
    /// 8. `[]` Council Token Mint - optional
    /// 9. `[writable]` Council Token Holding account - optional unless council
    ///    is used.
    ///     * PDA seeds: ['governance',realm,council_mint]
    ///     The account will be created with the Realm PDA as its owner
    /// 10. `[writable]` RealmConfig account.
    ///     * PDA seeds: ['realm-config', realm]
    /// 11. `[]` Optional Community Voter Weight Addin Program Id
    /// 12. `[]` Optional Max Community Voter Weight Addin Program Id
    /// 13. `[]` Optional Council Voter Weight Addin Program Id
    /// 14. `[]` Optional Max Council Voter Weight Addin Program Id
    SubmitVotes {},
}
