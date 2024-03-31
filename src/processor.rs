use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    address_lookup_table::instruction,
    borsh1::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::{self, Pubkey},
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
    vote::program,
};

use crate::{error::NotepadError, instruction::NotepadInstruction, state::NotepadAccountState};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ins = NotepadInstruction::unpack(instruction_data);
    match ins {
        NotepadInstruction::NoteCreate {
            title,
            body,
            pubkey,
        } => note_create(program_id, accounts, title, body, pubkey),
        NotepadInstruction::NoteUpdate { title, body } => {
            note_update(program_id, accounts, title, body)
        }
        NotepadInstruction::NoteDelete => node_delete(program_id, accounts),
    }
}

pub fn note_create(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: &str,
    body: &str,
    pubkey: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let from = next_account_info(account_info_iter)?;
    let to = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if title.len() > NotepadAccountState::TITLE_MAX_SIZE {
        return Err(NotepadError::InvalidTitleLen.into());
    }

    if body.len() > NotepadAccountState::BODY_MAX_SIZE {
        return Err(NotepadError::InvalidBodyLen.into());
    }

    let rent = Rent::get();
    let rent_lamports = rent.minimum_balance(NotepadAccountState::MAX_SIZE);

    invoke(
        &system_instruction::create_account(
            from.key,
            to.key,
            rent_lamports,
            NotepadAccountState,
            program_id,
        ),
        &[from.clone(), to.clone(), system_program.clone()],
    )?;
    let mut state = try_from_slice_unchecked::<NotepadAccountState>(&to.data.borrow()).unwrap();
    state.pubkey = pubkey;
    state.title = title;
    state.body = body;
    state.serialize(&mut &mut from.data.borrow_mut()[..])?;
    Ok(())
}

pub fn note_update(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: &str,
    body: &str,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let from = next_account_info(account_info_iter)?;
    let to = next_account_info(account_info_iter)?;

    if title.len() > NotepadAccountState::TITLE_MAX_SIZE {
        return Err(NotepadError::InvalidTitleLen.into());
    }

    if body.len() > NotepadAccountState::BODY_MAX_SIZE {
        return Err(NotepadError::InvalidBodyLen.into());
    }

    let mut state = try_from_slice_unchecked::<NotepadAccountState>(&to.data.borrow()).unwrap();

    if *from.key != state.pubkey {
        return Err(NotepadError::InvalidPubkey.into());
    }

    state.title = title;
    state.body = body;
    state.serialize(&mut &mut from.data.borrow_mut()[..])?;
    Ok(())
}

pub fn note_delete(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let from = next_account_info(account_info_iter)?;
    let to = next_account_info(account_info_iter)?;

    let mut state = try_from_slice_unchecked::<NotepadAccountState>(&to.data.borrow()).unwrap();

    if *from.key != state.pubkey {
        return Err(NotepadError::InvalidPubkey.into());
    }

    let from_lamports = from.lamports();
    **from.lamports.borrow_mut() = from_lamports.checked_add(to.lamports()).unwrap();
    **to.lamports.borrow_mut() = 0;
    let mut to_data = to.data.borrow_mut();
    to_data.fill(0);

    Ok(())
}
