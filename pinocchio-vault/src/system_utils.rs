use pinocchio::error::ProgramError;
use pinocchio::AccountView;
use pinocchio::Address;
use pinocchio_system::{
    create_account_with_minimum_balance_signed
};
use pinocchio::cpi::{Seed,Signer};
use pinocchio_log::log;
pub fn encure_pda_created( 
    pda: &AccountView,
    owner : &AccountView)
    -> Result<(), ProgramError>
{
    if pda.lamports() != 0 {
        return Ok(());
    }
    
    const ACCOUNT_DISCRIMINATOR_SIZE: usize = 8;

    let space =  ACCOUNT_DISCRIMINATOR_SIZE + core::mem::size_of::<u64>();

    let seeds = [b"vault",owner.address().as_ref()];
    let (expected_pda, bump) = Address::find_program_address(&seeds, &crate::ID);
    if pda.address() != &expected_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    
    let signed_seeds =[
        Seed::from(seeds[0]),
        Seed::from(seeds[1]),
        Seed::from(core::slice::from_ref(&bump)), //todo ???
    ];
    let signer = Signer::from(&signed_seeds);

    create_account_with_minimum_balance_signed(
        pda,
        space,
        owner.address()    ,
        owner,
        None,
        &[signer]
    )?;
    Ok(())
}