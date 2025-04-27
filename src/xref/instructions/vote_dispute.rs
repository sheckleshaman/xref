use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};

pub fn vote_dispute(program_id: &Pubkey, instruction_data: &[u8], accounts: &[AccountInfo]) ->ProgramResult {
    
    Ok(())
}