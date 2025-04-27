

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    system_instruction,
    program::invoke_signed,
};

pub fn delete_user(program_id: &Pubkey, instruction_data: &[u8], accounts: &[AccountInfo]) ->ProgramResult {
    // we are going to remove the lamports from the user account and close out. 
    // need to check that they are the user account first and foremost
    let acc_iter = &mut accounts.iter();
    let user_acc = next_account_info(acc_iter)?;
    let user_pda = next_account_info(acc_iter)?;
    let system_program = next_account_info(acc_iter)?;

    let (addr, bump) = Pubkey::find_program_address(&[&user_acc.key.to_bytes()], program_id);
    if &addr != user_pda.key {
        return Err(ProgramError::InvalidAccountData)
    }

    let lmps = user_pda.lamports();
    invoke_signed(
        &system_instruction::transfer(
            user_pda.key,
            
            user_acc.key,
            lmps,
        ),
        &[
            user_pda.clone(),
            user_acc.clone(),
            system_program.clone(),
        ],
        &[&[&user_acc.key.to_bytes(),&[bump]]],
    )?;

    // After transferring, set pda_account lamports to 0 manually
    **user_pda.try_borrow_mut_lamports()? = 0;

    Ok(())
}