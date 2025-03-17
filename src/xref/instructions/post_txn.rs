
use crate::state::txn;
use crate::instructions::create_user::create_user;
use crate::isntructions::create_referrer::create_referrer;

use solana_program::ed25519_dalek::{ed25519::signature::Signature, Signer, Verifier};

pub fn post_txn(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let txn_data = TxnData::try_from_slice(instruction_data)?;
    let account_iter = &mut accounts.iter();
    /*
    okay the PDA's we need to pull in are:
    - PDA signed by my program with the merchant wallet seed 
    - PDA signed by my program with the user wallet seed contained
    - PDA signed by my program with the referrer wallet seed contained
    - PDA hash table of recent transactions between merchant and user. 

     */

    let merchant_acc = next_account_info(account_iter)?;
    let ref_acc = next_account_info(accounts_iter)?;
    let user_acc = next_account_info(accounts_iter)?;
    
    let merchant_pda = find_program_address(program_id, &merchant_acc.pubkey.as_ref())?;
    let user_pda = find_program_address(program_id, &user_acc.key.as_ref())?;
    let ref_pda = find_program_address(program_id, &ref_acc.key.as_ref())?;

    let ed25519 = next_account_info(account_iter)?;
    let sysvar = next_account_info(account_iter)?;
    let ed25519_program_id = Pubkey::from_str("Ed25519SigVerify111111111111111111111111111").unwrap();|
    let sysvar_instruction_pubkey = sysvar::instructions::ID;
    if ed25519.key != ed25519_program_id || sysvar.key != sysvar_instruction_pubkey {
        return Err()
    }
    let user_init_needed: bool = false;
    let ref_init_needed: bool = false;
    // these 2 might not exist yet, might be uninitializied, check if initializied 
    let ref_pda = next_account_info(accounts_iter)?;
    let user_pda = next_account_info(accounts_iter)?;
    if !ref_pda.data_is_empty() && *ref_pda.owner != MY_PROGRAM_ID && ref_pda.lamports() > 0 {
        // create the ref pda, ensure enough lamports
        ref_init_needed = true;
    }
    if !user_pda.data_is_empty() && *user_pda.owner != MY_PROGRAM_ID && user_pda.lamports() > 0 {
        // create the user pda, ensure enough lamports 
        user_init_needed = true;
    }
    let mut sig_accounts: Vec<AccountInfo> = Vec::new();
    sig_accounts.push(ed25519.clone());
    sig_accounts.push(sysvar.clone());
    sig_accounts.push(merchant_acc.clone())
    if txn_data.referrer_timestamp < (txn_data.mu_timestamp - 86400) {
        // if referrer timestamp is within an hour 
        return Err()
    } else if txn_data.referrer_timestamp > (txn_data.mu_timestamp -4) {
        /* this is trying to find some erratic log timestamps
         my thinking here is any real user would take at least 
         4 seconds from the time they are referred to the time 
         they purchase the product. Any shorter and its highly
         likely bot activity*/
        return Err()
    } 

    // constructiong the data the user/merchant signed on 
    let mut combined_data = Vec::new();
    combined_data.extend_from_slice(&txn_data.referrer_timestamp.to_le_bytes()?);
    combined_data.extend_from_slice(user_acc.key.as_ref());
    combined_data.extend_from_slice(merchant_acc.key.as_ref());
    combined_data.extend_from_slice(&txn_data.timestamp.to_le_bytes());
    combined_data.extend_from_slice(&txn_data.cost.to_le_bytes());
    combined_data.extend_from_slice(&txn_id);

    let merc_instruction = ed25519_program::new_ed25519_instruction(
        merchant_acc,
        combined_data,
        txn_data.merchant_signature,
    );

    // Invoke the Ed25519 signature verification program for merchannt
    invoke(
        &merc_instruction,
        sig_accounts,  // The accounts required for execution
    )?;


    // constructing data for the signatures
    let user_instruction = ed25519_program::new_ed25519_instruction(
        user_acc,
        txn_data.message,
        txn_data.user_signature,
    );
    // invocation of ed25519_instruction for user signature
    invoke(
        &user_instruction,
        sig_accounts,
    )?;



    let ref_instruction =ed25519_program::new_ed25519_instruction(
        ref_acc,
        // combined data is timestamp + user_pubkey + merchant_pubkey
        combined_data,
        txn_data.referrer_signature,
    );
    // invocation of ed25519_instruction for referrer signature
    invoke(
        &ref_instruction,
        sig_accounts,
    )?;

    let rewards = txn_data.cost * .1;

    if user_init_need {
        create_user(user_pda, user_acc, merchant_acc, rewards)?;
    }
    if ref_init_needed {
        create_referrer(ref_pda, ref_acc, merchant_acc, rewards)?;
    }
    // the msg macro is just logging success/fail, the rest of the data is just call data
    msg!("success");
    Ok(());
}