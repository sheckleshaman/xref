use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Ad {
    pub url: String, // url of the page with the purchase page for ad
    pub name: String, // name of the product for the ad
    pub industry_code: u8, // type of industry this is
    pub total_rate: u8, // rewards rate for the product, must be within range per industry code
    pub referrer_rate: u8, // rate for the referrer
    pub user_rate: u8, // points funded to user pool or direct back to user, or bought with whatever memecoin
    pub type_: bool, // indirect or direct
    pub additional_params: Vec<u8>, // parameters the merchant requires, if SPA or some shit 
}