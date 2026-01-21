//#![no_std]

mod data;
mod instructions;
mod util;
use instructions::store_data;
use pinocchio::{
    ProgramResult, account_info::AccountInfo, msg, pubkey::Pubkey,
    default_panic_handler,
    program_entrypoint, no_allocator,

    
};
use pinocchio_pubkey::declare_id;


program_entrypoint!(process_instruction);
default_panic_handler!();
//nostd_panic_handler!();
no_allocator!();

declare_id!("CKyJT8Eg76c62xh3uHztNMC8AcKAHVrAopw9vBFhvpQV");

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    store_data(accounts, instruction_data)?;
    msg!("storage processed");
   Ok(())
}


