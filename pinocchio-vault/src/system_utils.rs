use pinocchio::error::ProgramError;
use pinocchio::AccountView;
use pinocchio::Address;
use pinocchio_system::{
    create_account_with_minimum_balance_signed
};
use pinocchio::sysvars::{Sysvar,rent::Rent};
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
    
    //const ACCOUNT_DISCRIMINATOR_SIZE: usize = 8;

    let space =  0;

    //let seeds = [b"vault",owner.address().as_ref()];
    //let (expected_pda, bump) = Address::find_program_address(&seeds, &crate::ID);
    let (expected_pda ,bump,seeds) = crate::utils::derive_vault_pda(owner.address());
    if pda.address() != &expected_pda {
        return Err(ProgramError::InvalidSeeds);
    }
    let tail = [bump];

    
    let signed_seeds = [
        Seed::from(seeds[0]),
        Seed::from(seeds[1]),
        Seed::from(&tail)
    ];

    let signer = Signer::from(&signed_seeds);

    create_account_with_minimum_balance_signed(
        pda,
        space,
        &crate::ID,//owner is program
        owner,
        None,
        &[signer]
    )?;
    log!("create pda, spacelen {}, fee {}",space, Rent::get()?.try_minimum_balance(space)?);
    Ok(())
}