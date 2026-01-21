import {FailedTransactionMetadata, LiteSVM} from 'litesvm';
import {Keypair, PublicKey, Transaction,TransactionInstruction
, SystemProgram, SYSVAR_RENT_PUBKEY,

} from '@solana/web3.js'
import {describe , it } from 'node:test';
import assert from 'node:assert/strict';
import path from 'path';
import fs from 'fs';
import {fileURLToPath} from 'url';
import type { Key } from 'readline';
let __dirname = path.dirname( fileURLToPath(import.meta.url));
console.log("__dirname:",__dirname);
let keyfile = path.resolve(__dirname, '../../target/deploy/storage-keypair.json');
console.log("read file:" ,keyfile);
let key_arr = JSON.parse(fs.readFileSync(keyfile, 'utf-8'));
const program_keypair = Keypair.fromSecretKey(new Uint8Array(key_arr));
const program_id = program_keypair.publicKey;

function  getAirdropedKeyPair (svm : LiteSVM  ) : Keypair  {
    const keypair = Keypair.generate();
    svm.airdrop(keypair.publicKey, 1000000000n);
    
    return keypair;
}

async function updatePda(svm : LiteSVM,alice : Keypair,pda : PublicKey,data :Buffer)  {
 const tx = new Transaction();
       const call_ins = new TransactionInstruction(
         {
              programId: program_id,
              keys: [
                  {
                        pubkey: alice.publicKey,
                        isSigner: true,
                        isWritable: true
                  },
                  {
                        pubkey: pda,
                        isSigner: false,
                        isWritable: true
                  },
                  {
                        pubkey: SYSVAR_RENT_PUBKEY,
                        isSigner: false,
                        isWritable: false
                  },
                  {
                        pubkey: SystemProgram.programId,
                        isSigner: false,
                        isWritable: false
                  }                 
              ],
              data: data
         });
     tx.add(call_ins);
     tx.recentBlockhash =  svm.latestBlockhash();
     tx.sign(alice);
     let result = await svm.sendTransaction(tx);
     if('err' in result) {
      console.log("transaction error:",result.toString())
      //console.log("transaction :",result.toString())
      console.log("log:",result.meta().prettyLogs());
      
     } else if('prettyLogs' in result) {
        console.log("transaction logs:",result.prettyLogs())
     }
     
     console.log("pda:",pda.toBase58())
     const balance = svm.getBalance(pda);
     let rent_lamports = svm.getRent().minimumBalance(BigInt(data.length));
     console.log("balance:",balance);
     assert.strictEqual(balance,rent_lamports);
}
describe('test', () => {

  const svm = new LiteSVM();
  console.log("pubkey:",program_id.toBase58()) 

  const program_file = path.resolve(__dirname, '../../target/deploy/storage.so');
  console.log("program file:",program_file);
  svm.addProgramFromFile(program_id,program_file);
  console.log("load program:",program_id.toBase58());

  const alice = getAirdropedKeyPair(svm);
  const [pda,bump] = PublicKey.findProgramAddressSync([Buffer.from("storage"),alice.publicKey.toBuffer()], program_id);
  console.log("program id",program_id.toBase58())


  it('initialize', async () => {
    let data :Buffer = Buffer.from("abcdef");
    updatePda(svm,alice,pda,data);
    //    const tx = new Transaction();
    //    const call_ins = new TransactionInstruction(
    //      {
    //           programId: program_id,
    //           keys: [
    //               {
    //                     pubkey: alice.publicKey,
    //                     isSigner: true,
    //                     isWritable: true
    //               },
    //               {
    //                     pubkey: pda,
    //                     isSigner: false,
    //                     isWritable: true
    //               },
    //               {
    //                     pubkey: SYSVAR_RENT_PUBKEY,
    //                     isSigner: false,
    //                     isWritable: false
    //               },
    //               {
    //                     pubkey: SystemProgram.programId,
    //                     isSigner: false,
    //                     isWritable: false
    //               }                 
    //           ],
    //           data: data
    //      });
    //  tx.add(call_ins);
    //  tx.recentBlockhash =  svm.latestBlockhash();
    //  tx.sign(alice);
    //  let result = await svm.sendTransaction(tx);
    //  if('err' in result) {
    //   console.log("transaction error:",result.toString())
    //   //console.log("transaction :",result.toString())
    //   console.log("log:",result.meta().prettyLogs());
      
    //  } else if('prettyLogs' in result) {
    //     console.log("transaction logs:",result.prettyLogs())
    //  }
     
    //  console.log("pda:",pda.toBase58())
    //  const balance = svm.getBalance(pda);
    //  let rent_lamports = svm.getRent().minimumBalance(BigInt(data.length));
    //  console.log("balance:",balance);
    //  assert.strictEqual(balance,rent_lamports);


  });//end it


  it('update data', async () => {
       let data :Buffer = Buffer.from("ghijklmnop");
       await updatePda(svm,alice,pda,data);



  });//end it
});//describe