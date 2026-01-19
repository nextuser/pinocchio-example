use litesvm::LiteSVM;
use solana_program::example_mocks::{solana_account::Account, solana_sdk::{address_lookup_table::instruction, system_program}};
#[cfg(test)]
use solana_sdk::signer::keypair;
use solana_system_interface;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    instruction::AccountMeta,
    instruction::Instruction,
    message::Message,
    transaction::Transaction,
};
use solana_address::Address;
const DEPOSIT : u8 = 0;
const WITHDRAW : u8 = 1;
use std::str::FromStr;

use std::env;
use std::path::Path;
use solana_keypair::read_keypair_file;

// use solana_sdk::{
//     program,
//     account::ReadableAccount,
//     instruction::{Instruction,AccountMeta},
//     address_lookup_table::Program,
//     message::Message,
//     signature::{Keypair, Signer},
//     system::system_program,
//     transaction::Transaction,
//     pubkey::Pubkey,
// };

fn deploy_program(svm :&mut LiteSVM,_publisher:&Keypair,program_id : &Address) {
    //use solana_sdk::{message::Instruction, signer::keypair};

    // let publisher = Keypair::new();
    // let lamports = 1000_000_000;
    // svm.airdrop(&publisher.pubkey(), lamports).expect("airdrop failed");

    //../pinocchio-vault/target/deploy/pinocchio_vault.so
    let curr_dir = env::current_dir().unwrap();
    let file_path = curr_dir.join("../pinocchio-vault/target/deploy/pinocchio_vault.so");
    println!("load path {}", file_path.display());
    //let path = "../pinocchio-vault/target/deploy/pinocchio_vault.so";
    svm.add_program_from_file(program_id, file_path).expect("failed to deploy");    
    
}

fn get_airdrop_keypair(svm :&mut LiteSVM) -> Keypair {
    let keypair = Keypair::new();
    let airdrop_address = keypair.pubkey();
    let airdrop_lamports = 1000_000_000;
    svm.airdrop(&airdrop_address, airdrop_lamports).expect("airdrop failed");
    keypair
}


fn call_process(svm :&mut LiteSVM,program_id: &Address, caller : &Keypair, instruction_data: &[u8], name : &str) {

    // let caller = Keypair::new();
    // let lamports = 1000_000_000;
    // svm.airdrop(&caller.pubkey(), lamports).expect("airdrop failed");
    println!("start of {}",name);
    let caller_addr = caller.pubkey();
    let seeds = [b"vault", caller_addr.as_ref()];
    let (pda,_bump) = Pubkey::find_program_address(&seeds, program_id);
    let accounts = vec![
        AccountMeta::new(caller.pubkey(), true),
        AccountMeta::new(pda, false),
        AccountMeta::new(solana_program::sysvar::rent::id(), false),
        AccountMeta::new(solana_system_interface::program::ID, false),
    ];
    let ins = Instruction::new_with_bytes(* program_id, instruction_data, accounts);
    let message = Message::new(
        &[ins], //instructions
        Some(&caller.pubkey()),//payer
        );
    let tx = Transaction::new(
        &[&caller],
        message,
        svm.latest_blockhash(),
    );
    println!("before send {} ", name);
    let result = svm.send_transaction(tx);
    println!("after send result={:#?}",result);
    if !result.is_ok(){
        panic!("transaction failed");
        println!("reuslt failed: send transaction error {:?}", result.unwrap_err());
    } else {
        println!("result ok logs:   {:?}",&result.unwrap().logs);
    }

    println!("end of {}",name);



}

fn call_withdraw(svm :&mut LiteSVM, program_id : &Address, caller : &Keypair){
    let data = [WITHDRAW];
    call_process(svm, program_id, caller, &data, "withdraw");
}

fn call_deposit(svm :&mut LiteSVM, program_id : &Address, caller : &Keypair, amount : u64){
    let mut v = Vec::<u8>::new();
    v.push(DEPOSIT);
    let bytes = amount.to_le_bytes();
    v.extend_from_slice(&bytes);
    call_process( svm, program_id, &caller, &v, "deposit");
}

#[test]
pub fn test_process(){
    let mut svm = LiteSVM::new();
    let publisher = get_airdrop_keypair(&mut svm);
    let caller = get_airdrop_keypair(&mut svm);
    let curr_dir = env::current_dir().unwrap();
    let file_path = curr_dir.join("../pinocchio-vault/target/deploy/pinocchio_vault-keypair.json");
    //let file = std::fs::File::open(file_path).expect("file not found");
    let program_key = read_keypair_file(file_path).unwrap();
    let program_id = program_key.pubkey();
    deploy_program(&mut svm,  &publisher, &program_id);
    println!("deployed program id: {}",program_id);
    call_deposit(&mut svm , &program_id, &caller, 5000_000);
    call_withdraw(&mut svm, &program_id, &caller );

}

pub fn main() {
    println!("Hello, world!");
}

#[test]
fn test_path(){
    let curr_dir = env::current_dir().unwrap();
    let file_path = curr_dir.join("../pinocchio-vault/target/deploy/pinocchio_vault.so");
    println!("current dir: {}", file_path.display());
}
