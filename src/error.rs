use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]

pub enum NotepadError {
    #[error("Notepad title exceeds max len!")]
    InvalidTitleLen,
    #[error("Notepad body exceeds max len!")]
    InvalidBodyLen,
    #[error("Notepad invalid public key!")]
    InvalidPubkey,
}

impl From<NotepadError> for ProgramError {
    fn from(value: NotepadError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
