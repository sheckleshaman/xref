
use crate::state::txn::{TxnData,Ts};
use crate::state::ad::Ad;
use crate::state::user::User;
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
    hash::hash
};
use borsh::{BorshDeserialize, BorshSerialize}; // Needed for TxnData::try_from_slice

pub fn post_txn(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let txn_data = TxnData::try_from_slice(instruction_data)?;
    let accounts_iter = &mut accounts.iter();
    /*
    okay the PDA' ordering is:
    1. sysvar account
    2. merchant account
    3. referrer account
    4. user account
    5. ad account
    6. merchant pda 
    7. user pda 
    8. referrer pda 
    9. global_addr pda 
    10. local_addr pda 


     */
    let sysvar = next_account_info(accounts_iter)?;
    if sysvar.key != &SYSVAR_INSTRUCTIONS_ID {
        return Err(ProgramError::InvalidAccountOwner)
    }
    /*
    okay so this is going to be a single ed25519 instruction that is used to verify all the signatures
    basically packing them all into one instruction and check_ed25519 is just verifying the inputs are correct 
    */
    let merchant_acc = next_account_info(accounts_iter)?;
    let ref_acc = next_account_info(accounts_iter)?;
    let user_acc = next_account_info(accounts_iter)?;
    let ad_acc = next_account_info(accounts_iter)?;
        /*
    the respective pdas will be to temporarily track txn hashes for 10 minutes or however long 
    this is to prevent replays within the acceptable timeframe limit 
    theoretically a merchant could replay the process thousands of times to try and game the system. 

    This ensures they at least have to go through 
    */
    
    let merchant_pda_ = next_account_info(accounts_iter)?;
    let (merchant_pda, _bump) = Pubkey::find_program_address( &[merchant_acc.key.as_ref()], program_id);
    checkpdalamports(merchant_pda_, &merchant_pda, program_id, false)?;
    let user_pda_ = next_account_info(accounts_iter)?;
    let (user_pda, _bump) = Pubkey::find_program_address(&[user_acc.key.as_ref()], program_id);
    checkpdalamports(user_pda_, &user_pda, program_id, false )?;
    let ref_pda_ = next_account_info(accounts_iter)?;
    let (ref_pda, _bump) = Pubkey::find_program_address(&[ref_acc.key.as_ref()], program_id);
    checkpdalamports(ref_pda_, &ref_pda, program_id, false)?;
    let global_msg_ = [merchant_acc.key.as_ref(),
    user_acc.key.as_ref(),
    ref_acc.key.as_ref(),
    ad_acc.key.as_ref(),
];
    let global_txn_pda_seed = hash(&global_msg_.concat()).to_bytes();
    let (global_addr, _bump) = Pubkey::find_program_address(&[&global_txn_pda_seed], program_id);
    let global_addr_ = next_account_info(accounts_iter)?;
    
    checkpdalamports(global_addr_, &global_addr, program_id, true)?;

    let local_msg_ =[
        merchant_acc.key.as_ref(),
        user_acc.key.as_ref(),
        ad_acc.key.as_ref(),
    ];
    let local_txn_pda_seed = hash(&local_msg_.concat()).to_bytes();
    let (local_addr, _bump) = Pubkey::find_program_address(&[&local_txn_pda_seed], program_id);
    let local_pda_ = next_account_info(accounts_iter)?;
    checkpdalamports(local_pda_, &local_addr, program_id, true)?;
    let ed25519_instruction = load_instruction_at_checked(0, sysvar)?;
    let ed25519_program_id = Pubkey::from_str("Ed25519SigVerify111111111111111111111111111").unwrap();
    if ed25519_instruction.program_id != ed25519_program_id {
        return Err(ProgramError::InvalidAccountOwner)
    }
    let sig_data: &[u8] = &ed25519_instruction.data;

    /* (   data: &[u8], merchant_pubkey: &[u8], user_pubkey: &[u8], 
    referrer_pubkey: &[u8], merch_msg: &[u8], user_msg: &[u8], 
    ref_msg: &[u8],merch_sig: &[u8], user_sig: &[u8], ref_sig: &[u8])
    */

    // okay now for the construction of the contents of the signature for the user/merchant
    // I am gonna have it be:
    // b64 of txn, cost of product purchase, ad identifier, merch_pubkey, user_pubkey, ref_pubkey, merch_user_timestamp 

    let cost = txn_data.cost.to_le_bytes().to_vec();
    let ts = txn_data.mu_timestamp.to_le_bytes().to_vec();
    let agreed_msg = [
        &txn_data.b64txn,
        &cost,
        ad_acc.key.as_ref(),
        merchant_acc.key.as_ref(),
        user_acc.key.as_ref(),
        ref_acc.key.as_ref(),
        &ts,
    ];
    // merchant key, referrer key, ad identifier
    let ref_msg = [
        merchant_acc.key.as_ref(),
        user_acc.key.as_ref(),
        ad_acc.key.as_ref(),
    ];
    // input data for e25519 instruction, pubkey of merchant/user/referrer, agreed message, ref_msg, merchant/user/referrer signature
    check_ed25519_data(sig_data, &merchant_acc.key.as_ref(), &user_acc.key.as_ref(), 
    &ref_acc.key.as_ref(), &agreed_msg.concat(), &ref_msg.concat(), &txn_data.merchant_signature, 
    &txn_data.user_signature, &txn_data.referrer_signature)?;
    


    let ed25519 = next_account_info(accounts_iter)?;
    
    
    let sysvar_instruction_pubkey = sysvar::instructions::ID;
    if *ed25519.key != ed25519_program_id || *sysvar.key != sysvar_instruction_pubkey {
        return Err(ProgramError::InvalidAccountOwner)
    }
    let mut user_init_needed: bool = false;
    let mut ref_init_needed: bool = false;
    // these 2 might not exist yet, might be uninitializied, check if initializied 

    if ref_pda_.data_is_empty() || *ref_pda_.owner != *program_id || ref_pda_.lamports() == 0 {
        // create the ref pda, ensure enough lamports
        ref_init_needed = true;
    }
    if user_pda_.data_is_empty() || *user_pda_.owner != *program_id || user_pda_.lamports() == 0 {
        // create the user pda, ensure enough lamports 
        user_init_needed = true;
    }

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
    } else if txn_data.mu_timestamp < current_time.unix_timestamp - 50 || txn_data.mu_timestamp > current_time.unix_timestamp + 1 {
        return Err(ProgramError::InvalidInstructionData)
    }

    // constructiong the data the user/merchant signed on 
    let mut combined_data = Vec::new();
    combined_data.extend_from_slice(&txn_data.referrer_timestamp.to_le_bytes());
    combined_data.extend_from_slice(user_acc.key.as_ref());
    combined_data.extend_from_slice(merchant_acc.key.as_ref());
    combined_data.extend_from_slice(&txn_data.mu_timestamp.to_le_bytes());
    combined_data.extend_from_slice(&txn_data.cost.to_le_bytes());
    combined_data.extend_from_slice(&global_txn_pda_seed);

    let ad_data = Ad::try_from_slice(&ad_acc.data.borrow())?;

    // this gives me the total amount of rewards to be distrbiuted between the user and referrer
    // example would be 100 and 4.5% posted rate, so 4.5 $ goes to referrer
    let user_rewards:u64 = txn_data.cost * ad_data.user_rate as u64 / 100 as u64;
    let ref_rewards:u64 = txn_data.cost * ad_data.referrer_rate as u64 / 100;

    
 /*
 add rewards tracking for each of their pda's 

 */

    

    if user_init_needed {
        let accs : [AccountInfo; 2]= [merchant_acc.clone(), ref_acc.clone()];
        create_user(user_pda_.key, user_rewards, &accs)?;
    } else {
        let mut user_data = user_pda_.data.borrow_mut();
        let mut rewards = User::try_from_slice(&user_data)?;
        rewards.rewards += user_rewards;
        rewards.serialize(&mut *user_data)?;
    }
    
    if ref_init_needed {
        let ref_accs : [AccountInfo; 2]= [merchant_acc.clone(), ref_acc.clone()];
 
        create_referrer(ref_pda_.key, ref_rewards, &ref_accs)?;
    }else {
        let mut ref_data = ref_pda_.data.borrow_mut();
        let mut rewards = User::try_from_slice(&ref_data)?;
        rewards.rewards += ref_rewards;
        rewards.serialize(&mut *ref_data)?;
    }
    // the msg macro is just logging success/fail, the rest of the data is just call data
    msg!("success");
    Ok(())
}


pub fn checkpdalamports(pda: &AccountInfo, expected_pda_key: &Pubkey, program_id: &Pubkey, check_lamports: bool) -> Result<(), ProgramError> {
    if pda.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    } else if pda.key != expected_pda_key {
        return Err(ProgramError::InvalidSeeds);
    } else if check_lamports {

        if pda.lamports() != 0 {
            let mut pda_data = pda.data.borrow_mut();
            let mut ts_data = Ts::try_from_slice(&pda_data)?;
            let current_time = Clock::get()?.unix_timestamp;
            if ts_data.timestamp <= (current_time-300) {
                ts_data.timestamp += 300;
                ts_data.serialize(&mut &mut *pda_data)?;
            } else {
                return Err(ProgramError::InvalidAccountData)
            }
        }
    }

    Ok(())
}
// input data for e25519 instruction, pubkey of merchant/user/referrer, agreed message, ref_msg, merchant/user/referrer signature
pub fn check_ed25519_data(
    data: &[u8], merchant_pubkey: &[u8], user_pubkey: &[u8], 
    referrer_pubkey: &[u8], agreed_msg: &[u8], 
    ref_msg: &[u8],merch_sig: &[u8], user_sig: &[u8], ref_sig: &[u8]) -> ProgramResult {
    // According to this layout used by the Ed25519Program
    // https://github.com/solana-labs/solana-web3.js/blob/master/src/ed25519-program.ts#L33
        // Expected values

    let exp_public_key_offset:      u16 = 16; // 2*u8 + 7*u16
    let exp_signature_offset:       u16 = exp_public_key_offset + merchant_pubkey.len() as u16;
    let exp_message_data_offset:    u16 = exp_signature_offset + merch_sig.len() as u16;
    //let exp_num_signatures:          u8 = 3;
    let exp_message_data_size:      u16 = agreed_msg.len().try_into().unwrap();
    
    // "Deserializing" byte slices

    // this describes everything about instruction
    //let num_signatures                  = &[data[0]];        // Byte  0
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
    for i in 0..3{
        let data_pubkey                     = &data[16*i..16*i+32];  // Bytes 16..16+32
        let data_sig                        = &data[48*i..48*i+64];  // Bytes 48..48+64
        let data_msg                        = &data[112*i..];      // Bytes 112..end
        // for the above need to change it based on how long each respective message is

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
            0 => {
                if data_pubkey != merchant_pubkey || data_msg != agreed_msg || data_sig != merch_sig {
                    return Err(InvalidInstructionData);
                }
            },
            1 => {
                if data_pubkey != user_pubkey || data_msg != agreed_msg || data_sig != user_sig{
                    return Err(InvalidInstructionData);
                }
            }, 
            2 => {
                if data_pubkey != referrer_pubkey || data_msg != ref_msg || data_sig != ref_sig {
                    return Err(InvalidInstructionData);
                }
            }
            _ =>  {
                return Err(InvalidInstructionData);
            }
        }

    }
     
    Ok(())
}
