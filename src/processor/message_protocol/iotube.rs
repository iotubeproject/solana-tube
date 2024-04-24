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
        let payload = Payload::try_from_slice(&self.raw[0])?;
        return Ok(hash(&payload.bytes()).to_bytes());
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
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.program_id.to_bytes());
        bytes.extend_from_slice(&self.cashier);
        bytes.extend_from_slice(&self.co_token.to_bytes());
        bytes.extend_from_slice(&self.index.to_le_bytes());
        bytes.extend_from_slice(self.sender.as_bytes());
        bytes.extend_from_slice(&self.recipient.to_bytes());
        bytes.extend_from_slice(&self.amount.to_le_bytes());
        bytes
    }
}
