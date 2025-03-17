use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};
pub mod instructions;
pub mod state;
use crate::instructions::*;

entrypoint!(process_instruction);
/*
order of accounts passed in:
caller_account (my router programs signed CPI call)
merchant account
referrer account
user account
*/
pub const ROUTER_PUBKEY: Pubkey = Pubkey::new_from_array([/* your router's pubkey here */]);

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
            return create_ad(program_id, accounts, instruction_data);
        },
        // removing an ad
        1=> {
            return remove_ad(accounts, instruction_data);
        },
        // changing an ad
        2=> {
            return edit_ad(accounts, instruction_data);
        },
        // creating merchant account
        3=> {
            return create_merchant(accounts, instruction_data);
        },
        // deleting a merchant account
        4=> {
            return delete_merchant(accounts, instruction_data);
        },
        // posting transaction of referral 
        5=> {
            return post_txn(accounts, instruction_data)?;
        },
        // dispute txn
        6=> {
            return dispute_txn(accounts, instruction_data);
        }, 
        // vote dispute 
        7=> {
            return vote_dispute(accounts, instruction_data);
        }, 
        // distribute weekly rewards
        8=> {
            return distribute_rewards(accounts, instruction_data);
        },
        // catchall 
        _ => {
            return Err()
        }
    }
}

