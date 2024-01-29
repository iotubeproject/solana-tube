// File: secp256k1_module.rs

use secp256k1::{PublicKey, Secp256k1, Signature};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

// The secp256k1_extract_signer function for signature verification
pub fn secp256k1_extract_signer(
    message: &[u8],
    signature_bytes: &[u8],
) -> Result<Option<Pubkey>, ProgramError> {
    // Create a Secp256k1 context
    let secp = Secp256k1::new();

    // Parse the signature
    let signature =
        Signature::from_der(signature_bytes).map_err(|_| ProgramError::InvalidArgument)?;

    // Recover the public key from the signature
    let public_key = secp.recover(&secp256k1::Message::from_slice(message), &signature)?;

    // Serialize the public key to bytes
    let serialized_public_key = public_key.serialize();

    // Return the recovered public key
    Ok(Some(Pubkey::new(&serialized_public_key)))
}

