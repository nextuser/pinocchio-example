use pinocchio::{
    ProgramResult, account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::{Pubkey, find_program_address}, 
    sysvars::rent::Rent,
};
use pinocchio_system::{
    self,
    instructions::{
        Transfer,
    },
    

};


const SEEDS_PREFIX: &[u8] = b"storage";
pub fn  derive_pda(owner : &Pubkey) -> (Pubkey,u8){
    return find_program_address(&[SEEDS_PREFIX,owner.as_ref()], &crate::ID);

}
pub fn ensure_create(authority : &AccountInfo, pda : &AccountInfo,rent_account : &AccountInfo, data_len : usize,bump : u8) -> ProgramResult{
    // pda is already initialized
    if pda.lamports() != 0{
        return Ok(());
    }
    let bump_arr = [bump];
    let seeds = [
        Seed::from(SEEDS_PREFIX),
        Seed::from(authority.key().as_ref()),
        Seed::from(&bump_arr),
    ];
    let signer = Signer::from(&seeds);
        // 使用 PDA 的种子创建 Signer，即使 PDA 尚未创建也可以


    pinocchio_system::create_account_with_minimum_balance_signed(
        pda,
        data_len,//space
        &crate::ID,//owner
        authority,//payer
        Some(rent_account),//rent sysvar
        &[signer]
    ) 
    
}

pub fn fill_data( pda : &AccountInfo, rent_account : &AccountInfo,data : &[u8], authority : &AccountInfo) -> ProgramResult{

    let nee_fee = Rent::from_account_info(rent_account)?.minimum_balance(data.len());
    let lamports = pda.lamports();
    if lamports < nee_fee {
        let diff = nee_fee.checked_sub(lamports).ok_or(ProgramError::ArithmeticOverflow)?;
        let transfer = Transfer{
            from : authority,
            to : pda,
            lamports : diff,
        };
        transfer.invoke()?;
    } else if lamports > nee_fee {
        let diff = lamports.checked_sub(nee_fee).expect("lamports must be greater than nee_fee");
        let mut pda_lamports = pda.try_borrow_mut_lamports()?;
        let mut payer_lamports = authority.try_borrow_mut_lamports()?;
        * pda_lamports = pda_lamports.checked_sub(diff).ok_or(ProgramError::InsufficientFunds)?;
        * payer_lamports = payer_lamports.checked_add(diff).ok_or(ProgramError::ArithmeticOverflow)?;

    };
    pda.resize(data.len())?;
    pda.try_borrow_mut_data()?.copy_from_slice(data);   
    Ok(())
}