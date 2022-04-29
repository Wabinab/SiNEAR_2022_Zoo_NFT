use crate::*;

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