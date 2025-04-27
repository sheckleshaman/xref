
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};
use borsh::BorshDeserialize;
use crate::state::ad::DeleteAd;
pub fn delete_ad(program_id: &Pubkey, instruction_data: &[u8], accounts: &[AccountInfo]) -> ProgramResult {

    let deletion_info = DeleteAd::try_from_slice(instruction_data);

    // expectation is the hash is provided in the original request 


    let (pda_addr, bump) = Pubkey::find_program_address(&[&hash, merch_acc.key.to_bytes()], program_id);
    Ok(())
}