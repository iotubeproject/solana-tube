//! Program instructions

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Instructions supported by the Governance program
#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum GovernanceAddinInstruction {
    ///  Submit offchain voting record to cast votes on a Proposal
    ///  By doing so you indicate you approve or disapprove of running
    ///  the Proposal set of transactions  If you tip the consensus
    ///  then the transactions can begin to be run  
    ///
    ///   0. `[]` SYSVAR account
    ///   1. `[]` Realm account
    ///   2. `[]` The Governing Token Mint which is used to cast the vote
    ///      (vote_governing_token_mint).
    ///   3. `[]` Governance account
    ///   4. `[writable]` Proposal account
    ///   5. `[]` ProposalTransaction account
    ///   6. `[writable]` Offchain VoteRecord account
    ///   7. `[writable]` Record Transaction account
    ///   8. `[signer]` Payer
    ///   9. `[]` System program
    ///   10+ Any extra accounts that are part of TokenOwnerRecord of the voter, in order
    SubmitVotes {},
}
