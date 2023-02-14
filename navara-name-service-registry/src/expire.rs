use near_sdk::json_types::U64;

use crate::*;

impl Contract {

    pub fn is_name_expired(&self, token_id: &TokenId) -> bool {
        let name_expire_date = self.name_expired_date.get(token_id).unwrap();
        name_expire_date < env::block_timestamp_ms()
    }

    pub fn asset_name_expired(&self, token_id: &TokenId) {
        assert!(!self.is_name_expired(token_id), "Expired")
    }

    pub fn update_expired_date(&mut self, token_id: &TokenId, expired_date: &u64) {
        assert!(expired_date.to_owned() > env::block_timestamp_ms(), "Invalid expired date");
        self.name_expired_date.insert(token_id, expired_date);
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn extend(&mut self, token_id: TokenId) {
        self.token_owner_only(&token_id);
        
        let name_expired_date = self.name_expired_date.get(&token_id).unwrap_or(env::block_timestamp_ms());
        assert!(name_expired_date >= env::block_timestamp_ms(), "Name expired");

        let deposited = env::attached_deposit();
        let years_extended: u64 = (deposited / self.price_for_one_year).try_into().unwrap();
        let new_expired_date = name_expired_date + (years_extended * ONE_YEAR_MILLISECOND);
        self.update_expired_date(&token_id, &new_expired_date);
    }

    pub fn expired_date(&self, token_id: TokenId) -> Option<U64> {
        if let Some(expired_date) = self.name_expired_date.get(&token_id) {
            return Some(U64::from(expired_date))
        }
        None
    }
}