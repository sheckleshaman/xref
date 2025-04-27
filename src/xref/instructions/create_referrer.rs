
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult,  program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, system_instruction, sysvar::rent::Rent,
    system_program,
};
use borsh::BorshSerialize;
pub fn create_referrer(program_id: &Pubkey, rewards: u64, accounts: &[AccountInfo]) -> ProgramResult {
    //let ad_data = Ad::try_from_slice(instruction_data)?;

    let acc_iter = &mut accounts.iter();
    let ref_acc = next_account_info(acc_iter)?;
    // here we validate that the ad isn't available already by finding pda addr
    let (addr, bump) = Pubkey::find_program_address(&[&ref_acc.key.to_bytes()], program_id);
    let pda_acc = next_account_info(acc_iter)?;
    if *pda_acc.key != addr {
        return Err(ProgramError::InvalidAccountOwner)
    } else if !pda_acc.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized)
    }
    let merch_acc = next_account_info(acc_iter)?;

    let system_program = next_account_info(acc_iter)?;
    if *system_program.key != system_program::ID {
        return Err(ProgramError::InvalidArgument)
    }
    // get the size of the data, need to solve this
    let account_span = rewards.to_le_bytes().len();
    let rent_rate = (Rent::get()?).minimum_balance(account_span);
    invoke_signed(
        &system_instruction::create_account(
            merch_acc.key,
            &addr,
            rent_rate,
            account_span as u64,
            program_id,
        ),
        &[
            merch_acc.clone(),
            pda_acc.clone(),
            system_program.clone(),
        ],
        // seeds go here
        &[&[
            &ref_acc.key.to_bytes(),
            &[bump],
        ]]
    )?;

    rewards.serialize(&mut &mut pda_acc.data.borrow_mut()[..])?;
    Ok(())
}