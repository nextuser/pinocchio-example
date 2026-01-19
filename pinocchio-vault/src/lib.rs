mod instruction;
use instruction::*;
mod utils;
use utils::parse_amount;
use utils::derive_vault_pda;
mod system_utils;
use system_utils::encure_pda_created;

use pinocchio::{

    AccountView, Address, ProgramResult, 
    address::declare_id, 
    default_allocator, 
    default_panic_handler, 
    error::ProgramError, 
    program_entrypoint
};
use pinocchio_log::log;

program_entrypoint!(process_instruction);
declare_id!("3gN1GKoQFgqBDZGaBxTwKdLrD4sYdWRbNqvgM2yiwduX");
default_allocator!();
default_panic_handler!();


fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Deposit::DESCRIMINATOR,tail_data))=> {
            Deposit::try_from((tail_data,accounts))?.process()

        },
        Some((Withdraw::DESCRIMINATOR,tail_data)) => {
            Withdraw::try_from((tail_data,accounts))?.process()
        },
        _ => {
            log!("Instruction: Unknown");
            Err(ProgramError::InvalidInstructionData)
        }
    }
}