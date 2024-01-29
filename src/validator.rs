use secp256k1_module::secp256k1_extract_signer;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
mod secp256k1_module;

struct StateData {
    validators: std::collections::BTreeMap<Pubkey, ()>,
}

// Solana entrypoint ID
solana_program::declare_id!("ValidateSig1111111111111111111111111111111111");

// Program entry point
pub fn id() -> Pubkey {
    solana_program::declare_id!("ValidateSig1111111111111111111111111111111111")
}

impl StateData {
    // Deserialize the program state from account data
    fn deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        Ok(bincode::deserialize(data).unwrap_or_default())
    }

    fn unpack() {}

    fn pack() {}
}

// Entrypoint for Solana program
entrypoint!(process_instruction);

// Process the instruction
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Retrieve the StateData account
    let state_data_account = next_account_info(accounts)?;

    // Deserialize the StateData from the account data
    let mut state_data = StateData::unpack(&state_data_account.data.borrow())?;

    match instruction_data.get(0) {
        Some(&1) => {
            // If the first byte is 1, add validator
            add_validator(&mut state_data, accounts)?;
            StateData::pack(state_data, &mut state_data_account.data.borrow_mut())?;
        }
        Some(&2) => {
            // If the first byte is 2, delete validator
            delete_validator(&mut state_data, accounts)?;
            StateData::pack(state_data, &mut state_data_account.data.borrow_mut())?;
        }
        _ => {
            // Otherwise, call submit
            submit(accounts, instruction_data, &state_data)?;
        }
    }

    // Update the StateData in the account data

    Ok(())
}

// Function to add a validator to the StateData
fn add_validator(state_data: &mut StateData, _accounts: &[AccountInfo]) -> ProgramResult {
    // Example: Add a validator (modify based on your logic)
    let new_validator = Pubkey::new_unique();
    state_data.validators.insert(new_validator);

    msg!("Validator added: {:?}", new_validator);

    Ok(())
}

// Function to delete a validator from the StateData
fn delete_validator(state_data: &mut StateData, _accounts: &[AccountInfo]) -> ProgramResult {
    // Example: Delete a validator if it exists
    let validator_to_delete = Pubkey::new_unique();
    state_data.validators.remove(&validator_to_delete);

    msg!("Validator removed: {:?}", validator_to_delete);

    Ok(())
}

fn validate_signatures(
    validators: BTreeMap<Pubkey, ()>,
    message: Vec<u8>,
    signatures: Vec<Vec<u8>>,
) -> ProgramResult {
    let mut encountered_signers = std::collections::HashSet::new();
    let mut valid_signatures = 0;
    for signature in signatures {
        let signer = match secp256k1_extract_signer(&message, &signature) {
            Ok(Some(signer)) => signer,
            _ => {
                return Err(ProgramError::Custom(1)); // Custom error code for invalid signature
            }
        };
        if !validators.contains_key(&signer) {
            return Err(ProgramError::Custom(2)); // Custom error code for invalid signer
        }
        if !encountered_signers.insert(signer) {
            return Err(ProgramError::Custom(3)); // Custom error code for duplicate signer
        }
        valid_signatures += 1;
    }

    // Check if more than 2/3 of validators have signed
    let required_signatures = (state_data.validators.len() as f64 * 2.0 / 3.0).ceil() as usize;
    if valid_signatures <= required_signatures {
        return Err(ProgramError::Custom(4)); // Custom error code for insufficient signatures
    }
    Ok(())
}

fn submit(
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
    _state_data: &StateData,
) -> ProgramResult {
    // decode instruction_data
    validate_signatures(_state_data.validators, message, signatures);
    // make transfer
    Ok(())
}

