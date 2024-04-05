use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NotepadInstructionPayload {
    pub contents: String, //文本内容
    pub pubkey: Pubkey,   //文本权限
}

impl NotepadInstructionPayload {}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum NotepadInstruction {
    NoteCreate { contents: String, pubkey: Pubkey },
    NoteUpdate { contents: String },
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
                    contents: payload.contents,
                    pubkey: payload.pubkey,
                }
            }
            1 => {
                let payload = NotepadInstructionPayload::try_from_slice(rest).unwrap();
                Self::NoteUpdate {
                    contents: payload.contents,
                }
            }
            2 => Self::NoteDelete,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
