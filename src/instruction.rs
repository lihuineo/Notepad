use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NotepadInstructionPayload {
    pub title: String,
    pub body: String,
    pub pubkey: Pubkey,
}

impl NotepadInstructionPayload {}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum NotepadInstruction {
    NoteCreate {
        title: String,
        body: String,
        pubkey: Pubkey,
    },
    NoteUpdate {
        title: String,
        body: String,
    },
    NoteDelete,
}

impl NotepadInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&var, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match var {
            0 => {
                let payload = NotepadInstructionPayload::try_from_slice(rest).unwrap();
                Self::NoteCreate {
                    title: payload.title,
                    body: payload.body,
                    pubkey: payload.pubkey,
                }
            }
            1 => {
                let payload = NotepadInstructionPayload::try_from_slice(rest).unwrap();
                Self::NoteUpdate {
                    title: payload.title,
                    body: payload.body,
                }
            }
            2 => Self::NoteDelete,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
