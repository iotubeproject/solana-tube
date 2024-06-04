pub mod dummy;
pub mod iotube;

use {
    solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey},
    spl_governance::state::{proposal_transaction::InstructionData, vote_record::Vote},
};

pub trait MessageParser<'a> {
    fn new(messages: &'a [u8], messages_hash: &'a Vec<&Vec<u8>>) -> Self;

    fn validate(&self, program_id: &Pubkey) -> Result<(), ProgramError>;

    fn votes(&self) -> Result<Vec<Vote>, ProgramError>;

    fn record_id(&self) -> Result<[u8; 32], ProgramError>;

    fn instructions_from_proposal(
        &self,
        proposal_instructions: &Vec<InstructionData>,
        accounts_info: &[AccountInfo],
    ) -> Result<Vec<InstructionData>, ProgramError>;
}
