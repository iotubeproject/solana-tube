pub mod dummy;
pub mod iotube;

use {
    solana_program::program_error::ProgramError,
    spl_governance::state::{proposal_transaction::InstructionData, vote_record::Vote},
};

pub trait MessageParser<'a> {
    fn new(message: &'a Vec<&Vec<u8>>) -> Self;

    fn votes(&self) -> Result<Vec<Vote>, ProgramError>;

    fn record_id(&self) -> Result<[u8; 32], ProgramError>;

    fn instructions_from_proposal(
        &self,
        proposal_instructions: &Vec<InstructionData>,
    ) -> Result<Vec<InstructionData>, ProgramError>;
}
