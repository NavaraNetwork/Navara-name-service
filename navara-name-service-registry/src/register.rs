use crate::*;

impl Contract {
    fn metadata_by_token(&self, token_id: &TokenId) -> TokenMetadata {
        TokenMetadata {
            title: Some(format!("{}.nns", token_id.to_owned())),
            description: Some("Navara name service powered by NEAR protocol".into()),
            media: Some(DATA_IMAGE_SVG_NEAR_ICON.to_owned()),
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn register(
        &mut self, 
        token_id: TokenId, 
        token_owner_id: AccountId
    ) -> Promise {
        if let Some(name_expired_date) = self.name_expired_date.get(&token_id) {
            assert!(name_expired_date < env::block_timestamp_ms(), "Unexpired");
        }
        let deposited = env::attached_deposit() - REGISTER_GAS_DEPOSIT;
        assert!(deposited >= self.price_for_one_year, "Deposit at least one NEAR");
        let years_extended: u64 = (deposited / self.price_for_one_year).try_into().unwrap();
        let token_metadata = self.metadata_by_token(&token_id);
        Self::ext(env::current_account_id()).with_attached_deposit(deposited).register_name(token_id.to_owned(), token_owner_id.to_owned(), token_metadata, years_extended).then(
            Self::ext(env::current_account_id()).failure_resolve(token_owner_id, deposited)
        )
    }

    #[private]
    #[payable]
    pub fn register_name(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
        years_extended: u64
    ) -> Token {
        let signer = env::signer_account_id();
        let new_expired_date = env::block_timestamp_ms() + (years_extended * ONE_YEAR_MILLISECOND);
        self.update_expired_date(&token_id, &new_expired_date);
        if let Some(token) = self.nft_token(token_id.to_owned()) {
            self.tokens.internal_transfer_unguarded(&token_id, &token.owner_id, &signer);
            self.nft_token(token_id).unwrap()
        } else {
            self.tokens.internal_mint(token_id, token_owner_id, Some(token_metadata))
        }
    }
}