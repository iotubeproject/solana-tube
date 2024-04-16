pub mod dummy;
pub mod iotube;

use {
    solana_program::program_error::ProgramError,
    spl_governance::state::proposal_transaction::InstructionData,
};

pub trait MessageParser {
    fn new(message: &Vec<u8>) -> Self;

    fn record_id(&self) -> Result<[u8; 32], ProgramError>;

    fn instructions_from_proposal(
        &self,
        proposal_instructions: &Vec<InstructionData>,
    ) -> Result<Vec<InstructionData>, ProgramError>;
}
