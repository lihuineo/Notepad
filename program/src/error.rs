use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]

pub enum NotepadError {
    #[error("Notepad: input contents exceeds max len!")]
    InvalidContentsLen, //超出文本长度错误
    #[error("Notepad: invalid public key!")]
    InvalidPubkey, //权限错误
}

impl From<NotepadError> for ProgramError {
    fn from(value: NotepadError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
