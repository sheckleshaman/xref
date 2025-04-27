use crate::state::ad::Ad;
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, hash::hash, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, system_instruction, sysvar::rent::Rent,
    system_program,
};
use borsh::{BorshSerialize, BorshDeserialize};
pub fn create_ad(program_id: &Pubkey, instruction_data:&[u8], accounts: &[AccountInfo]) -> ProgramResult {
    let ad_data = Ad::try_from_slice(instruction_data)?;
    let mut combined = Vec::new();
    combined.extend_from_slice(&ad_data.industry_code.to_le_bytes());
    combined.extend_from_slice(ad_data.url.as_bytes());
    combined.extend_from_slice(ad_data.name.as_bytes());
    combined.extend_from_slice(&ad_data.type_.to_string().as_bytes());
    let hashed = hash(&combined);
    let acc_iter = &mut accounts.iter();
    let merc_acc = next_account_info(acc_iter)?;
    // here we validate that the ad isn't available already by finding pda addr
    let (addr, bump) = Pubkey::find_program_address(&[&hashed.to_bytes(), &merc_acc.key.to_bytes()], program_id);
    let pda_acc = next_account_info(acc_iter)?;
    if *pda_acc.key != addr {
        return Err(ProgramError::InvalidAccountOwner)
    } else if !pda_acc.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized)
    }

    let system_program = next_account_info(acc_iter)?;
    if *system_program.key != system_program::ID {
        return Err(ProgramError::InvalidArgument)
    }
    // get the size of the data, need to solve this
    let account_span = instruction_data.len();
    let rent_rate = (Rent::get()?).minimum_balance(account_span);
    invoke_signed(
        &system_instruction::create_account(
            merc_acc.key,
            &addr,
            rent_rate,
            account_span as u64,
            program_id,
        ),
        &[
            merc_acc.clone(),
            pda_acc.clone(),
            system_program.clone(),
        ],
        // seeds go here
        &[&[
            &hashed.to_bytes(),
            &merc_acc.key.to_bytes(),
            &[bump],
        ]]
    )?;

    ad_data.serialize(&mut &mut pda_acc.data.borrow_mut()[..])?;
    Ok(())
}