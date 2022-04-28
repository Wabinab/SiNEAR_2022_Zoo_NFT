use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
  require, assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance,
  Gas, PanicOnDefault, Promise, CryptoHash, BorshStorageKey,
};
use std::collections::HashMap;

use near_helper::{expect_lightweight, yoctonear_to_near};

use crate::external::*;
use crate::internal::*;
use crate::sale::*;
use near_sdk::env::STORAGE_PRICE_PER_BYTE;

mod external;
mod internal;
mod nft_callbacks;
mod sale;
mod sale_views;

// GAS constants
const GAS_FOR_ROYALTIES: Gas = Gas(115_000_000_000_000);
const GAS_FOR_NFT_TRANSFER: Gas = Gas(15_000_000_000_000);

// attach 0 NEAR to call
const NO_DEPOSIT: Balance = 0;

// min storage to have a sale on contract
const STORAGE_PER_SALE: u128 = 1000 * STORAGE_PRICE_PER_BYTE;

// every sale have unique ID: `CONTRACT + DELIMITER + TOKEN_ID`
static DELIMITER: &str = "_";

// custom types
pub type SalePriceInYoctoNear = U128;
pub type TokenId = String;
pub type FungibleTokenId = AccountId;
pub type ContractAndTokenId = String;

// payout as part of royalty standard.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

// main contract structure to store all info
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,

    // ContractAndTokenId is unique Identifier for every sale. 
    // = `contract ID + DELIMITER + token ID`
    pub sales: UnorderedMap<ContractAndTokenId, Sale>,

    // keep track of all sale IDs for every account
    pub by_owner_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>,

    // keep track of all token IDs for sale for a given contract
    pub by_nft_contract_id: LookupMap<AccountId, UnorderedSet<TokenId>>,

    // keep track of storage that accounts payed. 
    pub storage_deposits: LookupMap<AccountId, Balance>,
}


/// Helper structure for keys of persistent collections
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Sales,
    ByOwnerId,
    ByOwnerIdInner { account_id_hash: CryptoHash },
    ByNFTContractId,
    ByNFTContractIdInner { account_id_hash: CryptoHash },
    ByNFTTokenType,
    ByNFTTokenTypeInner { token_type_hash: CryptoHash },
    FTTokenIds,
    StorageDeposits,
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
      Self {
        owner_id,
        sales: UnorderedMap::new(StorageKey::Sales),
        by_owner_id: LookupMap::new(StorageKey::ByOwnerId),
        by_nft_contract_id: LookupMap::new(StorageKey::ByNFTContractId),
        storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
      }
    }

    /// Allow users to deposit storage. This cover cost of storing sale objects
    /// on the contract. Optional account ID for users to pay for storage for
    /// other people. 
    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
      let storage_account_id = account_id
              .map(|a| a.into())
              // if no specify account_id, use caller of function
              .unwrap_or_else(env::predecessor_account_id);

      let deposit = env::attached_deposit();

      require!(
        deposit >= STORAGE_PER_SALE,
        format!(
          "Requires minimum deposit of {}",
          STORAGE_PER_SALE
        ),
      );

      let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
      balance += deposit;
      self.storage_deposits.insert(&storage_account_id, &balance);
    }


    //// Allow users to withdraw any excess storage that they're not using. 
    #[payable]
    pub fn storage_withdraw(&mut self) {
      assert_one_yocto();

      let owner_id = env::predecessor_account_id();

      // storage deposit stores the excess. If they're not in the map, no
      // excess. 
      let mut amount = self.storage_deposits.remove(&owner_id).unwrap_or(0);

      // how many sales the user taking up currently. Returns a set. 
      let sales = self.by_owner_id.get(&owner_id);

      // get length of that set
      let len = sales.map(|s| s.len()).unwrap_or_default();

      // how much NEAR is used up for all current sales on the account
      let diff = u128::from(len) * STORAGE_PER_SALE;

      // excess
      amount -= diff;

      // if excess > 0, we transfer amount to user. 
      if amount > 0 {
        Promise::new(owner_id.clone()).transfer(amount);
      }

      // Add back storage used up into the map if it's greater than 0. 
      // this will be freed (withdrawable) when storage no longer need. 
      if diff > 0 {
        self.storage_deposits.insert(&owner_id, &diff);
      }
    }

    // views
    /// return the minimum storage for 1 sale
    pub fn storage_minimum_balance(&self) -> U128 {
      U128(STORAGE_PER_SALE)
    }

    /// returns how much storage an account has paid for. 
    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
      U128(self.storage_deposits.get(&account_id).unwrap_or(0))
    }
}
