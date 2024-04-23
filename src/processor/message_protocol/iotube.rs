use {
    super::MessageParser,
    solana_program::{hash::hash, program_error::ProgramError},
    spl_governance::state::{
        proposal_transaction::InstructionData,
        vote_record::{Vote, VoteChoice},
    },
};

pub struct IoTubeProtocol<'a> {
    raw: &'a Vec<&'a Vec<u8>>,
    validated: bool,
}

impl<'a> MessageParser<'a> for IoTubeProtocol<'a> {
    fn new(message: &'a Vec<&Vec<u8>>) -> Self {
        IoTubeProtocol {
            raw: message,
            validated: false,
        }
    }

    fn votes(&self) -> Result<Vec<Vote>, ProgramError> {
        if let Some(first) = self.raw.get(0) {
            if !self.raw.iter().all(|v| v == first) {
                return Err(ProgramError::InvalidAccountData);
            };
            return Ok(vec![
                Vote::Approve(vec![VoteChoice {
                    rank: 0,
                    weight_percentage: 100,
                },]);
                self.raw.len()
            ]);
        } else {
            return Ok(vec![]);
        }
    }

    fn record_id(&self) -> Result<[u8; 32], ProgramError> {
        if let Some(first) = self.raw.get(0) {
            if !self.raw.iter().all(|v| v == first) {
                return Err(ProgramError::InvalidAccountData);
            };
            // TODO: keccak256 hash
            return Ok(hash(&self.raw[0]).to_bytes());
        } else {
            return Err(ProgramError::InvalidAccountData);
        }
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
        if self.validated {
            return Ok(());
        }
        s
    }
}
