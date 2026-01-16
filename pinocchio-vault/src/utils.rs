use pinocchio::error::ProgramError;
use pinocchio::{
    Address,
};
pub fn parse_amount(data : &[u8] ) -> Result<u64, ProgramError>{
    let expected_len = core::mem::size_of::<u64>();
    if data.len() != expected_len {
        return Err(ProgramError::MaxInstructionTraceLengthExceeded);
    }
    let amount = u64::from_le_bytes(data.try_into().unwrap());
    if amount == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }
    return Ok(amount);
}


pub fn derive_vault_pda(owner : &Address) -> (Address,u8){
    let seeds = [b"valult",owner.as_ref()];
    let (pda, bump) = Address::find_program_address(&seeds, &crate::ID);
    (pda,bump)
}