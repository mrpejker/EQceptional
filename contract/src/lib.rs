mod constants;

use near_contract_standards::non_fungible_token::{Token, TokenId, NonFungibleToken};
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
  };

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, log, near_bindgen, PanicOnDefault, AccountId, BorshStorageKey, Promise, PromiseOrValue
};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{ LazyOption, LookupMap};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde_json::json;

pub use constants::{BASE_URL, DATA_IMAGE_SVG_NEAR_ICON, ONE_NEAR, ONE_YOCTO, SINGLE_CALL_GAS};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Experience,
    TokenType,
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

//  tokentype
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenType {
    Interest,
    Joy,
    Trust,
    Love
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    // NFT implementation
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    experience: LookupMap<TokenId, u64>,
    tokentype: LookupMap<TokenId, TokenType>,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let owner_id = env::current_account_id(); // Who deployed owns

        let metadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "EQceptional earned NFT".to_string(),
            symbol: "EQ".to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            base_uri: Some(BASE_URL.to_string()),
            reference: None,
            reference_hash: None,
        };
        metadata.assert_valid();        

        Self {
            owner_id: owner_id.clone().into(),
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            experience: LookupMap::new(StorageKey::Experience),
            tokentype: LookupMap::new(StorageKey::TokenType),
        }                
    }

    // We don't use this method in current version
    #[payable]
    pub fn play(&mut self) -> u8 {
        //let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();
        assert!((deposit > ONE_NEAR), "not enough currency to play");
        
        // Toss the dice
        let rand: u8 = *env::random_seed().get(0).unwrap();
        return rand;
    }

    // Update user statistics
    pub fn update_user_stats(&mut self, token_id: TokenId , new_experience: u64) {
        self.experience.insert(&token_id, &new_experience);
    }

    // Get statistics of a hero
    pub fn get_stats(&self, token_id: TokenId) -> ( Option<u64>, Option<TokenType>) {
        (self.experience.get(&token_id), self.tokentype.get(&token_id))
    }

    // Mint nft ans send them to `username` account
    #[payable]
    pub fn new_token(&mut self, username: String, token_type: String) -> TokenId {
        let timestamp: u64 = env::block_timestamp();
        let rand: u8 = *env::random_seed().get(0).unwrap();
        let token_id: String = format!("USER:{}:{}", rand, timestamp);
        log!("token id: {}", token_id.clone());

        // Default to common token
        let mut token_metadata = TokenMetadata {
            title: None,
            description: None,
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: Some(timestamp.to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };

        // Choose and set tokentype
        let tokentype = match token_type.as_str() {
            "interest" => {
                token_metadata.title = Some("Interest".to_string());
                token_metadata.description = Some("Interest NFT Description".to_string());
                token_metadata.media = Some("interest.png".to_string());
                Some(TokenType::Interest)
            },
            "joy" => {
                token_metadata.title = Some("Joy".to_string());
                token_metadata.description = Some("Joy NFT Description".to_string());
                token_metadata.media = Some("motivation.png".to_string());
                Some(TokenType::Joy)
            },
            "trust" => {
                token_metadata.title = Some("Trust".to_string());
                token_metadata.description = Some("Trust NFT Description".to_string());
                token_metadata.media = Some("trust.png".to_string());
                Some(TokenType::Trust)
            },
            "love" => {
                token_metadata.title = Some("Love".to_string());
                token_metadata.description = Some("Love NFT Description".to_string());
                token_metadata.media = Some("commit.png".to_string());
                Some(TokenType::Love)
            },
            _ => None,
        };        
        assert_ne!(tokentype, None, "Wrong token type parameter");
        token_metadata.media_hash = Some(Base64VecU8(env::sha256(token_metadata.media.clone().unwrap().as_bytes())));

        // Mint NFT 
        let contract_id = env::current_account_id();
        let root_id = AccountId::try_from(contract_id).unwrap();
        self.nft_mint(token_id.clone(), root_id.clone(), token_metadata.clone());

        // Transfer NFT to new owner
        log!("username: {}", username.clone());
        let receiver_id = AccountId::try_from(username).unwrap();
        log!("receiver id: {}", receiver_id.clone());
        log!("token_id: {}", token_id.clone());
        env::promise_create(
            root_id,
            "nft_transfer",
            json!({
                "token_id": token_id.clone(),
                "receiver_id": receiver_id,
            })
            .to_string()
            .as_bytes(),
            ONE_YOCTO,
            SINGLE_CALL_GAS,
        );
        log!("Success! NFT transfering for {}! Token ID = {}", receiver_id.clone(), token_id.clone());

        // Init token stats
        self.experience.insert(&token_id, &0);                
        self.tokentype.insert(&token_id, &tokentype.unwrap());

        token_id
    }

    // Mint a new token with ID=token_id belonging to receiver_id.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. self.tokens.mint will also require it to be Some, since
    /// StorageKey::TokenMetadata was provided at initialization.
    ///
    /// self.tokens.mint will enforce predecessor_account_id to equal the owner_id given in
    /// initialization call to new.
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.tokens.internal_mint(token_id, receiver_id, Some(token_metadata))
    }
}

// Implement NFT standart
near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
  fn nft_metadata(&self) -> NFTContractMetadata {
      self.metadata.get().unwrap()
  }
}