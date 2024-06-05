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
    raw_data: &'a [u8],
    hashes: &'a Vec<&'a Vec<u8>>,
}
impl<'a> MessageParser<'a> for IoTubeProtocol<'a> {
    fn new(raw_data: &'a [u8], hashes: &'a Vec<&Vec<u8>>) -> Self {
        IoTubeProtocol { raw_data, hashes }
    }
    fn validate(&self, program_id: &Pubkey) -> Result<(), ProgramError> {
        if let Some(first) = self.hashes.get(0) {
            if !self.hashes.iter().all(|v| v == first) {
                return Err(ProgramError::InvalidAccountData);
            };
            if &self.record_id()? != first.as_slice() {
                return Err(ProgramError::InvalidAccountData);
            }
            let payload = Payload::try_from_slice(self.raw_data)?;
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
            self.hashes.len()
        ]);
    }
    fn record_id(&self) -> Result<[u8; 32], ProgramError> {
        return Ok(hash(&self.raw_data).to_bytes());
    }
    fn instructions_from_proposal(
        &self,
        proposal_instruction: &Vec<InstructionData>,
        ctoken_infos: &[AccountInfo], // accountinfo for co_token & co_token_programID(later one to be removed)
    ) -> Result<Vec<InstructionData>, ProgramError> {
        let payload = Payload::try_from_slice(self.raw_data)?;
        if ctoken_infos.len() != 1 || *ctoken_infos[0].key != payload.co_token {
            return Err(ProgramError::InvalidAccountData);
        }
        let c_token = CToken::try_from_slice(&ctoken_infos[0].data.borrow())?;
        let authority =
            Pubkey::find_program_address(&[&ctoken_infos[0].key.to_bytes()], ctoken_infos[0].owner)
                .0;
        if proposal_instruction.len() != 1 {
            return Err(ProgramError::InvalidAccountData);
        }
        let mut new_instrs = proposal_instruction.clone();
        for instruction in new_instrs.iter_mut() {
            if let CTokenInstruction::Settle { .. } =
                CTokenInstruction::try_from_slice(&instruction.data)?
            {
                if instruction.accounts.len() != 8 {
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
                    is_writable: true,
                };
                instruction.accounts[7] = AccountMetaData {
                    pubkey: c_token.config,
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
