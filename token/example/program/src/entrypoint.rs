use crate::{error::HelloError, processor::Processor};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::PrintProgramError,
    pubkey::Pubkey,
};

solana_program::entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<HelloError>();
        return Err(error);
    }
    Ok(())
}
