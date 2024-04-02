use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NotepadInstructionPayload {
    title: String,
    body: String,
    pubkey: Pubkey,
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

#[cfg(test)]
mod tests {
    use super::{NotepadInstruction, NotepadInstructionPayload};
    use borsh::BorshSerialize;
    use solana_program::pubkey;
    use std::str::FromStr;

    #[test]
    fn unpack_test() {
        let mut input: Vec<u8> = Vec::new();
        input.push(0u8);

        let mut payload = NotepadInstructionPayload {
            title: String::from("first input"),
            body: String::from("wo"),
            pubkey: pubkey::Pubkey::from_str("666c9Vw6FsjbRhxsVeC4kFR5jQQPqZip9CNMw1ivAGb1")
                .unwrap(),
        };

        payload.serialize(&mut input).unwrap();

        let ins = NotepadInstruction::unpack(&input).unwrap();
        println!("hello");
    }
}
