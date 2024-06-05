#![allow(dead_code)]

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Instructions for CToken
#[derive(Clone, Debug, BorshSchema, BorshDeserialize, BorshSerialize, PartialEq)]
pub enum CTokenInstruction {
    Config {
        fee: u64,
    },
    TransferOwner,
    ChangeAuthority,
    ChangeFee {
        fee: u64,
    },
    Create {
        destination: u32,
        max: u64,
        min: u64,
    },
    ChangeLimit {
        max: u64,
        min: u64,
    },
    Bridge {
        amount: u64,
        recipient: String,
    },
    Settle {
        amount: u64,
    },
}

pub fn settle(
    program_id: &Pubkey,
    c_token: &Pubkey,
    token_authority: &Pubkey,
    c_token_token_account: &Pubkey,
    user_account: &Pubkey,
    authorith: &Pubkey,
    token_mint: &Pubkey,
    token_program_id: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = CTokenInstruction::Settle { amount }.try_to_vec()?;

    let accounts = vec![
        AccountMeta::new_readonly(*c_token, false),
        AccountMeta::new_readonly(*token_authority, false),
        AccountMeta::new(*c_token_token_account, false),
        AccountMeta::new(*user_account, false),
        AccountMeta::new_readonly(*authorith, true),
        AccountMeta::new(*token_mint, false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
