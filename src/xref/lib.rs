use solana_program::{

};

use crate::instructions::{
    create_da::create_ad, create_merchant::create_merchant,delete_merchant::delete_merchant, 
    dispute_txn::dispute_txn, edit_ad::edit_ad, remove_ad::remove_ad,
    post_txn::post_txn, vote_dispute::vote_dispute, distribute_rewards::distribute_rewards
    };
use crate::state;

entrypoint!(process_instruction);
/*
order of accounts passed in:
caller_account (my router programs signed CPI call)
merchant account
referrer account
user account
*/
pub const ROUTER_PUBKEY: Pubkey = Pubkey::new_from_array([/* your router's pubkey here */]);

pub fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> Result<()> {
    let mut acc_iter = accounts.iter();
    // 
    let caller_acc = next_account_info(acc_iter);
    if caller_acc.is_signer != true {
        return Err()
    } else if caller_acc.key != ROUTER_PUBKEY {
        return Err()
    }

    match instruction_data[0] {
        // creating an ad
        0=> {
            create_ad(program_id, accounts, instruction_data);
        },
        // removing an ad
        1=> {
            remove_ad(accounts, instruction_data);
        },
        // changing an ad
        2=> {
            edit_ad(accounts, instruction_data);
        },
        // creating merchant account
        3=> {
            create_merchant(accounts, instruction_data);
        },
        // deleting a merchant account
        4=> {
            delete_merchant(accounts, instruction_data);
        },
        // posting transaction of referral 
        5=> {
            post_txn(accounts, instruction_data);
        },
        // dispute txn
        6=> {
            dispute_txn(accounts, instruction_data);
        }, 
        // vote dispute 
        7=> {
            vote_dispute(accounts, instruction_data);
        }, 
        // distribute weekly rewards
        8=> {
            distribute_rewards(accounts, instruction_data);
        },
        // catchall 
        _ => {
            return Err()
        }
    }
}

