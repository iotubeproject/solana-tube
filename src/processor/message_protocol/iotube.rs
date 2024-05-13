use {
    super::MessageParser,
    borsh::{BorshDeserialize, BorshSerialize},
    ctoken::{instruction::CTokenInstruction, state::CToken},
    solana_program::{
        account_info::AccountInfo, keccak::hash, program_error::ProgramError, pubkey::Pubkey,
    },
    spl_governance::state::{
        proposal_transaction::{AccountMetaData, InstructionData},
        vote_record::{Vote, VoteChoice},
    },
};

pub struct IoTubeProtocol<'a> {
    raw: &'a Vec<&'a Vec<u8>>,
}

impl<'a> MessageParser<'a> for IoTubeProtocol<'a> {
    fn new(message: &'a Vec<&Vec<u8>>) -> Self {
        IoTubeProtocol { raw: message }
    }

    fn validate(&self, program_id: &Pubkey) -> Result<(), ProgramError> {
        if let Some(first) = self.raw.get(0) {
            if !self.raw.iter().all(|v| v == first) {
                return Err(ProgramError::InvalidAccountData);
            };
            let payload = Payload::try_from_slice(first)?;
            payload.validate(program_id)?;
            return Ok(());
        }
        return Err(ProgramError::InvalidAccountData);
    }

    fn votes(&self) -> Result<Vec<Vote>, ProgramError> {
        return Ok(vec![
            Vote::Approve(vec![VoteChoice {
                rank: 0,
                weight_percentage: 100,
            },]);
            self.raw.len()
        ]);
    }

    fn record_id(&self) -> Result<[u8; 32], ProgramError> {
        return Ok(hash(&self.raw[0]).to_bytes());
    }

    fn instructions_from_proposal(
        &self,
        proposal_instruction: &Vec<InstructionData>,
        ctoken_infos: &[AccountInfo], // accountinfo for co_token & co_token_programID(later one to be removed)
    ) -> Result<Vec<InstructionData>, ProgramError> {
        let payload = Payload::try_from_slice(self.raw.get(0).unwrap())?;

        if ctoken_infos.len() != 2 || *ctoken_infos[0].key != payload.co_token {
            return Err(ProgramError::InvalidAccountData);
        }
        let c_token = CToken::try_from_slice(&ctoken_infos[0].data.borrow())?;

        let authority =
            Pubkey::find_program_address(&[&ctoken_infos[0].key.to_bytes()], ctoken_infos[1].key).0;

        if proposal_instruction.len() != 1 {
            return Err(ProgramError::InvalidAccountData);
        }
        let mut new_instrs = proposal_instruction.clone();

        // const keys = [
        //     { pubkey: cToken, isSigner: false, isWritable: false }, //
        //     { pubkey: authority, isSigner: false, isWritable: false }, // from data & programID
        //     { pubkey: cTokenTokenAccount, isSigner: false, isWritable: true }, //
        //     { pubkey: userAccount, isSigner: false, isWritable: true }, //
        //     { pubkey: onwer, isSigner: false, isWritable: false },
        //     { pubkey: tokenMint, isSigner: false, isWritable: false }, //
        //     { pubkey: tokenProgramInfo, isSigner: false, isWritable: false },
        // ];
        for instruction in new_instrs.iter_mut() {
            if let CTokenInstruction::Settle { amount } =
                CTokenInstruction::try_from_slice(&instruction.data)?
            {
                if instruction.accounts.len() != 7 {
                    return Err(ProgramError::InvalidAccountData);
                }
                instruction.data = CTokenInstruction::Settle {
                    amount: payload.amount,
                }
                .try_to_vec()?;

                instruction.accounts[0] = AccountMetaData {
                    pubkey: payload.co_token,
                    is_signer: false,
                    is_writable: false,
                };
                instruction.accounts[1] = AccountMetaData {
                    pubkey: authority,
                    is_signer: false,
                    is_writable: false,
                };
                instruction.accounts[2] = AccountMetaData {
                    pubkey: c_token.token,
                    is_signer: false,
                    is_writable: true,
                };
                instruction.accounts[3] = AccountMetaData {
                    pubkey: payload.recipient,
                    is_signer: false,
                    is_writable: true,
                };
                instruction.accounts[5] = AccountMetaData {
                    pubkey: c_token.token_mint,
                    is_signer: false,
                    is_writable: false,
                };
            }
        }
        Ok(new_instrs)
    }
}

const ETH_ADDRESS_SIZE: usize = 20;

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
struct Payload {
    pub program_id: Pubkey,
    pub cashier: [u8; ETH_ADDRESS_SIZE],
    pub co_token: Pubkey,
    pub index: u64,
    pub sender: String,
    pub recipient: Pubkey,
    pub amount: u64,
}

impl Payload {
    fn validate(&self, program_id: &Pubkey) -> Result<(), ProgramError> {
        if self.program_id != *program_id {
            return Err(ProgramError::InvalidAccountData);
        }
        if self.amount == 0 {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
