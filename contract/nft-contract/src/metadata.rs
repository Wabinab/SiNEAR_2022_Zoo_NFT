use crate::*;
pub type TokenId = String;
//defines the payout type we'll be returning as a part of the royalty standards.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
} 

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,  // Data URL
    pub base_uri: Option<String>,  // centralized gateway to decentralized storage. 
    pub reference: Option<String>,  // URL to JSON file with more info.
    pub reference_hash: Option<Base64VecU8>,  // Base64-encoded sha256 hash of JSON from ref field. 
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,  // URL to associated media. 
    pub media_hash: Option<Base64VecU8>,  // required if "media" included
    pub copies: Option<u64>,  // number of copies of this set of metadata in existence when minted.
    
    pub issued_at: Option<u64>,   // When token issued/minted
    pub expires_at: Option<u64>, // Unix epoch in millieseconds.
    pub starts_at: Option<u64>,  // when token starts being valid. 
    pub updated_at: Option<u64>,  // token last updated. 
    pub extra: Option<String>,  // anything extra store on chain. Can be stringified JSON. 
    pub reference: Option<String>,  // URL to off-chain JSON file with more info. 
    pub reference_hash: Option<Base64VecU8>,  
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    // owner of the token. 
    pub owner_id: AccountId,

    // list of approved account IDs that have access to transfer the token. 
    // This maps an account ID to an approval ID. 
    pub approved_account_ids: HashMap<AccountId, u64>,

    // next approval ID to give out
    pub next_approval_id: u64,

    pub royalty: HashMap<AccountId, u16>,
}

//The Json token is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    pub owner_id: AccountId, 
    pub shared_owners: Vec<AccountId>,
    pub ticket_used: Vec<bool>,
    pub token_id: TokenId,
    pub metadata: TokenMetadata,
    pub approved_account_ids: HashMap<AccountId, u64>,
    pub royalty: HashMap<AccountId, u16>,
}

pub trait NonFungibleTokenMetadata {
    //view call for returning the contract metadata
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}