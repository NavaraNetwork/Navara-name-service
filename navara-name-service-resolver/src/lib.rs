use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, PromiseOrValue, Promise, Balance
};
use near_sdk_contract_tools::owner::OwnerExternal;
use near_sdk_contract_tools::{owner::Owner, Owner};

//The Json token is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    pub network: String,
    pub address: Option<String>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Owner)]
pub struct Contract {
    registry: AccountId,
    address_by_networks: UnorderedMap<String, String>,
    ipfs: LazyOption<String>,
    text_records: HashMap<String, String>,
    icon: LazyOption<String>,
}

const DATA_IMAGE_SVG_NAVARA_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    AddressByNetworks,
    Ipfs,
    Icon,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        let mut contract = Self::new(
            
        );
        Owner::init(&mut contract, &owner_id);
        contract
    }

    #[init]
    pub fn new() -> Self {
        require!(!env::state_exists(), "Already initialized");
        let registry = env::predecessor_account_id();
        Self {
            registry,
            address_by_networks: UnorderedMap::new(StorageKey::AddressByNetworks),
            ipfs: LazyOption::new(StorageKey::Ipfs, None),
            text_records: HashMap::new(),
            icon: LazyOption::new(StorageKey::Icon, Some(&DATA_IMAGE_SVG_NAVARA_ICON.to_owned())),
        }
    }

    pub fn resolve(&self, network: String) -> JsonToken {
        JsonToken {
            network: network.to_owned(),
            address: self.address_by_networks.get(&network)
        }
    }

    pub fn set_addresses(&mut self, addresses: HashMap<String, String>) {
        Self::require_owner();
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

    fn only_registry(&self) {
        assert_eq!(env::predecessor_account_id(), self.registry, "Only registry")
    }

    pub fn owner_changed(&mut self, new_owner: AccountId) -> PromiseOrValue<bool> {
        self.only_registry();
        let signer = env::signer_account_id();
        let previous_owner = self.own_get_owner().unwrap();
        assert_ne!(signer, previous_owner, "Owner not changed");
        let initial_storage_usage = env::storage_usage(); 
        Self::update_owner(self, Some(new_owner.to_owned()));
        self.address_by_networks.clear();
        self.ipfs.remove();
        self.text_records.clear();
        let storage_released = env::storage_usage() - initial_storage_usage;
        Promise::new(new_owner).transfer(Balance::from(storage_released) * env::storage_byte_cost());
        PromiseOrValue::Value(true)
    }

    pub fn set_ipfs(&mut self, value: String) {
        Self::require_owner();
        self.ipfs.set(&value);
    }

    pub fn ipfs(&self) -> Option<String> {
        return self.ipfs.get()
    }

    pub fn set_text_records(&mut self, records: HashMap<String, String>) {
        Self::require_owner();
        for (key, value) in records {
            self.text_records.insert(key, value);
        }
        assert!(self.text_records.len() <= 10, "Too many records")
    }

    pub fn get_text_records(&self) -> HashMap<String, String> {
        self.text_records.to_owned()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.get_addresses(None, None).len(), 0);
    }

    #[test]
    fn test_add_addresses() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(1).into());
        let mut addresses = HashMap::new();
        let bitcoin = "bitcoin".to_string();
        let ethereum = "ethereum".to_string();
        let ethereum_address = "0xB65B139A319A09F088486C22D18074810BA99715".to_string();
        let bitcoin_address = "mp8g4GZLbAUJZyY7DTMMHroiW9SzbocJUh".to_string();
        addresses.insert(ethereum.to_owned(), ethereum_address.to_owned());
        addresses.insert(bitcoin.to_owned(), bitcoin_address.to_owned());
        contract.set_addresses(addresses);
        assert_eq!(contract.get_addresses(None, None).len(), 2);
        assert_eq!(contract.resolve(ethereum).address.unwrap(), ethereum_address);
        assert_eq!(contract.resolve(bitcoin).address.unwrap(), bitcoin_address);
    }

    #[test]
    #[should_panic(expected = "Owner only")]
    fn test_add_addresses_panic() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into());
        let mut addresses = HashMap::new();
        let ethereum = "ethereum".to_string();
        let ethereum_address = "0xB65B139A319A09F088486C22D18074810BA99715".to_string();
        addresses.insert(ethereum.to_owned(), ethereum_address.to_owned());
        contract.set_addresses(addresses);
    }

    #[test]
    fn test_add_ipfs() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(1).into());
        let ipfs = "bafybeighxhsavoanjqkqvnnpbkvoweurybjt7gauunbg37ueahcbze5ise".to_owned();
        contract.set_ipfs(ipfs.to_owned());
        assert_eq!(contract.ipfs().unwrap(), ipfs);
    }

    #[test]
    fn test_add_record() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(1).into());
        let mut records = HashMap::new();
        let facebook = "facebook".to_string();
        let youtube = "youtube".to_string();
        let facebook_url = "https://facebook.com".to_string();
        let youtube_url = "https://youtube.com".to_string();
        records.insert(facebook.to_owned(), facebook_url.to_owned());
        records.insert(youtube.to_owned(), youtube_url.to_owned());
        contract.set_text_records(records);
        assert_eq!(contract.get_text_records().len(), 2);
        assert_eq!(contract.get_text_records().get(&facebook).unwrap(), &facebook_url);
        assert_eq!(contract.get_text_records().get(&youtube).unwrap(), &youtube_url);
    }

    #[test]
    fn test_owner_changed() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(1).into());
        contract.owner_changed(accounts(2));
    }
}