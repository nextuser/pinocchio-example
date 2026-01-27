use litesvm::LiteSVM;
// use solana_program::example_mocks::{solana_account::Account, solana_sdk::{address_lookup_table::instruction, system_program}};
use solana_system_interface;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    sysvar::rent::Rent,
    instruction::AccountMeta,
    instruction::Instruction,

    message::Message,
    transaction::Transaction,
};
use solana_address::Address;
const DEPOSIT : u8 = 0;
const WITHDRAW : u8 = 1;
use std::env;
//use std::path::Path;
use five8;
use core::str::from_utf8;
fn encode_address(address: &[u8]) -> String {
    let mut buffer = [0u8; 44];
    let address = address.as_ref().try_into().unwrap();
    let count = five8::encode_32(address, &mut buffer);
    from_utf8(&buffer[..count as usize]).unwrap().to_string()
}

const PROGRAM_ID : Address = Address::from_str_const("22222222222222222222222222222222222222222222");


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

fn deploy_program(svm :&mut LiteSVM,_publisher:&Keypair,program_id : &Address) -> usize  {
    //use solana_sdk::{message::Instruction, signer::keypair};

    // let publisher = Keypair::new();
    // let lamports = 1000_000_000;
    // svm.airdrop(&publisher.pubkey(), lamports).expect("airdrop failed");

    //../pinocchio-vault/target/deploy/pinocchio_vault.so
    let curr_dir = env::current_dir().unwrap();
    let file_path = curr_dir.join("../target/deploy/pinocchio_vault.so");
    println!("load path {}", file_path.display());
    let bytes = std::fs::read(file_path).expect("read file failed");
    svm.add_program(program_id, &bytes).expect("add program failed");
    //let path = "../pinocchio-vault/target/deploy/pinocchio_vault.so";
    return bytes.len();
    
}

fn get_airdrop_keypair(svm :&mut LiteSVM) -> Keypair {
    let keypair = Keypair::new();
    let airdrop_address = keypair.pubkey();
    let airdrop_lamports = 1000_000_000;
    svm.airdrop(&airdrop_address, airdrop_lamports).expect("airdrop failed");
    keypair
}


fn call_process(svm :&mut LiteSVM,program_id: &Address, caller : &Keypair, payer : &Keypair, instruction_data: &[u8], name : &str) {

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
        AccountMeta::new(solana_program::sysvar::rent::id(), false), //如果不传递，会出现miss account错误
        AccountMeta::new(solana_system_interface::program::ID, false),//如果不传递，会出现miss account错误
        AccountMeta::new(payer.pubkey(), true),
    ];
    
    println!("call process name={} pda: {}",name, pda.to_string());
    let account_info =  svm.get_account( & pda);
    let owner : &str = if account_info.is_none() { ""  } else {  &encode_address(account_info.unwrap().owner.as_ref())};
    println!("pda.owner: {}", owner);
    let ins = Instruction::new_with_bytes(* program_id, instruction_data, accounts);
    let message = Message::new(
        &[ins], //instructions
        Some(&payer.pubkey()),//payer
        );
    let tx = Transaction::new(
        &[&payer,&caller],
        message,
        svm.latest_blockhash(),
    );
    println!("before send {} ", name);
    let result = svm.send_transaction(tx);
    println!("after send result={:#?}",result);
    if !result.is_ok(){
        
        println!("reuslt failed: send transaction error {:#?}", result.unwrap_err());
        panic!("transaction failed");
    } else {
        println!("result ok logs:   {:#?}",&result.unwrap().logs);
    }

    println!("end of {}",name);



}

fn call_withdraw(svm :&mut LiteSVM, program_id : &Address, caller : &Keypair, payer : &Keypair){
    let data = [WITHDRAW];
    call_process(svm, program_id, caller,payer, &data, "withdraw");
}

fn call_deposit(svm :&mut LiteSVM, program_id : &Address, caller : &Keypair, payer : &Keypair, amount : u64){
    let mut v = Vec::<u8>::new();
    v.push(DEPOSIT);
    let bytes = amount.to_le_bytes();
    v.extend_from_slice(&bytes);
    call_process( svm, program_id, &caller, payer, &v,  "deposit");
}

// fn get_program_id() -> Address {
//     let file_path = curr_dir.join("../target/deploy/pinocchio_vault-keypair.json");
//     let program_key = read_keypair_file(file_path).unwrap();
//     let program_id = program_key.pubkey();
//     return program_id;
// }
#[test]
pub fn test_process(){
    let mut svm = LiteSVM::new();
    let publisher = get_airdrop_keypair(&mut svm);
    let caller = get_airdrop_keypair(&mut svm);
    let payer = get_airdrop_keypair(&mut svm);
    deploy_program(&mut svm,  &publisher,&PROGRAM_ID);
    println!("deployed program id: {}",PROGRAM_ID);
    let caller_addr = caller.pubkey();
    let rent = svm.get_sysvar::<Rent>();
    let space = 0;
    let rent_fee = rent.minimum_balance(space);
    println!("rent_fee: {},for space : {}", rent_fee, space );
    let init_balance = svm.get_balance(&caller_addr).unwrap();
    println!("init_balance: {}", init_balance);
    call_deposit(&mut svm , &PROGRAM_ID, &caller,&payer, 5000_000);
    let balance_after_deposit = svm.get_balance(&caller_addr).unwrap();
    assert_eq!(balance_after_deposit, init_balance - 5000_000 - rent_fee);
    call_withdraw(&mut svm, &PROGRAM_ID, &caller,&payer );
    let new_balance = svm.get_balance(&caller_addr).unwrap();
    assert_eq!(init_balance - rent_fee, new_balance);

}

pub fn main() {
    println!("Hello, world!");
}

#[test]
fn test_path(){
    let curr_dir = env::current_dir().unwrap();
    let file_path = curr_dir.join("../target/deploy/pinocchio_vault.so");
    println!("current dir: {}", file_path.display());
}
