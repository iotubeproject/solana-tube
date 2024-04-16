use {
    super::MessageParser,
    solana_program::{hash::hash, instruction::Instruction, msg, program_error::ProgramError},
    spl_governance::state::proposal_transaction::*,
    spl_token::instruction::TokenInstruction,
};

pub struct DummyProtocol {
    raw: Vec<u8>,
}

impl MessageParser for DummyProtocol {
    fn new(message: &Vec<u8>) -> Self {
        DummyProtocol {
            raw: message.clone(),
        }
    }

    fn record_id(&self) -> Result<[u8; 32], ProgramError> {
        Ok(hash(&self.raw).to_bytes())
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
