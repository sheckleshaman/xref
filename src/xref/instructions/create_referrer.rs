use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};

pub fn create_referrer(program_id: &Pubkey, instruction_data: u64, accounts: &[AccountInfo; 2]) ->ProgramResult {
    Ok(())
}