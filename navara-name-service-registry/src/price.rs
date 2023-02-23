use near_sdk::{json_types::U128, assert_one_yocto};

use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn set_price(&mut self, price: U128) {
        assert_one_yocto();
        Self::require_owner();
        let new_price = u128::from(price);
        self.price_for_one_year = new_price;
    }

    pub fn price_per_year(&self) -> U128 {
        U128::from(self.price_for_one_year)
    }

    #[payable]
    pub fn set_fee_register(&mut self, price: U128) {
        assert_one_yocto();
        Self::require_owner();
        let new_fee = u128::from(price);
        self.fee_register = new_fee;
    }

    pub fn fee_register(&self) -> U128 {
        U128::from(self.fee_register)
    }
}