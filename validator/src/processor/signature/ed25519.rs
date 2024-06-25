use solana_program::{
    account_info::AccountInfo, ed25519_program, program_error::ProgramError, pubkey::Pubkey, sysvar,
};
#[derive(Debug)]
pub struct Data {
    pub pubkey: Pubkey,
    pub message: Vec<u8>,
}
pub fn ed25519_verify(
    instructions_sysvar_account: &AccountInfo,
) -> Result<Vec<Data>, ProgramError> {
    assert!(sysvar::instructions::check_id(
        instructions_sysvar_account.key
    ));
    let cur_index = sysvar::instructions::load_current_index_checked(instructions_sysvar_account)?;
    assert!(cur_index > 0, "cur_index should be greater than 0");
    let ed25519_instr_index = cur_index - 1;
    let ed25519_instr = sysvar::instructions::load_instruction_at_checked(
        ed25519_instr_index as usize,
        instructions_sysvar_account,
    )?;
    assert!(ed25519_program::check_id(&ed25519_instr.program_id));
    let mut data_array = vec![];
    for offsets in ed25519_defs::iter_signature_offsets(&ed25519_instr.data)? {
        assert_eq!(ed25519_instr_index, offsets.signature_instruction_index);
        assert_eq!(ed25519_instr_index, offsets.public_key_instruction_index);
        assert_eq!(ed25519_instr_index, offsets.message_instruction_index);
        let pubkey = &ed25519_instr.data[offsets.public_key_offset as usize
            ..offsets.public_key_offset as usize + ed25519_defs::PUBKEY_SERIALIZED_SIZE];
        let message = &ed25519_instr.data[offsets.message_data_offset as usize
            ..offsets.message_data_offset as usize + offsets.message_data_size as usize];
        let publickey = Pubkey::try_from(pubkey).unwrap();
        let message = Vec::from(message);
        data_array.push(Data {
            pubkey: publickey,
            message,
        })
    }
    Ok(data_array)
}
mod ed25519_defs {
    use solana_program::program_error::ProgramError;
    use std::iter::Iterator;
    pub const PUBKEY_SERIALIZED_SIZE: usize = 32;
    pub const SIGNATURE_OFFSETS_SERIALIZED_SIZE: usize = 14;
    pub const SIGNATURE_OFFSETS_START: usize = 2;
    pub struct Ed25519SignatureOffsets {
        pub signature_offset: u16, // offset to ed25519 signature of 64 bytes
        pub signature_instruction_index: u16, // instruction index to find signature
        pub public_key_offset: u16, // offset to public key of 32 bytes
        pub public_key_instruction_index: u16, // instruction index to find public key
        pub message_data_offset: u16, // offset to start of message data
        pub message_data_size: u16, // size of message data
        pub message_instruction_index: u16, // index of instruction data to get message data
    }
    pub fn iter_signature_offsets(
        ed25519_instr_data: &[u8],
    ) -> Result<impl Iterator<Item = Ed25519SignatureOffsets> + '_, ProgramError> {
        let num_structs = *ed25519_instr_data
            .get(0)
            .ok_or(ProgramError::InvalidArgument)?;
        let all_structs_size = SIGNATURE_OFFSETS_SERIALIZED_SIZE * num_structs as usize;
        let all_structs_slice = ed25519_instr_data
            .get(SIGNATURE_OFFSETS_START..all_structs_size + SIGNATURE_OFFSETS_START)
            .ok_or(ProgramError::InvalidArgument)?;
        fn decode_u16(chunk: &[u8], index: usize) -> u16 {
            u16::from_le_bytes(<[u8; 2]>::try_from(&chunk[index..index + 2]).unwrap())
        }
        Ok(all_structs_slice
            .chunks(SIGNATURE_OFFSETS_SERIALIZED_SIZE)
            .map(|chunk| Ed25519SignatureOffsets {
                signature_offset: decode_u16(chunk, 0),
                signature_instruction_index: decode_u16(chunk, 2),
                public_key_offset: decode_u16(chunk, 4),
                public_key_instruction_index: decode_u16(chunk, 6),
                message_data_offset: decode_u16(chunk, 8),
                message_data_size: decode_u16(chunk, 10),
                message_instruction_index: decode_u16(chunk, 12),
            }))
    }
}
