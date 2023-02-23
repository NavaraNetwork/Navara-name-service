use crate::*;

#[near_bindgen]
impl Contract {
    pub fn resolve(&self, network: String) -> JsonToken {
        JsonToken {
            network: network.to_owned(),
            address: self.address_by_networks.get(&network)
        }
    }

    pub fn set_addresses(&mut self, addresses: HashMap<String, String>) {
        Self::require_owner();
        for (key, value) in addresses {
            self.address_by_networks.insert(&key, &value);
        }
        assert!(self.text_records.len() <= 10, "Too many records")
    }

    pub fn get_addresses(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        let start = u128::from(from_index.unwrap_or(U128(0)));
        self.address_by_networks.keys()
        .skip(start as usize) 
        .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|network| self.resolve(network.clone()))
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}