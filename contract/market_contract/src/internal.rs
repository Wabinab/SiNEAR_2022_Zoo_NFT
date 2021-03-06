use crate::*;

/// use to generate a unique prefix in our storage collections
/// (to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}


impl Contract {
    /// internal methods for removing a sale from the market. This returns
    /// the previously removed sale object.
    pub(crate) fn internal_remove_sale(
      &mut self, 
      nft_contract_id: AccountId,
      token_id: TokenId,
    ) -> Sale {
      let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMITER, token_id);
      
      let sale = expect_lightweight(
        self.sales.remove(&contract_and_token_id),
        "No Sale"
      );

      let mut by_owner_id = expect_lightweight(
        self.by_owner_id.get(&sale.owner_id),
        "No sale found by owner id."
      );
      by_owner_id.remove(&contract_and_token_id);
      

      // if set of sale is now empty, we simply remove owner from map. 
      if by_owner_id.is_empty() {
        self.by_owner_id.remove(&sale.owner_id);
      } else {  // else insert set back into map for owner.
        self.by_owner_id.insert(&sale.owner_id, &by_owner_id);
      }

      // get the set of token IDs for sale for nft contract ID. Panic if none. 
      let mut by_nft_contract_id = expect_lightweight(
        self.by_nft_contract_id.get(&nft_contract_id),
        "No sale found by nft contract id."
      );
      by_nft_contract_id.remove(&token_id);

      if by_nft_contract_id.is_empty() {
        self.by_nft_contract_id.remove(&nft_contract_id);
      } else {
        self.by_nft_contract_id.insert(&nft_contract_id, &by_nft_contract_id);
      }

      sale
    }


    /// Refund deposit, usually for storage used. 
    pub(crate) fn refund_deposit(&mut self, storage_used: u64, to_signer: AccountId) {
      let required_cost_to_store_info = env::storage_byte_cost() 
          * Balance::from(storage_used);  // move up if fail. 
    
      let attached_deposit = env::attached_deposit();
    
      require!(  // use assert or if and env::panic if fail. 
        required_cost_to_store_info <= attached_deposit,
        format!("Must attach {} yoctoNEAR to cover storage",
          required_cost_to_store_info),
      );
    
      let refund = attached_deposit - required_cost_to_store_info;
    
      // if refund is greater than 1 yoctoNEAR, 
      // refund the predecessor that amount. 
      if refund > 1 {
          Promise::new(to_signer).transfer(refund);
      }
    }
}