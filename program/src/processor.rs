use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh1::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{error::NotepadError, instruction::NotepadInstruction, state::NotepadAccountState};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ins = NotepadInstruction::unpack(instruction_data)?;
    match ins {
        NotepadInstruction::NoteCreate { contents, pubkey } => {
            return note_create(program_id, accounts, contents, pubkey)
        }
        NotepadInstruction::NoteUpdate { contents } => note_update(program_id, accounts, contents),
        NotepadInstruction::NoteDelete => note_delete(program_id, accounts),
    }
}

pub fn note_create(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    contents: String,
    pubkey: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let from = next_account_info(account_info_iter)?;
    let to = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if contents.len() > NotepadAccountState::CONTENTS_MAX_SIZE {
        msg!("Error: input contents exceeds max len!");
        return Err(NotepadError::InvalidContentsLen.into());
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(NotepadAccountState::MAX_SIZE);

    msg!("rent_lamports: {:?}", rent_lamports);
    msg!("program_id: {:?}", program_id);
    msg!("accounts: {:?}", accounts);

    invoke(
        &system_instruction::create_account(
            from.key,
            to.key,
            rent_lamports,
            NotepadAccountState::MAX_SIZE.try_into().unwrap(),
            program_id,
        ),
        &[from.clone(), to.clone(), system_program.clone()],
    )?;

    let mut state = try_from_slice_unchecked::<NotepadAccountState>(&to.data.borrow()).unwrap();
    msg!(
        "note create: 状态创建前 pubkey: {:?} contents: {:?}",
        state.pubkey,
        state.contents
    );

    state.pubkey = pubkey;
    state.contents = contents;
    state.serialize(&mut &mut to.data.borrow_mut()[..])?;

    let new_state = try_from_slice_unchecked::<NotepadAccountState>(&to.data.borrow()).unwrap();
    msg!(
        "note create: 状态创建后 pubkey: {:?} contents: {:?}",
        new_state.pubkey,
        new_state.contents
    );

    Ok(())
}

pub fn note_update(_: &Pubkey, accounts: &[AccountInfo], contents: String) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let from = next_account_info(account_info_iter)?;
    let to = next_account_info(account_info_iter)?;

    msg!("contents len: {}", contents.len());
    if contents.len() > NotepadAccountState::CONTENTS_MAX_SIZE {
        msg!("Error: input contents exceeds max len!");
        return Err(NotepadError::InvalidContentsLen.into());
    }

    let mut state = try_from_slice_unchecked::<NotepadAccountState>(&to.data.borrow()).unwrap();
    msg!(
        "note update: 状态更新前 pubkey: {:?} contents: {:?}",
        state.pubkey,
        state.contents
    );

    if *from.key != state.pubkey {
        msg!("Error: input invalid public key!");
        return Err(NotepadError::InvalidPubkey.into());
    }

    state.contents = contents;
    state.serialize(&mut &mut to.data.borrow_mut()[..])?;

    let new_state = try_from_slice_unchecked::<NotepadAccountState>(&to.data.borrow()).unwrap();
    msg!(
        "note update: 状态更新后 pubkey: {:?} contents: {:?}",
        new_state.pubkey,
        new_state.contents
    );

    Ok(())
}

pub fn note_delete(_: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let from = next_account_info(account_info_iter)?;
    let to = next_account_info(account_info_iter)?;

    let state = try_from_slice_unchecked::<NotepadAccountState>(&to.data.borrow()).unwrap();
    msg!(
        "note delete: 状态删除前 pubkey: {:?} contents: {:?}",
        state.pubkey,
        state.contents
    );

    if *from.key != state.pubkey {
        msg!("Error: input invalid public key!");
        return Err(NotepadError::InvalidPubkey.into());
    }

    let from_lamports = from.lamports();
    **from.lamports.borrow_mut() = from_lamports.checked_add(to.lamports()).unwrap();
    **to.lamports.borrow_mut() = 0;
    let mut to_data = to.data.borrow_mut();
    to_data.fill(0);

    let new_state = try_from_slice_unchecked::<NotepadAccountState>(&to_data).unwrap();
    msg!(
        "note delete: 状态删除后 pubkey: {:?} contents: {:?}",
        new_state.pubkey,
        new_state.contents
    );

    Ok(())
}
