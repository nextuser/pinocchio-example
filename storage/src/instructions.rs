use pinocchio::{
    account_info::AccountInfo,
    pubkey::{pubkey_eq},
    program_error::ProgramError,
    ProgramResult,
    sysvars::rent::RENT_ID,
    
};

pub fn store_data(accounts : &[AccountInfo],data : &[u8] ) -> ProgramResult{ 
   if accounts.len() < 3 {
        return Err(ProgramError::InvalidAccountData);

   }
   let authority = &accounts[0];
   let pda = &accounts[1];
   let rent_account = & accounts[2];
   if !pubkey_eq(&RENT_ID, rent_account.key()) {
       return Err(ProgramError::UnsupportedSysvar);
   }
   let (expected_pda,bump) = crate::util::derive_pda(authority.key());
   if !pubkey_eq(&expected_pda , pda.key()) {
       return Err(ProgramError::InvalidSeeds);
   }


   crate::util::ensure_create(authority, pda,rent_account,data.len(),bump).expect("account must be created");
   // 没有创建的时候，owner 是缺省值，不会是 crate::ID
   if !pda.is_owned_by(&crate::ID) {
        return Err(ProgramError::InvalidAccountOwner);
   }
  


  crate::util::fill_data(pda, rent_account, data, authority)

}

