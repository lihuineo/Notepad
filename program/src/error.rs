use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::program_error::ProgramError;
use solana_program::{decode_error::DecodeError, msg, program_error::PrintProgramError};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]

pub enum NotepadError {
    #[error("Invalid Contents Len")]
    InvalidContentsLen, //超出文本长度错误
    #[error("Invalid Pubkey")]
    InvalidPubkey, //权限错误
}

impl From<NotepadError> for ProgramError {
    fn from(value: NotepadError) -> Self {
        ProgramError::Custom(value as u32)
    }
}

impl PrintProgramError for NotepadError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            NotepadError::InvalidContentsLen => {
                msg!("Error: input contents exceeds max len!")
            }
            NotepadError::InvalidPubkey => {
                msg!("Error: input invalid public key!")
            }
        }
    }
}
