//! Program state processor

use {
    super::signature::secp256k1::{secp256k1_verify, Data},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        instruction::Instruction,
        msg,
        program::invoke_signed,
        program_error::ProgramError,
        program_pack::*,
        pubkey::Pubkey,
        sysvar::Sysvar,
    },
    spl_governance::state::{
        enums::{ProposalState, TransactionExecutionStatus},
        governance::get_governance_data,
        native_treasury::get_native_treasury_address_seeds,
        proposal::{get_proposal_data_for_governance, OptionVoteResult},
        proposal_transaction::get_proposal_transaction_data_for_proposal,
    },
    spl_token::{
        state::{Account, Mint},
        ID as TOKEN_PROGRAM_ID,
    },
};

/// Processes ExecuteTransaction instruction
pub fn process_execute_transaction(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let instructions_sysvar_account = next_account_info(account_info_iter)?; // -1
    let governance_info = next_account_info(account_info_iter)?; // 0
    let token_mint = next_account_info(account_info_iter)?; // 1
    let user_ata = next_account_info(account_info_iter)?; // 2
    let mint_auth = next_account_info(account_info_iter)?; // 3
    let token_program = next_account_info(account_info_iter)?; // 4

    // let proposal_info = next_account_info(account_info_iter)?; // 1
    // let proposal_transaction_info = next_account_info(account_info_iter)?; // 2

    let clock = Clock::get()?;

    let governance_data = get_governance_data(program_id, governance_info)?;

    // TODO: this is a temp verification
    let msgs = secp256k1_verify(&instructions_sysvar_account)?;
    msg!("secp256k1_verify: {:?}", msgs);

    // let mut proposal_transaction_data = get_proposal_transaction_data_for_proposal(
    //     program_id,
    //     proposal_transaction_info,
    //     proposal_info.key,
    // )?;

    // Proposal is always executed in the POC
    // TODO: Enable Proposal after POC
    // let mut proposal_data =
    //     get_proposal_data_for_governance(program_id, proposal_info, governance_info.key)?;
    // proposal_data
    //     .assert_can_execute_transaction(&proposal_transaction_data, clock.unix_timestamp)?;

    // Execute instruction with Governance PDA as signer
    // let instructions = proposal_transaction_data
    //     .instructions
    //     .iter()
    //     .map(Instruction::from);

    // In the current implementation accounts for all instructions are passed to
    // each instruction invocation. This is an overhead but shouldn't be a
    // showstopper because if we can invoke the parent instruction with that many
    // accounts then we should also be able to invoke all the nested ones
    // TODO: Optimize the invocation to split the provided accounts for each
    // individual instruction
    // let instruction_account_infos = account_info_iter.as_slice();

    let mut signers_seeds: Vec<&[&[u8]]> = vec![];

    // Sign the transaction using the governance PDA
    let mut governance_seeds = governance_data.get_governance_address_seeds()?.to_vec();
    let (pda, bump_seed) = Pubkey::find_program_address(&governance_seeds, program_id);

    if pda != *mint_auth.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidSeeds);
    }

    if TOKEN_PROGRAM_ID != *token_program.key {
        msg!("Invalid TOKEN_PROGRAM_ID");
        return Err(ProgramError::InvalidSeeds);
    }

    let bump = &[bump_seed];
    governance_seeds.push(bump);

    signers_seeds.push(&governance_seeds[..]);

    // TODO: Enable Proposal after POC
    // proposal_data.executing_at = Some(clock.unix_timestamp);
    // proposal_data.state = ProposalState::Executing;

    // TODO: Hardcode Mint_to instruction for POC
    // for instruction in instructions {
    invoke_signed(
        &spl_token::instruction::mint_to(
            &token_program.key,
            token_mint.key,
            user_ata.key,
            mint_auth.key,
            &[],
            10,
        )?,
        &[token_mint.clone(), user_ata.clone(), mint_auth.clone()],
        &signers_seeds[..],
    )?;
    // }

    let mint = Mint::unpack(&token_mint.data.borrow())?;
    let destination_account = Account::unpack(&user_ata.data.borrow())?;
    msg!("Mint token: {:?}", mint);
    msg!("Mint amount: 10");
    msg!("Destination account: {:?}", destination_account);

    // Update proposal and instruction accounts

    // TODO: Enable Proposal after POC
    // let option = &mut proposal_data.options[proposal_transaction_data.option_index as usize];
    // option.transactions_executed_count = option.transactions_executed_count.checked_add(1).unwrap();
    // proposal_data.closed_at = Some(clock.unix_timestamp);
    // proposal_data.state = ProposalState::Completed;
    // proposal_data.serialize(&mut proposal_info.data.borrow_mut()[..])?;

    // proposal_transaction_data.executed_at = Some(clock.unix_timestamp);
    // proposal_transaction_data.execution_status = TransactionExecutionStatus::Success;
    // proposal_transaction_data.serialize(&mut proposal_transaction_info.data.borrow_mut()[..])?;

    Ok(())
}
