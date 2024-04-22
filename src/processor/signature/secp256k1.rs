use libsecp256k1;
use solana_program::{
    account_info::AccountInfo, msg, program_error::ProgramError, secp256k1_program, sysvar,
};

#[derive(Debug)]
pub struct Data {
    pub eth_address: [u8; secp256k1_defs::HASHED_PUBKEY_SERIALIZED_SIZE],
    pub message: Vec<u8>,
}

/// Validate the secp256k1 instruction, and extract `eth_address` and `message`
/// from the verified data
#[allow(dead_code)]
pub fn secp256k1_verify(
    instructions_sysvar_account: &AccountInfo,
) -> Result<Vec<Data>, ProgramError> {
    // The instructions sysvar gives access to the instructions in the transaction.
    assert!(sysvar::instructions::check_id(
        instructions_sysvar_account.key
    ));

    // Load the secp256k1 instruction.
    // `new_secp256k1_instruction` generates an instruction that must be at index 0.
    let secp256k1_instr =
        sysvar::instructions::load_instruction_at_checked(0, instructions_sysvar_account)?;

    // Verify it is a secp256k1 instruction.
    // This is security-critical - what if the transaction uses an imposter secp256k1 program?
    assert!(secp256k1_program::check_id(&secp256k1_instr.program_id));

    let mut data_array = vec![];

    for offsets in secp256k1_defs::iter_signature_offsets(&secp256k1_instr.data)? {
        // `new_secp256k1_instruction` generates an instruction that only uses instruction index 0.
        assert_eq!(0, offsets.signature_instruction_index);
        assert_eq!(0, offsets.eth_address_instruction_index);
        assert_eq!(0, offsets.message_instruction_index);

        // These indexes must all be valid because the runtime already verified them.
        let signature = &secp256k1_instr.data[offsets.signature_offset as usize
            ..offsets.signature_offset as usize + secp256k1_defs::SIGNATURE_SERIALIZED_SIZE];
        let eth_address = &secp256k1_instr.data[offsets.eth_address_offset as usize
            ..offsets.eth_address_offset as usize + secp256k1_defs::HASHED_PUBKEY_SERIALIZED_SIZE];
        let message = &secp256k1_instr.data[offsets.message_data_offset as usize
            ..offsets.message_data_offset as usize + offsets.message_data_size as usize];

        let signature = libsecp256k1::Signature::parse_standard_slice(signature)
            .map_err(|_| ProgramError::InvalidArgument)?;
        if signature.s.is_high() {
            msg!("signature with high-s value");
            return Err(ProgramError::InvalidArgument);
        }

        let eth_address =
            <[u8; secp256k1_defs::HASHED_PUBKEY_SERIALIZED_SIZE]>::try_from(eth_address).unwrap();
        let message = Vec::from(message);

        data_array.push(Data {
            eth_address,
            message,
        })
    }
    Ok(data_array)
}

mod secp256k1_defs {
    use solana_program::program_error::ProgramError;
    use std::iter::Iterator;

    pub const HASHED_PUBKEY_SERIALIZED_SIZE: usize = 20;
    pub const SIGNATURE_SERIALIZED_SIZE: usize = 64;
    pub const SIGNATURE_OFFSETS_SERIALIZED_SIZE: usize = 11;

    /// The structure encoded in the secp2256k1 instruction data.
    pub struct SecpSignatureOffsets {
        pub signature_offset: u16,
        pub signature_instruction_index: u8,
        pub eth_address_offset: u16,
        pub eth_address_instruction_index: u8,
        pub message_data_offset: u16,
        pub message_data_size: u16,
        pub message_instruction_index: u8,
    }

    pub fn iter_signature_offsets(
        secp256k1_instr_data: &[u8],
    ) -> Result<impl Iterator<Item = SecpSignatureOffsets> + '_, ProgramError> {
        // First element is the number of `SecpSignatureOffsets`.
        let num_structs = *secp256k1_instr_data
            .get(0)
            .ok_or(ProgramError::InvalidArgument)?;

        let all_structs_size = SIGNATURE_OFFSETS_SERIALIZED_SIZE * num_structs as usize;
        let all_structs_slice = secp256k1_instr_data
            .get(1..all_structs_size + 1)
            .ok_or(ProgramError::InvalidArgument)?;

        fn decode_u16(chunk: &[u8], index: usize) -> u16 {
            u16::from_le_bytes(<[u8; 2]>::try_from(&chunk[index..index + 2]).unwrap())
        }

        Ok(all_structs_slice
            .chunks(SIGNATURE_OFFSETS_SERIALIZED_SIZE)
            .map(|chunk| SecpSignatureOffsets {
                signature_offset: decode_u16(chunk, 0),
                signature_instruction_index: chunk[2],
                eth_address_offset: decode_u16(chunk, 3),
                eth_address_instruction_index: chunk[5],
                message_data_offset: decode_u16(chunk, 6),
                message_data_size: decode_u16(chunk, 8),
                message_instruction_index: chunk[10],
            }))
    }
}
