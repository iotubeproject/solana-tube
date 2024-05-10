use {
    super::MessageParser,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{keccak::hash, program_error::ProgramError, pubkey::Pubkey},
    spl_governance::state::{
        proposal_transaction::InstructionData,
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
    ) -> Result<Vec<InstructionData>, ProgramError> {
        let mut new_ints = proposal_instruction.clone();
        for instruction in new_ints.iter_mut() {
            // if let TokenInstruction::MintTo { amount } =
            //     TokenInstruction::unpack(&instruction.data)?
            // {
            //     msg!("original MintTo amount: {:?}", amount);
            //     let new_amount = amount.checked_mul(2).unwrap();
            //     msg!("new MintTo amount: {:?}", new_amount);
            //     instruction.data = TokenInstruction::MintTo { amount: new_amount }.pack();
            // }
        }
        Ok(new_ints)
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
