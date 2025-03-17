
#[derive(BorshDeserialize)]
pub struct TxnData {
    pub b64txn: Vec<u8>,
    // passed in accounts array
    // ad_pda_addr: &Pubkey,
    pub merchant_signature: Vec<u8>,
    pub user_signature: Vec<u8>,
    pub cost: u8,// this is in usd
    pub global_txn_id: Vec<u8>, // this is the hash of the user/merchant/referrer
    pub local_txn_id: Vec<u8>, // this is the hash of the user/merchant
    pub mu_timestamp: u64, // merchant/user agreed upon timestamp
    pub referrer_signature: Vec<u8>,
    pub referrer_timestamp: u64,
}
// wallets should be passed in as accounts and validated against this informtion
