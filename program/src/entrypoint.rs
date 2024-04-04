use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

use crate::processor;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    account: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("process entry:");
    processor::process_instruction(program_id, account, instruction_data)
}
