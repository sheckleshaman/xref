use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};
pub mod instructions;
pub mod state;
use crate::instructions::{
    edit_ad::edit_ad,
    create_ad::create_ad,
    create_merchant::create_merchant,
    delete_ad::delete_ad,
    dispute_txn::dispute_txn,
    post_txn::post_txn,
    distribute_rewards::distribute_rewards,
    delete_merchant::delete_merchant,
    vote_dispute::vote_dispute,
};

entrypoint!(process_instruction);
/*
order of accounts passed in:
caller_account (my router programs signed CPI call)
merchant account
referrer account
user account
*/
pub const ROUTER_PUBKEY: Pubkey = Pubkey::new_from_array([00, 00, 00, 00, 00,00,
    00, 00, 00, 00, 00,00,
    00, 00, 00, 00, 00,00,
    00, 00, 00, 00, 00,00,
    00, 00, 00, 00, 00,00,
    00, 00]);

pub fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let acc_iter = &mut accounts.iter();
    // 
    let caller_acc = next_account_info(acc_iter)?;
    if caller_acc.is_signer != true {
        return Err(ProgramError::AccountBorrowFailed)
    } else if *caller_acc.key != ROUTER_PUBKEY {
        return Err(ProgramError::IllegalOwner)
    }

    match instruction_data[0] {
        // creating an ad
        0=> {
            return create_ad(program_id, instruction_data, accounts);
        },
        // removing an ad
        1=> {
            return delete_ad(program_id, instruction_data, accounts);
        },
        // changing an ad
        2=> {
            return edit_ad(program_id, instruction_data, accounts);
        },
        // creating merchant account
        3=> {
            return create_merchant(program_id, instruction_data, accounts);
        },
        // deleting a merchant account
        4=> {
            return delete_merchant(program_id, instruction_data, accounts);
        },
        // posting transaction of referral 
        5=> {
            return post_txn(program_id, accounts, instruction_data);
        },
        // dispute txn
        6=> {
            return dispute_txn(program_id, instruction_data, accounts);
        }, 
        // vote dispute 
        7=> {
            return vote_dispute(program_id, instruction_data, accounts);
        }, 
        // distribute weekly rewards
        8=> {
            return distribute_rewards(program_id, instruction_data, accounts);
        },
        // catchall 
        _ => {
            return Err(ProgramError::InvalidInstructionData)
        }
    }
}

