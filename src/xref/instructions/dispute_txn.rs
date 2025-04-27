use solana_program::{
    account_info::{next_account_info, AccountInfo},
  
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    program::invoke_signed,
    system_instruction,
    hash::hash,
};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::state::dispute::DisputeDetails;
pub fn dispute_txn(program_id: &Pubkey, instruction_data: &[u8], accounts: &[AccountInfo]) ->ProgramResult {

    // okay for this we are going to create a pda that will hold voting results, this will record addrs voting and their vote
    let acc_iter = &mut accounts.iter();

    let dispute_details = DisputeDetails::try_from_slice(instruction_data)?;
    let reporter = next_account_info(acc_iter)?;
    
    if dispute_details.reporter != *reporter.key {
        return Err(ProgramError::InvalidInstructionData)
    }
    let mut combined:Vec<u8> = Vec::new();
    combined.extend_from_slice(&[dispute_details.malicious_actors
        .iter()
        .flat_map(|pk| pk.as_ref())
    ]);
    combined.extend_from_slice(&[dispute_details.reporter]);
    combined.extend_from_slice(&dispute_details.justification);
    combined.extend_from_slice(&dispute_details.txns);
    let hashed = hash(&combined);

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
            &user_acc.key.to_bytes(),
            &[bump],
        ]]
    )?;

    rewards.serialize(&mut &mut pda_acc.data.borrow_mut()[..])?;
    Ok(())
}