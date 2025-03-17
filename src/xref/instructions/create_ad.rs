use crate::state::ad::Ad;

pub fn create_ad(instruction_data:&[u8], accounts: &[AccountInfo]) -> ProgramResult {
    let ad_data = Ad::try_from_slice(instruction_data)?;

    let acc_iter = &mut accounts.iter();
    let merc_acc = next_account_info(accounts)?;
    // here we validate that the ad isn't available already by finding pda addr
    let (addr, _bump) = find_program_address(&[ad_data, ])?;

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