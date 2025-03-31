use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshDeserialize)]
pub struct TxnData {
    // this will contain details I do not care to outline
    pub b64txn: Vec<u8>,
    // passed in accounts array
    // ad_pda_addr: &Pubkey,
    pub merchant_signature: Vec<u8>,
    pub user_signature: Vec<u8>,
    pub cost: u8,// this is in usd
    // the bottom 2 are constructed locally and should not be passed in by sender
    //pub global_txn_id: Vec<u8>, // this is the hash of the user/merchant/referrer
    //pub local_txn_id: Vec<u8>, // this is the hash of the user/merchant
    pub mu_timestamp: i64, // merchant/user agreed upon timestamp
    pub referrer_signature: Vec<u8>,
    pub referrer_timestamp: i64,
}
// wallets should be passed in as accounts and validated against this informtion
