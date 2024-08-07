use {
    super::MessageParser,
    solana_program::{
        account_info::AccountInfo, hash::hash, program_error::ProgramError, pubkey::Pubkey,
    },
    spl_governance::state::{
        proposal_transaction::InstructionData,
        vote_record::{Vote, VoteChoice},
    },
};

pub struct DummyProtocol<'a> {
    raw: &'a Vec<&'a Vec<u8>>,
}

impl<'a> MessageParser<'a> for DummyProtocol<'a> {
    fn new(_: &'a [u8], message: &'a Vec<&Vec<u8>>) -> Self {
        DummyProtocol { raw: message }
    }

    fn validate(&self, _: &Pubkey) -> Result<(), ProgramError> {
        Ok(())
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
        _: &[AccountInfo],
    ) -> Result<Vec<InstructionData>, ProgramError> {
        Ok(proposal_instruction.clone())
    }
}
