use crate::*;

#[near_bindgen]
impl Contract {
    //Query for the total supply of NFTs on the contract
    pub fn nft_total_supply(&self) -> U128 {
        U128(self.token_metadata_by_id.len() as u128)
    }

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn nft_tokens(
      &self, 
      from_index: Option<U128>, 
      limit: Option<u64>) 
    -> Vec<JsonToken> {
      // get a vector of keys
      let keys = self.token_metadata_by_id.keys_as_vector();

      let start = u128::from(from_index.unwrap_or(U128(0)));

      // iterate through keys
      keys.iter()
          .skip(start as usize)
          .take(limit.unwrap_or(0) as usize)
          .map(|token_id| self.nft_token(token_id.clone()).unwrap())
          .collect()
    }

    //get the total supply of NFTs for a given owner
    pub fn nft_supply_for_owner(
        &self,
        account_id: AccountId,
    ) -> U128 {
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

        if let Some(tokens_for_owner_set) = tokens_for_owner_set {
          U128(tokens_for_owner_set.len() as u128)
        } else {
          U128(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        // get the set of tokens from this owner. 
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

        // if exist some tokens, set tokens variable equal that set.
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
          tokens_for_owner_set
        } else {
          return vec![];  // if None. 
        };

        // convert UnorderedSet into vector of strings.
        let keys = tokens.as_vector();

        // start pagination. 
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // iterate through key vectors
        keys.iter()
            .skip(start as usize)  // skip to index specified. 
            .take(limit.unwrap_or(0) as usize)  // take first "limit" in vec
            // map token IDs (strings) into Json Tokens
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()  // turn iterator back to vector to return
    }
}