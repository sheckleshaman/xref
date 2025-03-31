use crate::state::ad::Ad;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    
};


pub fn create_ad(program_id: &Pubkey, instruction_data:&[u8], accounts: &[AccountInfo]) -> ProgramResult {
    let ad_data = Ad::try_from_slice(instruction_data)?;

    let acc_iter = &mut accounts.iter();
    let merc_acc = next_account_info(acc_iter)?;
    // here we validate that the ad isn't available already by finding pda addr
    let (addr, _bump) = Pubkey::find_program_address(&[ad_data], program_id);
    let account_span = ad_data.len();
    invoke(
        &system_instruction::create_account(
            payer.key,
            addr.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            payer.clone(),
            addr.clone(),
            system_program.clone(),
        ],
    )?;

    ad_data.serialize(&mut &mut addr.data.borrow_mut()[..])?;
    Ok(())
}