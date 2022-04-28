use crate::*;

// Approval callbacks from NFT contracts

/// struct for keeping track of sale conditions for a Sale
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleArgs {
    pub sale_conditions: SalePriceInYoctoNear,
}


/// when nft_approve is called, it'll fire a cross contract 
/// call to this marketplace and this is the function that is
/// invoked. 
trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
      &mut self,
      token_id: TokenId,
      owner_id: AccountId,
      approval_id: u64,
      msg: String,
    );
}


/// implementation on the trait
#[near_bindgen]
impl NonFungibleTokenApprovalsReceiver for Contract {

    fn nft_on_approve(
      &mut self,
      token_id: TokenId,
      owner_id: AccountId,
      approval_id: u64,
      msg: String,
    ) {
      let nft_contract_id = env::predecessor_account_id();
      let signer_id = env::signer_account_id();

      require!(
        nft_contract_id != signer_id,
        format!(
          concat!(
            "nft_contract_id: {}, signer_id: {}, must be called via cross contract. ",
            "Error if signer is also contract host."
          ),
          nft_contract_id,
          signer_id
        ),
        // "nft_on_approve should only be called via cross-contract call"
      );

      require!(
        owner_id == signer_id,
        "Only owner can call nft_on_approve. "
      );

      // Enforce user has enough storage for 1 EXTRA sale. 

      let storage_amount = self.storage_minimum_balance().0;
      let owner_paid_storage = self.storage_deposits.get(&signer_id).unwrap_or(0);
      let signer_storage_required = (
        self.get_supply_by_owner_id(signer_id).0 + 1  // 1 EXTRA
      ) as u128 * storage_amount;

      require!(
        owner_paid_storage >= signer_storage_required,
        format!(
          "Insufficient storage paid: {}, for {} sales at {} rate per sale",
          owner_paid_storage, 
          signer_storage_required / STORAGE_PER_SALE,
          STORAGE_PER_SALE,
        ),
      );

      // if all checks pass we can create sale conditions object

      let SaleArgs { sale_conditions } = 
          near_sdk::serde_json::from_str(&msg).unwrap_or_else(|err|
            env::panic_str("Message passed in is not valid SaleArgs")
      );

      let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMITER, token_id);

      self.sales.insert(
        &contract_and_token_id,
        &Sale {
          owner_id: owner_id.clone(),
          approval_id,
          nft_contract_id: nft_contract_id.to_string(),
          token_id: token_id.clone(),
          sale_conditions,
        },
      );

      // Extra functionality that populates collections necessary for view calls. 

      // get the sales by owner ID for given owner. If none, create new empty set. 
      let mut by_owner_id = self.by_owner_id.get(&owner_id).unwrap_or_else(|| {
        UnorderedSet::new(
          StorageKey::ByOwnerIdInner {
            account_id_hash: hash_account_id(&owner_id),
          }
          .try_to_vec()
          .unwrap(),
        )
      });

      by_owner_id.insert(&contract_and_token_id);
      self.by_owner_id.insert(&owner_id, &by_owner_id);

      // get token IDs for given nft contract ID. Create new empty set if none. 
      let mut by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id)
            .unwrap_or_else(|| {
              UnorderedSet::new(
                StorageKey::ByNFTContractIdInner {
                  account_id_hash: hash_account_id(&nft_contract_id),
                }
                .try_to_vec()
                .unwrap(),
              )
            });

      by_nft_contract_id.insert(&token_id);
      self.by_nft_contract_id.insert(&nft_contract_id, &by_nft_contract_id);
    }
}