//! Program instructions

use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        bpf_loader_upgradeable,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    spl_governance::{
        state::{
            enums::MintMaxVoterWeightSource,
            governance::{
                get_governance_address, get_mint_governance_address,
                get_program_governance_address, get_token_governance_address, GovernanceConfig,
            },
            native_treasury::get_native_treasury_address,
            program_metadata::get_program_metadata_address,
            proposal::{get_proposal_address, VoteType},
            proposal_deposit::get_proposal_deposit_address,
            proposal_transaction::{get_proposal_transaction_address, InstructionData},
            realm::{
                get_governing_token_holding_address, get_realm_address,
                GoverningTokenConfigAccountArgs, GoverningTokenConfigArgs, RealmConfigArgs,
                SetRealmAuthorityAction,
            },
            realm_config::get_realm_config_address,
            required_signatory::get_required_signatory_address,
            signatory_record::get_signatory_record_address,
            token_owner_record::get_token_owner_record_address,
            vote_record::{get_vote_record_address, Vote},
        },
        tools::bpf_loader_upgradeable::get_program_data_address,
    },
};

/// Instructions supported by the Governance program
#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
#[allow(clippy::large_enum_variant)]
pub enum GovernanceAddinInstruction {
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
    SubmitVotes {
        // #[allow(dead_code)]
        // /// UTF-8 encoded Governance Realm name
        // name: String,

        // #[allow(dead_code)]
        // /// Realm config args
        // config_args: RealmConfigArgs,
    },
}
