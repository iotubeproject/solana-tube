use std::error::Error;

use borsh::{BorshDeserialize, BorshSerialize};
use num_traits::FromPrimitive;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    decode_error::DecodeError,
    entrypoint::ProgramResult,
    instruction::Instruction,
    msg,
    program::{invoke, invoke_signed},
    program_error::{PrintProgramError, ProgramError},
    program_option::COption,
    pubkey::Pubkey,
    system_instruction,
};
use spl_token_2022::{
    check_spl_token_program_account,
    error::TokenError,
    extension::StateWithExtensions,
    state::{Account, Mint},
};

use crate::{
    error::CTokenError,
    instruction::CTokenInstruction,
    log,
    state::{CToken, Config},
};

pub struct Processor {}

impl Processor {
    /// Unpacks a spl_token `Account`.
    pub fn unpack_token_account(
        account_info: &AccountInfo,
        token_program_id: &Pubkey,
    ) -> Result<Account, CTokenError> {
        if account_info.owner != token_program_id
            && check_spl_token_program_account(account_info.owner).is_err()
        {
            Err(CTokenError::IncorrectTokenProgramId)
        } else {
            StateWithExtensions::<Account>::unpack(&account_info.data.borrow())
                .map(|a| a.base)
                .map_err(|_| CTokenError::ExpectedAccount)
        }
    }

    /// Unpacks a spl_token `Mint`.
    pub fn unpack_mint(
        account_info: &AccountInfo,
        token_program_id: &Pubkey,
    ) -> Result<Mint, CTokenError> {
        if account_info.owner != token_program_id
            && check_spl_token_program_account(account_info.owner).is_err()
        {
            Err(CTokenError::IncorrectTokenProgramId)
        } else {
            StateWithExtensions::<Mint>::unpack(&account_info.data.borrow())
                .map(|m| m.base)
                .map_err(|_| CTokenError::ExpectedMint)
        }
    }

    /// transfer token
    pub fn token_transfer<'a>(
        c_token: &Pubkey,
        token_program: AccountInfo<'a>,
        source: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        destination: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        bump_seed: u8,
        amount: u64,
        decimals: u8,
    ) -> Result<(), ProgramError> {
        let c_token_bytes = c_token.to_bytes();
        let authority_signature_seeds = [&c_token_bytes[..32], &[bump_seed]];
        let signers = &[&authority_signature_seeds[..]];
        let ix = spl_token_2022::instruction::transfer_checked(
            token_program.key,
            source.key,
            mint.key,
            destination.key,
            authority.key,
            &[],
            amount,
            decimals,
        )?;
        invoke_signed_wrapper::<TokenError>(
            &ix,
            &[source, mint, destination, authority, token_program],
            signers,
        )
    }

    /// Issue a spl_token `Burn` instruction.
    pub fn token_burn<'a>(
        c_token: &Pubkey,
        token_program: AccountInfo<'a>,
        burn_account: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        bump_seed: u8,
        amount: u64,
    ) -> Result<(), ProgramError> {
        let c_token_bytes = c_token.to_bytes();
        let authority_signature_seeds = [&c_token_bytes[..32], &[bump_seed]];
        let signers = &[&authority_signature_seeds[..]];

        let ix = spl_token_2022::instruction::burn(
            token_program.key,
            burn_account.key,
            mint.key,
            authority.key,
            &[],
            amount,
        )?;

        invoke_signed_wrapper::<TokenError>(
            &ix,
            &[burn_account, mint, authority, token_program],
            signers,
        )
    }

    /// Issue a spl_token `MintTo` instruction.
    pub fn token_mint_to<'a>(
        c_token: &Pubkey,
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        destination: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        bump_seed: u8,
        amount: u64,
    ) -> Result<(), ProgramError> {
        let c_token_bytes = c_token.to_bytes();
        let authority_signature_seeds = [&c_token_bytes[..32], &[bump_seed]];
        let signers = &[&authority_signature_seeds[..]];
        let ix = spl_token_2022::instruction::mint_to(
            token_program.key,
            mint.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?;

        invoke_signed_wrapper::<TokenError>(
            &ix,
            &[mint, destination, authority, token_program],
            signers,
        )
    }

    pub fn authority_id(
        program_id: &Pubkey,
        my_info: &Pubkey,
        bump_seed: u8,
    ) -> Result<Pubkey, CTokenError> {
        Pubkey::create_program_address(&[&my_info.to_bytes()[..32], &[bump_seed]], program_id)
            .or(Err(CTokenError::InvalidProgramAddress))
    }

    pub fn process_initial_config(accounts: &[AccountInfo], fee: u64) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let config_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let fee_collector_info = next_account_info(account_info_iter)?;

        let config_account = Config::try_from_slice(&config_info.data.borrow())?;
        if config_account.is_initialized {
            return Err(CTokenError::AlreadyInUse.into());
        }

        let config = Config {
            is_initialized: true,
            owner: *owner_info.key,
            authority: *authority_info.key,
            fee,
            fee_collector: *fee_collector_info.key,
        };
        config.serialize(&mut *config_info.data.borrow_mut())?;

        Ok(())
    }

    pub fn process_transfer_owner(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let config_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;
        let new_owner_info = next_account_info(account_info_iter)?;
        if config_info.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut config = Config::try_from_slice(&config_info.data.borrow())?;

        if !owner_info.is_signer || *owner_info.key != config.owner {
            return Err(CTokenError::InvalidOwner.into());
        }
        config.owner = *new_owner_info.key;
        config.serialize(&mut *config_info.data.borrow_mut())?;

        msg!("Owner change to {}", new_owner_info.key);

        Ok(())
    }

    pub fn process_change_authority(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let config_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;
        let new_authority_info = next_account_info(account_info_iter)?;
        if config_info.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut config = Config::try_from_slice(&config_info.data.borrow())?;

        if !owner_info.is_signer || *owner_info.key != config.owner {
            return Err(CTokenError::InvalidOwner.into());
        }
        config.authority = *new_authority_info.key;
        config.serialize(&mut *config_info.data.borrow_mut())?;

        msg!("Authority change to {}", new_authority_info.key);

        Ok(())
    }

    pub fn process_change_fee(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        fee: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let config_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;
        if config_info.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut config = Config::try_from_slice(&config_info.data.borrow())?;

        if !owner_info.is_signer || *owner_info.key != config.owner {
            return Err(CTokenError::InvalidOwner.into());
        }
        config.fee = fee;
        if fee > 0 {
            let fee_collector = next_account_info(account_info_iter)?;
            config.fee_collector = *fee_collector.key;
        }
        config.serialize(&mut *config_info.data.borrow_mut())?;

        msg!("Fee change to {}", fee);

        Ok(())
    }

    pub fn process_change_limit(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        max: u64,
        min: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        if min > max {
            return Err(CTokenError::InvalidInput.into());
        }

        let config_info = next_account_info(account_info_iter)?;
        let c_token_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;
        if config_info.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        if c_token_info.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let config = Config::try_from_slice(&config_info.data.borrow())?;

        if !owner_info.is_signer || *owner_info.key != config.owner {
            return Err(CTokenError::InvalidOwner.into());
        }

        let mut c_token = CToken::try_from_slice(&c_token_info.data.borrow())?;
        if c_token.config != *config_info.key {
            return Err(CTokenError::InvalidConfig.into());
        }
        c_token.max = max;
        c_token.min = min;
        c_token.serialize(&mut *c_token_info.data.borrow_mut())?;

        msg!("cToken bridge limit change to {} - {}", max, min);

        Ok(())
    }

    pub fn process_create(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        destination: u32,
        max: u64,
        min: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let c_token_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let token_mint_info = next_account_info(account_info_iter)?;
        let token_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;
        let config_info = next_account_info(account_info_iter)?;

        if min > max {
            return Err(CTokenError::InvalidInput.into());
        }

        let config = Config::try_from_slice(&config_info.data.borrow())?;
        if config_info.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        if !owner_info.is_signer || *owner_info.key != config.owner {
            return Err(CTokenError::InvalidOwner.into());
        }

        let token_program_id = *token_program_info.key;
        let c_token_account = CToken::try_from_slice(&c_token_info.data.borrow())?;
        if c_token_account.is_initialized {
            return Err(CTokenError::AlreadyInUse.into());
        }

        let (c_token_authority, bump_seed) =
            Pubkey::find_program_address(&[&c_token_info.key.to_bytes()], program_id);
        if *authority_info.key != c_token_authority {
            return Err(CTokenError::InvalidProgramAddress.into());
        }

        if destination != 0 {
            let token = Self::unpack_token_account(token_info, &token_program_id)?;
            if *authority_info.key != token.owner {
                return Err(CTokenError::InvalidToken.into());
            }
            if *token_mint_info.key != token.mint {
                return Err(CTokenError::InvalidToken.into());
            }
        } else {
            let token_mint = Self::unpack_mint(token_mint_info, &token_program_id)?;
            if COption::Some(*authority_info.key) != token_mint.mint_authority {
                return Err(CTokenError::InvalidOwner.into());
            }
        }

        let c_token = CToken {
            is_initialized: true,
            bump_seed,
            token_program_id,
            config: *config_info.key,
            token: *token_info.key,
            token_mint: *token_mint_info.key,
            destination,
            index: 0,
            max,
            min,
        };
        c_token.serialize(&mut *c_token_info.data.borrow_mut())?;

        msg!(
            "Created cToken {} for token mint {}",
            c_token_info.key,
            token_mint_info.key
        );

        Ok(())
    }

    pub fn process_bridge(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        recipient: String,
        payload: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let c_token_info = next_account_info(account_info_iter)?;
        let c_token_token_info = next_account_info(account_info_iter)?;
        let user_info = next_account_info(account_info_iter)?;
        let user_transfer_authority_info = next_account_info(account_info_iter)?;
        let token_mint_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;
        let config_info = next_account_info(account_info_iter)?;

        let config = Config::try_from_slice(&config_info.data.borrow())?;
        if config_info.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if c_token_info.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut c_token = CToken::try_from_slice(&c_token_info.data.borrow())?;
        if c_token.config != *config_info.key {
            return Err(CTokenError::InvalidConfig.into());
        }
        if token_program_info.key != &c_token.token_program_id {
            return Err(CTokenError::InvalidInput.into());
        }
        if amount > c_token.max || amount < c_token.min {
            return Err(CTokenError::InvalidAmount.into());
        }
        if c_token.token_mint != *token_mint_info.key {
            return Err(CTokenError::InvalidMint.into());
        }

        let token_mint = Self::unpack_mint(token_mint_info, &c_token.token_program_id)?;

        if config.fee > 0 {
            let payer = next_account_info(account_info_iter)?;
            let fee_collector = next_account_info(account_info_iter)?;
            if config.fee_collector != *fee_collector.key {
                return Err(CTokenError::InvalidFeeCollector.into());
            }

            invoke(
                &system_instruction::transfer(payer.key, fee_collector.key, config.fee),
                &[payer.clone(), fee_collector.clone()],
            )?;
        }

        if c_token.destination == 0 {
            // burn token
            Self::token_burn(
                c_token_info.key,
                token_program_info.clone(),
                user_info.clone(),
                token_mint_info.clone(),
                user_transfer_authority_info.clone(),
                c_token.bump_seed,
                amount,
            )?;
        } else {
            if user_info.key == c_token_token_info.key {
                return Err(CTokenError::InvalidInput.into());
            }
            // lock token
            Self::token_transfer(
                c_token_info.key,
                token_program_info.clone(),
                user_info.clone(),
                token_mint_info.clone(),
                c_token_token_info.clone(),
                user_transfer_authority_info.clone(),
                c_token.bump_seed,
                amount,
                token_mint.decimals,
            )?;
        }
        c_token.index = c_token.index + 1;
        c_token.serialize(&mut *c_token_info.data.borrow_mut())?;

        let bridge_log = log::Bridge {
            token: c_token.token_mint,
            index: c_token.index,
            sender: *user_info.key,
            recipient,
            amount,
            fee: config.fee,
            destination: c_token.destination,
            payload: payload.to_vec(),
        };
        msg!("Bridge: {}", bridge_log);

        Ok(())
    }

    pub fn process_settle(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let c_token_info = next_account_info(account_info_iter)?;
        let token_authority_info = next_account_info(account_info_iter)?;
        let c_token_token_info = next_account_info(account_info_iter)?;
        let user_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let token_mint_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;
        let config_info = next_account_info(account_info_iter)?;

        if c_token_info.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let config = Config::try_from_slice(&config_info.data.borrow())?;
        if config_info.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let c_token = CToken::try_from_slice(&c_token_info.data.borrow())?;
        if c_token.config != *config_info.key {
            return Err(CTokenError::InvalidConfig.into());
        }

        if !authority_info.is_signer || *authority_info.key != config.authority {
            return Err(CTokenError::InvalidAuthority.into());
        }

        if *token_authority_info.key
            != Self::authority_id(program_id, c_token_info.key, c_token.bump_seed)?
        {
            return Err(CTokenError::InvalidProgramAddress.into());
        }

        let token_mint = Self::unpack_mint(token_mint_info, &c_token.token_program_id)?;

        if c_token.destination == 0 {
            Self::token_mint_to(
                c_token_info.key,
                token_program_info.clone(),
                token_mint_info.clone(),
                user_info.clone(),
                token_authority_info.clone(),
                c_token.bump_seed,
                amount,
            )?;
        } else {
            Self::token_transfer(
                c_token_info.key,
                token_program_info.clone(),
                c_token_token_info.clone(),
                token_mint_info.clone(),
                user_info.clone(),
                token_authority_info.clone(),
                c_token.bump_seed,
                amount,
                token_mint.decimals,
            )?;
        }
        msg!(
            "Settle {} {} to {}",
            amount,
            &c_token.token_mint,
            user_info.key,
        );

        Ok(())
    }

    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = CTokenInstruction::try_from_slice(input)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            CTokenInstruction::Config { fee } => Processor::process_initial_config(accounts, fee),
            CTokenInstruction::TransferOwner => {
                Processor::process_transfer_owner(program_id, accounts)
            }
            CTokenInstruction::ChangeAuthority => {
                Processor::process_change_authority(program_id, accounts)
            }
            CTokenInstruction::ChangeFee { fee } => {
                Processor::process_change_fee(program_id, accounts, fee)
            }
            CTokenInstruction::Create {
                destination,
                max,
                min,
            } => Processor::process_create(program_id, accounts, destination, max, min),
            CTokenInstruction::ChangeLimit { max, min } => {
                Processor::process_change_limit(program_id, accounts, max, min)
            }
            CTokenInstruction::Bridge {
                amount,
                recipient,
                payload,
            } => Processor::process_bridge(program_id, accounts, amount, recipient, &payload),
            CTokenInstruction::Settle { amount } => {
                Processor::process_settle(program_id, accounts, amount)
            }
        }
    }
}

fn invoke_signed_wrapper<T>(
    instruction: &Instruction,
    account_infos: &[AccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError>
where
    T: 'static + PrintProgramError + DecodeError<T> + FromPrimitive + Error,
{
    invoke_signed(instruction, account_infos, signers_seeds).map_err(|err| {
        err.print::<T>();
        err
    })
}
