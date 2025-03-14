use borsh::BorshDeserialize;
use cross_program_invocatio_native_lever::SetPowerStatus;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
    pubkey::Pubkey,
};


entrypoint!(route);
pub fn route(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) ->Result<()> {
    let accounts_iter = &mut accounts.iter();
    let xref_pgm = next_account_info(accounts_iter)?;

    let request = instruction_data[0];

    match request {
        // creating an ad
        0=> {

        },
        // removing an ad
        1=> {

        },
        // changing an ad
        2=> {

        },
        // creating merchant account
        3=> {

        },
        // initializing user/referrer account
        4=> {

        },
        // deleting a user/referrer account
        5=> {

        },
        // deleting a merchant account
        6=> {

        },
        // posting transaction of referral 
        7=> {

        },
    }
    let ix = Instruction::new_with_borsh(
        *xref_pgm.key,                        // Our lever program's ID
        &set_power_status_instruction,             // Passing instructions through
        vec![AccountMeta::new(*power.key, false)], // Just the required account for the other program
    );

    invoke_signed(&ix, &[power.clone()]);
}
