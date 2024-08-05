use ctoken::instruction;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub struct Processor {}

impl Processor {
    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], _input: &[u8]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let c_token_program_info = next_account_info(account_info_iter)?;
        let c_token_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let c_token_token_info = next_account_info(account_info_iter)?;
        let user_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;
        let token_mint_info = next_account_info(account_info_iter)?;
        let config_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        let (owner, bump_seed) = Pubkey::find_program_address(&[b"ctoken"], program_id);
        if *owner_info.key != owner {
            return Err(ProgramError::InvalidAccountOwner);
        }

        let tx = instruction::settle(
            &c_token_program_info.key,
            &c_token_info.key,
            &authority_info.key,
            &c_token_token_info.key,
            &user_info.key,
            &owner_info.key,
            &token_mint_info.key,
            &config_info.key,
            &token_program_info.key,
            20000000,
        )?;

        invoke_signed(
            &tx,
            &[
                c_token_info.clone(),
                authority_info.clone(),
                c_token_token_info.clone(),
                user_info.clone(),
                owner_info.clone(),
                token_mint_info.clone(),
                config_info.clone(),
                token_program_info.clone(),
            ],
            &[&[b"ctoken", &[bump_seed]]],
        )?;

        Ok(())
    }
}
