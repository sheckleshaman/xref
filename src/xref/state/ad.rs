#[derive(BorseDeserialize)]
pub struct Ad {
    pub url: String, // url of the page with the purchase page for ad
    pub name: String, // name of the product for the ad
    pub industry_code: u8, // type of industry this is
    pub cost: u8, // cost of the product 
    pub type_: bool, // indirect or direct
    pub additional_params: Vec<u8>, // parameters the merchant requires, if SPA or some shit 
}