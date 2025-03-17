
#[derive(BorshDeserialize)]
pub struct TxnData {
    b64txn: Vec<u8>,
    // passed in accounts array
    // ad_pda_addr: &Pubkey,
    merchant_signature: Vec<u8>,
    user_signature: Vec<u8>,
    cost: u8;// this is in usd
    global_txn_id: Vec<u8>, // this is the hash of the user/merchant/referrer
    local_txn_id: Vec<u8>, // this is the hash of the user/merchant
    mu_timestamp: u64, // merchant/user agreed upon timestamp
    referrer_signature: Vec<u8>,
    referrer_timestamp: u64,
}
// wallets should be passed in as accounts and validated against this informtion
