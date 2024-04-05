use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize)]

pub struct NotepadAccountState {
    pub pubkey: Pubkey,
    pub contents: String,
}

impl NotepadAccountState {
    pub const CONTENTS_MAX_SIZE: usize = 200;
    pub const MAX_SIZE: usize = Self::CONTENTS_MAX_SIZE + 15;
}
