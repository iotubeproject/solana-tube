use {
    super::MessageParser,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{hash::hash, program_error::ProgramError, pubkey::Pubkey},
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

    fn votes(&self) -> Result<Vec<Vote>, ProgramError> {
        self.validate()?;
        return Ok(vec![
            Vote::Approve(vec![VoteChoice {
                rank: 0,
                weight_percentage: 100,
            },]);
            self.raw.len()
        ]);
    }

    fn record_id(&self) -> Result<[u8; 32], ProgramError> {
        // TODO: keccak256 hash
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

impl<'a> IoTubeProtocol<'a> {
    fn validate(&self) -> Result<(), ProgramError> {
        if let Some(first) = self.raw.get(0) {
            if !self.raw.iter().all(|v| v == first) {
                return Err(ProgramError::InvalidAccountData);
            };
            return Ok(());
        } else {
            return Err(ProgramError::InvalidAccountData);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub struct Payload {
    pub program_id: Pubkey,
    // pub votes: Vec<Vote>,
    pub index: u64,
    pub sender: String,
    pub recipient: Pubkey,
    pub amount: u64,
}

impl Payload {
    fn bytes(&self) -> Vec<u8> {
        [
            self.program_id.try_to_vec().unwrap(),
            self.amount.try_to_vec().unwrap(),
        ]
        .concat()
    }
}
