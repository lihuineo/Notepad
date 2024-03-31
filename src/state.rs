use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize)]

pub struct NotepadAccountState {
    pub pubkey: Pubkey,
    pub title: &str,
    pub body: &str,
}

impl NotepadAccountState {
    pub const TITLE_MAX_SIZE: usize = 15;
    pub const BODY_MAX_SIZE: usize = 200;
    pub const MAX_SIZE: usize = Self::TITLE_MAX_SIZE + Self::BODY_MAX_SIZE + 32;
}
