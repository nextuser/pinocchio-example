use pinocchio::{
    AccountView, 
    error::ProgramError,
    ProgramResult,
    sysvars::{rent::Rent,Sysvar}
};
use pinocchio_system::instructions::{Transfer as SystemTransfer};
use pinocchio_log::{log,logger};
use solana_address::Address;

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

use five8;
use core::str::from_utf8;
fn encode_address(address: &[u8]) -> String {
    let mut buffer = [0u8; 44];
    let address = address.as_ref().try_into().unwrap();
    let count = five8::encode_32(address, &mut buffer);
    from_utf8(&buffer[..count as usize]).unwrap().to_string()
}



pub struct Withdraw<'a> {
    pub authority : &'a AccountView,
    pub vault : &'a AccountView,
}
impl<'a> Withdraw<'a> {
    pub const DESCRIMINATOR: &'a u8 = &1;

    pub fn process(self) -> ProgramResult
    {
        let Withdraw {
            authority,
            vault
        } = self;

        if !authority.is_signer(){
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !vault.owned_by(&crate::ID) {
        unsafe{
            let owner = encode_address(vault.owner().as_ref());
            let expected_owner = encode_address(crate::ID.as_ref());
            let vault_str = encode_address(vault.address().as_ref());
            log!("vault owner is {} ,expect owner {},vaultId {}",
                    owner.as_str(),
                    expected_owner.as_str(),
                    vault_str.as_str());
            }
            //return Ok(())
            return Err(ProgramError::InvalidAccountOwner);

        }

        let (expected_pda, _bump, _seeds) = crate::derive_vault_pda(authority.address());
        if vault.address() != & expected_pda {
            return Err(ProgramError::InvalidAccountData);
        }
       
        
        let rent_fee = Rent::get()?.try_minimum_balance(vault.data_len())?;
        let withdraw_amount = vault.lamports().checked_sub(rent_fee).ok_or(ProgramError::InsufficientFunds)?;

        vault.set_lamports(vault.lamports().checked_sub(withdraw_amount).ok_or(ProgramError::InsufficientFunds)?);

        authority.set_lamports( authority.lamports().checked_add(withdraw_amount).ok_or(ProgramError::ArithmeticOverflow)?);

        //SystemTransfer{from:vault,to : authority,lamports:vault.lamports()}.invoke_signed(signers);
        log!("withdraw {} from vault success,left rentfee {}, for space len : {}",vault.lamports(), rent_fee, vault.data_len());
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

        let authority = &accounts[0];
        let vault = &accounts[1];
        Ok(Self{
            authority,
            vault
        })
    }
}