use borsh::BorshDeserialize;
use cross_program_invocatio_native_lever::SetPowerStatus;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
    pubkey::Pubkey,
    program_error::ProgramEror,
};

const TARGET_PROGRAM_ID = Pubkey::new_from_array([0; 32]);
entrypoint!(route);
pub fn route(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) ->ProgramResult{
    let accounts_iter = &mut accounts.iter();
    let xref_pgm = next_account_info(accounts_iter)?;
    if xref_pgm.key != TARGET_PROGRAM_ID {
        return Err(ProgramError::InvalidInstructionData)
    }
    let request = instruction_data[0];

    match request {
        // creating an ad
        0=> {
            execute(xref_pgm.key, instruction_data, accounts)?;
        },
        // removing an ad
        1=> {
            execute(xref_pgm.key, instruction_data, accounts)?;
        },
        // changing an ad
        2=> {
            execute(xref_pgm.key, instruction_data, accounts)?;
        },
        // creating merchant account
        3=> {
            execute(xref_pgm.key, instruction_data, accounts)?;
        },
        // initializing user/referrer account
        4=> {
            execute(xref_pgm.key, instruction_data, accounts)?;
        },
        // deleting a user/referrer account
        5=> {
            execute(xref_pgm.key, instruction_data, accounts)?;
        },
        // deleting a merchant account
        6=> {
            execute(xref_pgm.key, instruction_data, accounts)?;
        },
        // posting transaction of referral 
        7=> {
            execute(xref_pgm.key, instruction_data, accounts)?;
        },
        // catchall 
        _ => {
            return Err(ProgramError::InvalidInstructionData)
        }
    }
    Ok(())
}

pub fn execute(program_id: &Pubkey, instruction_data: &[u8], accounts: &[AccountInfo]) -> ProgramResult{
    let ix = Instruction::new_with_borsh(
        *program_id,                        // Our lever program's ID
        instruction_data,             // Passing instructions through
        accounts
            .iter()
            .map(|acc| {
                AccountMeta {
                    pubkey: *acc.key,
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                }
            })
            .collect(), // Just the required account for the other program
    );

    invoke(&ix, accounts)?;
    Ok(())
}