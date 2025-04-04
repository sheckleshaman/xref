use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    pub rewards: u64,
    pub root_hash: Vec<u8>,
}