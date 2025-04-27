use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DisputeDetails {
    pub txns: Vec<Pubkey>, // this is an array of pda's of which were generated upon merchant completion
    // this will help in justification, can whip through indexers and find out the relevant information quickly 
    pub malicious_actors: Vec<Pubkey>, // the accounts that are malicious or part of malicious activity
    pub justification: Vec<u8>, // this is why the dispute is submitted, reasoning
    pub reporter: Pubkey, // the account submitting the txn
}

