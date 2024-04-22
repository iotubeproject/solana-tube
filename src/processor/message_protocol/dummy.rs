use {
    super::MessageParser,
    solana_program::{hash::hash, msg, program_error::ProgramError},
    spl_governance::state::{
        proposal_transaction::InstructionData,
        vote_record::{Vote, VoteChoice},
    },
    spl_token::instruction::TokenInstruction,
};

pub struct DummyProtocol<'a> {
    raw: &'a Vec<&'a Vec<u8>>,
}

impl<'a> MessageParser<'a> for DummyProtocol<'a> {
    fn new(message: &'a Vec<&Vec<u8>>) -> Self {
        DummyProtocol { raw: message }
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
        Ok(hash(&self.raw[0]).to_bytes())
    }

    fn instructions_from_proposal(
        &self,
        proposal_instruction: &Vec<InstructionData>,
    ) -> Result<Vec<InstructionData>, ProgramError> {
        // Ok(proposal_instruction.clone())
        let mut new_ints = proposal_instruction.clone();
        for instruction in new_ints.iter_mut() {
            if let TokenInstruction::MintTo { amount } =
                TokenInstruction::unpack(&instruction.data)?
            {
                msg!("original MintTo amount: {:?}", amount);
                let new_amount = amount.checked_mul(2).unwrap();
                msg!("new MintTo amount: {:?}", new_amount);
                instruction.data = TokenInstruction::MintTo { amount: new_amount }.pack();
            }
        }
        Ok(new_ints)
    }
}
