use crate::*;

#[near_bindgen]
impl Contract {
    // views
    
    /// returns the number of sales the marketplace has up
    /// (as a string)
    pub fn get_supply_sales(&self) -> U64 {
      U64(self.sales.len())
    }

    /// Get number of ticket left for a certain template
    pub fn get_tickets_left(
      &self,
      template_id: String
    ) -> u64 {
      if let Some(value) = self.minted.get(&template_id) {
        value
      } else {
        0u64
      }
    }

    /// Get total number of tickets that can be minted 
    /// for certain template. 
    pub fn get_total_tickets(
      &self,
      template_id: String
    ) -> u64 {
      if let Some(value) = self.max_mint.get(&template_id) {
        value
      } else {
        0u64
      }
    }

    /// returns the number of sales for a given account
    /// (result is a string)
    pub fn get_supply_by_owner_id(&self, account_id: AccountId) -> U64 {
      let by_owner_id = self.by_owner_id.get(&account_id);

      if let Some(by_owner_id) = by_owner_id {
        U64(by_owner_id.len())
      } else {
        U64(0)
      }
    }

    /// returns paginated sale objects for a given account.
    /// (result is a vector of sales)
    pub fn get_sales_by_owner_id(
      &self,
      account_id: AccountId,
      from_index: Option<U128>,
      limit: Option<u64>,
    ) -> Vec<Sale> {
      let by_owner_id = self.by_owner_id.get(&account_id);
      let sales = if let Some(by_owner_id) = by_owner_id {
        by_owner_id
      } else {
        return vec![];
      };

      let keys = sales.as_vector();  // UnorderedSet to vector of strings. 

      let start = u128::from(from_index.unwrap_or(U128(0)));

      keys.iter()
          .skip(start as usize)
          .take(limit.unwrap_or(10) as usize)  // limit makes sense to have 10, not 0. 
          .map(|token_id| self.sales.get(&token_id).unwrap())
          .collect()
    }

    pub fn get_supply_by_nft_contract_id(
      &self,
      nft_contract_id: AccountId,
      from_index: Option<U128>,
      limit: Option<u64>,
    ) -> U64 {
      let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);

      if let Some(by_nft_contract_id) = by_nft_contract_id {
        U64(by_nft_contract_id.len())
      } else {
        U64(0)
      }
    }

    /// returns paginated sale objects associated with a given nft contract.
    /// (result is a vector of sales)
    pub fn get_sales_by_nft_contract_id(
      &self,
      nft_contract_id: AccountId,
      from_index: Option<U128>,
      limit: Option<u64>,
    ) -> Vec<Sale> {
      let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);

      let sales = if let Some(by_nft_contract_id) = by_nft_contract_id {
        by_nft_contract_id
      } else {
        return vec![];
      };

      let keys = sales.as_vector();

      let start = u128::from(from_index.unwrap_or(U128(0)));

      keys.iter()
          .skip(start as usize)
          .take(limit.unwrap_or(10) as usize)
          .map(|token_id| self.sales.get(&format!(
            "{}{}{}", nft_contract_id, DELIMITER, token_id
          )).unwrap())
          .collect()
    }

    /// get a sale information for a given unique sale ID
    /// (contract + DELIMITER + token ID)
    pub fn get_sale(&self, nft_contract_id: ContractAndTokenId) -> Option<Sale> {
      self.sales.get(&nft_contract_id)
    }
}