use crate::*;

#[near_bindgen]
impl Contract {
    pub fn set_ipfs(&mut self, value: String) {
        Self::require_owner();
        self.ipfs.set(&value);
    }

    pub fn ipfs(&self) -> Option<String> {
        return self.ipfs.get()
    }
}