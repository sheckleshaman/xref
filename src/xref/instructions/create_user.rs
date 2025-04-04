use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};
// first index in account array will be the user, second is the merchant
pub fn create_user(program_id: &Pubkey, reward_amount: u64, accounts: &[AccountInfo; 2]) ->ProgramResult {
    Ok(())
}