
use crate::state::txn::TxnData;
use crate::instructions::create_user::create_user;
use crate::instructions::create_referrer::create_referrer;
use solana_program::sysvar::Sysvar;
use solana_program::program_error::ProgramError;

use solana_program::account_info::{next_account_info, AccountInfo};
use std::str::FromStr; // Needed for Pubkey::from_str
use solana_program::{
    program_error::ProgramError::InvalidInstructionData,
    sysvar,                           // Needed for sysvar::instructions::ID
   // system_program,                   // Used for account funding
clock::Clock,                     // If timestamps require validation
    msg,                              // Required for logging messages
    pubkey::Pubkey,
    entrypoint::ProgramResult,
    sysvar::instructions::{load_instruction_at_checked, ID as SYSVAR_INSTRUCTIONS_ID},
};
use borsh::BorshDeserialize; // Needed for TxnData::try_from_slice

pub fn post_txn(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let txn_data = TxnData::try_from_slice(instruction_data)?;
    let accounts_iter = &mut accounts.iter();
    /*
    okay the PDA's we need to pull in are:
    - PDA signed by my program with the merchant wallet seed 
    - PDA signed by my program with the user wallet seed contained
    - PDA signed by my program with the referrer wallet seed contained
    - PDA hash table of recent transactions between merchant and user. 

     */
    let sysvar = next_account_info(accounts_iter)?;
    if sysvar.key != &SYSVAR_INSTRUCTIONS_ID {
        return Err(ProgramError::InvalidAccountOwner)
    }
    /*
    
    */
    let merchant_sig_instruction = load_instruction_at_checked(0, sysvar)?;

    let merch_sig_inputs: &[u8] = &merchant_sig_instruction.data;
    
    
    /*
    okay so this is going to be a single ed25519 instruction that is used to verify all the signatures
    basically packing them all into one instruction and check_ed25519 is just verifying the inputs are correct 
    */
    let merchant_acc = next_account_info(accounts_iter)?;
    let ref_acc = next_account_info(accounts_iter)?;
    let user_acc = next_account_info(accounts_iter)?;
    check_ed25519_data(merch_sig_inputs, &merchant_acc.key.to_bytes(), merch_sig_inputs, &txn_data.merchant_signature);
    
    /*
    the respective pdas will be to temporarily track txn hashes for 10 minutes or however long 
    this is to prevent replays within the acceptable timeframe limit 
    theoretically a merchant could replay the process thousands of times to try and game the system. 

    This ensures they at least have to go through 
    */
    
    let merchant_pda = Pubkey::find_program_address( &[merchant_acc.key.as_ref()], program_id);
    let user_pda = Pubkey::find_program_address(&[user_acc.key.as_ref()], program_id);
    let ref_pda = Pubkey::find_program_address(&[ref_acc.key.as_ref()], program_id);

    let ed25519 = next_account_info(accounts_iter)?;
    
    let ed25519_program_id = Pubkey::from_str("Ed25519SigVerify111111111111111111111111111").unwrap();
    let sysvar_instruction_pubkey = sysvar::instructions::ID;
    if *ed25519.key != ed25519_program_id || *sysvar.key != sysvar_instruction_pubkey {
        return Err(ProgramError::InvalidAccountOwner)
    }
    let mut user_init_needed: bool = false;
    let mut ref_init_needed: bool = false;
    // these 2 might not exist yet, might be uninitializied, check if initializied 
    let ref_pda = next_account_info(accounts_iter)?;
    let user_pda = next_account_info(accounts_iter)?;
    if !ref_pda.data_is_empty() && *ref_pda.owner != *program_id && ref_pda.lamports() > 0 {
        // create the ref pda, ensure enough lamports
        ref_init_needed = true;
    }
    if !user_pda.data_is_empty() && *user_pda.owner != *program_id && user_pda.lamports() > 0 {
        // create the user pda, ensure enough lamports 
        user_init_needed = true;
    }
    let mut sig_accounts: Vec<AccountInfo> = Vec::new();
    sig_accounts.push(ed25519.clone());
    sig_accounts.push(sysvar.clone());
    sig_accounts.push(merchant_acc.clone());

    let current_time = Clock::get()?;
    if txn_data.referrer_timestamp < (txn_data.mu_timestamp - 86400) {
        // if referrer timestamp is within an hour 
        return Err(ProgramError::InvalidInstructionData)
    } else if txn_data.referrer_timestamp > (txn_data.mu_timestamp -4) {
        /* this is trying to find some erratic log timestamps
         my thinking here is any real user would take at least 
         4 seconds from the time they are referred to the time 
         they purchase the product. Any shorter and its highly
         likely bot activity*/
        return Err(ProgramError::InvalidInstructionData)
    } else if txn_data.mu_timestamp > current_time.unix_timestamp - 50 || txn_data.mu_timestamp < current_time.unix_timestamp + 1 {
        return Err(ProgramError::InvalidInstructionData)
    }

    // constructiong the data the user/merchant signed on 
    let mut combined_data = Vec::new();
    combined_data.extend_from_slice(&txn_data.referrer_timestamp.to_le_bytes());
    combined_data.extend_from_slice(user_acc.key.as_ref());
    combined_data.extend_from_slice(merchant_acc.key.as_ref());
    combined_data.extend_from_slice(&txn_data.mu_timestamp.to_le_bytes());
    combined_data.extend_from_slice(&txn_data.cost.to_le_bytes());
    combined_data.extend_from_slice(&txn_data.global_txn_id);


    let rewards = txn_data.cost;
 
    

    if user_init_needed {
        let accs : [AccountInfo; 2]= [merchant_acc.clone(), ref_acc.clone()];
        create_user(user_pda.key, &rewards.to_le_bytes(), &accs)?;
    }
    
    if ref_init_needed {
        let ref_accs : [AccountInfo; 2]= [merchant_acc.clone(), ref_acc.clone()];
 
        create_referrer(ref_pda.key, &rewards.to_le_bytes(), &ref_accs)?;
    }
    // the msg macro is just logging success/fail, the rest of the data is just call data
    msg!("success");
    Ok(())
}

pub fn check_ed25519_data(
    data: &[u8], merchant_pubkey: &[u8], user_pubkey: &[u8], 
    referrer_pubkey: &[u8], merch_msg: &[u8], user_msg: &[u8], 
    ref_msg: &[u8],merch_sig: &[u8], user_sig: &[u8], ref_sig: &[u8]) -> ProgramResult {
    // According to this layout used by the Ed25519Program
    // https://github.com/solana-labs/solana-web3.js/blob/master/src/ed25519-program.ts#L33
        // Expected values

    let exp_public_key_offset:      u16 = 16; // 2*u8 + 7*u16
    let exp_signature_offset:       u16 = exp_public_key_offset + merchant_pubkey.len() as u16;
    let exp_message_data_offset:    u16 = exp_signature_offset + sig.len() as u16;
    let exp_num_signatures:          u8 = 3;
    let exp_message_data_size:      u16 = msg.len().try_into().unwrap();
    
    // "Deserializing" byte slices

    // this describes everything about instruction
    let num_signatures                  = &[data[0]];        // Byte  0
    let padding                         = &[data[1]];        // Byte  1
    let signature_offset                = &data[2..=3];      // Bytes 2,3
    let signature_instruction_index     = &data[4..=5];      // Bytes 4,5
    let public_key_offset               = &data[6..=7];      // Bytes 6,7
    let public_key_instruction_index    = &data[8..=9];      // Bytes 8,9
    let message_data_offset             = &data[10..=11];    // Bytes 10,11
    let message_data_size               = &data[12..=13];    // Bytes 12,13
    let message_instruction_index       = &data[14..=15];    // Bytes 14,15
        /*
    modifying this from original, we have 3 sigs so we need to do this 3 times and check the msg along with the pubkey
    this requires multiple sigs from data 
    */ 
    for i in 1..3{
        let data_pubkey                     = &data[16..16+32];  // Bytes 16..16+32
        let data_sig                        = &data[48..48+64];  // Bytes 48..48+64
        let data_msg                        = &data[112..];      // Bytes 112..end


        // Header and Arg Checks

        // Header


        if padding                         != &[0]                                     ||
        signature_offset                != &exp_signature_offset.to_le_bytes()      ||
        signature_instruction_index     != &u16::MAX.to_le_bytes()                  ||
        public_key_offset               != &exp_public_key_offset.to_le_bytes()     ||
        public_key_instruction_index    != &u16::MAX.to_le_bytes()                  ||
        message_data_offset             != &exp_message_data_offset.to_le_bytes()   ||
        message_data_size               != &exp_message_data_size.to_le_bytes()     ||
        message_instruction_index       != &u16::MAX.to_le_bytes()  
        {
            return Err(InvalidInstructionData);
        }
        match i {
            1 => {
                if data_pubkey != merchant_pubkey || data_msg != merch_msg || data_sig != merch_sig {
                    return Err(InvalidInstructionData);
                }
            },
            2 => {
                if data_pubkey != user_pubkey || data_msg != user_msg || data_sig != user_sig{

                }
            }
        }
        if  data_pubkey != pubkey   ||
        data_msg    != msg      ||
        data_sig    != sig
    {
        return Err(InvalidInstructionData);
    }
    }

    // Arguments

   
     
    Ok(())
}
