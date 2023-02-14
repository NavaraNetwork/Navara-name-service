const EXTRA_BYTES: usize = 10000;

use near_sdk::{ext_contract};

use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ResolverArgs {
    owner_id: AccountId
}

#[ext_contract(name_resolver)]
pub trait NameResolver {
    fn owner_changed(&mut self, owner_id: AccountId) -> AccountId;
}

#[near_bindgen]
impl Contract {

    pub fn get_min_attach_balance(&self, args: &ResolverArgs) -> Balance {
        ((RESOLVER_WASM_CODE.len() + EXTRA_BYTES + args.try_to_vec().unwrap().len() * 2) as Balance
            * STORAGE_PRICE_PER_BYTE + 5)
            .into()
    }

    #[payable]
    pub fn setup(&mut self, token_id: TokenId) -> Promise {
        let token = self.token_owner_only(&token_id);
        let owner_id = token.owner_id;
        let resolver_args = ResolverArgs {
            owner_id: owner_id.to_owned()
        };
        let min_attach_balance = self.get_min_attach_balance(&resolver_args);
        let deposited = env::attached_deposit();
        assert!(deposited > min_attach_balance, "Deposited not enough balance");
        let resolver_account_id = AccountId::try_from(format!("{}.{}", token_id, env::current_account_id())).unwrap();
        Promise::new(resolver_account_id)
            .create_account()
            .transfer(min_attach_balance)
            .deploy_contract(RESOLVER_WASM_CODE.to_vec())
            .function_call("new".to_owned(), serde_json::to_vec(&resolver_args).unwrap(), 0, GAS).then(
                Self::ext(env::current_account_id()).failure_resolve(owner_id, deposited)
            )
    }

    pub fn take_ownership(&mut self, token_id: TokenId) -> Promise {
        self.token_owner_only(&token_id);
        let account_id = env::predecessor_account_id();
        let resolver_args = ResolverArgs {
            owner_id: account_id.to_owned()
        };
        let resolver_account_id = AccountId::try_from(format!("{}.{}", token_id, env::current_account_id())).unwrap();
        Promise::new(resolver_account_id)
            .function_call("owner_changed".to_owned(), serde_json::to_vec(&resolver_args).unwrap(), 0, GAS)
    }
}