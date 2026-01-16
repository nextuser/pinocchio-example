use pinocchio::{
    AccountView, 
    error::ProgramError,
    ProgramResult,
    sysvars::{rent::Rent,Sysvar}
};
use pinocchio_system::instructions::{Transfer as SystemTransfer};
use pinocchio_log::log;

use crate::parse_amount;
pub struct Deposit<'a> {
    pub owner : &'a AccountView,
    pub vault : &'a AccountView,
    pub amount : u64,
}

impl<'a>  Deposit<'a>{
    pub const DESCRIMINATOR: &'a u8 = &0;

    pub fn process(self) -> ProgramResult
    {
        let Deposit {
            owner,
            vault,
            amount
        } = self;
        crate::encure_pda_created(vault, owner)?;
        SystemTransfer{from:owner,to : vault,lamports:amount}.invoke()?;
        log!("deposit {} to vault success",amount);
        Ok(())
    }
}



type ArgType<'a>  = (&'a [u8], &'a [AccountView]);

impl<'a> TryFrom< ArgType<'a> > for Deposit<'a > {
    type Error = ProgramError;
    fn try_from(value : ArgType<'a>) ->Result<Self,Self::Error> {
        let (data,accounts) = value;
        if accounts.len() < 2 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        let owner = &accounts[0];
        let vault = &accounts[1];
        let amount = parse_amount(data)?;
        Ok(Self{
            owner,
            vault,
            amount
        })
    }
}



pub struct Withdraw<'a> {
    pub owner : &'a AccountView,
    pub vault : &'a AccountView,
}
impl<'a> Withdraw<'a> {
    pub const DESCRIMINATOR: &'a u8 = &1;

    pub fn process(self) -> ProgramResult
    {
        let Withdraw {
            owner,
            vault
        } = self;

        if !owner.is_signer(){
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !owner.owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);

        }

        let (expected_pda, _bump) = crate::derive_vault_pda(owner.address());
        if vault.address() != & expected_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        
        let rent_fee = Rent::get()?.try_minimum_balance(vault.data_len())?;
        let withdraw_amount = vault.lamports().checked_sub(rent_fee).ok_or(ProgramError::InsufficientFunds)?;

        let _ = vault.lamports().checked_sub(withdraw_amount).ok_or(ProgramError::InsufficientFunds);

        let _ = owner.lamports().checked_add(withdraw_amount).ok_or(ProgramError::ArithmeticOverflow)?;

        SystemTransfer{from:vault,to : owner,lamports:vault.lamports()}.invoke()?;
        log!("withdraw {} from vault success",vault.lamports());
        Ok(())
    }
}


impl <'a> TryFrom<ArgType<'a>> for Withdraw<'a>{
    type Error = ProgramError;
    fn try_from(value : ArgType<'a>) ->Result<Self,Self::Error> {
        let (_,accounts) = value;
        if accounts.len() < 2 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        let owner = &accounts[0];
        let vault = &accounts[1];
        Ok(Self{
            owner,
            vault
        })
    }
}