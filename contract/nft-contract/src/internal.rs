use crate::*;
use near_sdk::{CryptoHash, require};
use std::mem::size_of;

const BORSH_EXTRA_BYTES_FOR_STRLEN_STORAGE: u64 = 4;


/// used to generate a unique prefix in our storage
/// collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
  let mut hash = CryptoHash::default();

  // hash the account id and return it
  hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
  hash
}


/// calculate how many bytes the account ID is taking up
pub(crate) fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
  account_id.as_str().len() as u64 
    + BORSH_EXTRA_BYTES_FOR_STRLEN_STORAGE
    + size_of::<u64>() as u64
}


/// convert royalty percentage and amount to pay into payout (U128)
pub(crate) fn royalty_to_payout(royalty_percentage: u16, amount_to_pay: Balance) -> U128 {
  U128(royalty_percentage as u128 * amount_to_pay / 10_000u128)
}


/// refund storage taken up, passing in approved account IDs and send the funds
/// to the passed-in account ID. 
pub(crate) fn refund_approved_account_ids_iter<'a, I>(
  account_id: AccountId,
  approved_account_ids: I,  // approved account IDs must be passed in as iterator
) -> Promise
where
  I: Iterator<Item = &'a AccountId>,
{
  // sum of all bytes for each approved account IDs. 
  let storage_released: u64 = approved_account_ids.map(bytes_for_approved_account_id).sum();

  // transfer to account storage cost equivalent to storage released. 
  Promise::new(account_id).transfer(
    Balance::from(storage_released) * env::storage_byte_cost()
  )
}


/// refund storage taken up to a given account id
pub(crate) fn refund_storage_to_owner(
  owner_id: AccountId,
  storage_released: u64,
) -> Promise {
    Promise::new(owner_id).transfer(
      Balance::from(storage_released) * env::storage_byte_cost()
    )
}


/// refund a map of approved account IDs and send the funds to the passed in 
/// account ID
pub(crate) fn refund_approved_account_ids(
  account_id: AccountId,
  approved_account_ids: &HashMap<AccountId, u64>,
) -> Promise {
  refund_approved_account_ids_iter(account_id, approved_account_ids.keys())
}


/// refund the initial deposit based on the amount of 
/// storage that was used up
pub(crate) fn refund_deposit(storage_used: u64) {
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
    Promise::new(env::predecessor_account_id()).transfer(refund);
  }
}


/// Assert user has attached at least 1 yoctoNEAr (for security reasons
/// and payment of storage)
pub(crate) fn assert_at_least_one_yocto() {
  require!(
    env::attached_deposit() >= 1,
    "Requires attached deposit of at least 1 yoctoNEAR",
  )
}


impl Contract {
    /// add a token to the set of tokens an owner has. 
    pub(crate) fn internal_add_token_to_owner(
      &mut self,
      account_id: &AccountId,
      token_id: &TokenId,
    ) {
      let mut tokens_set = self.tokens_per_owner.get(account_id)
        .unwrap_or_else(|| {
            UnorderedSet::new(
              StorageKey::TokenPerOwnerInner {
                account_id_hash: hash_account_id(&account_id),
              }
              .try_to_vec()
              .unwrap(),
            )
      });
      
      tokens_set.insert(token_id);

      self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    /// remove a token from an owner.
    pub(crate) fn internal_remove_token_from_owner(
      &mut self,
      account_id: &AccountId,
      token_id: &TokenId
    ) {
      let mut tokens_set = self
          .tokens_per_owner
          .get(account_id)
          // If no set of tokens for owner, panic
          .expect("Token should be owned by the sender.");

      // remove the token_id from set of tokens.
      tokens_set.remove(token_id);

      // if token set is now empty, remove owner from the 
      // tokens_per_owner collection
      if tokens_set.is_empty() {
        self.tokens_per_owner.remove(account_id);
      } else {
        // not empty, we insert it back for account ID
        self.tokens_per_owner.insert(account_id, &tokens_set);
      }
    }

    /// transfer the NFT to the receiver_id
    /// (cli can't call internal methods)
    pub(crate) fn internal_transfer(
      &mut self,
      sender_id: &AccountId,
      receiver_id: &AccountId,
      token_id: &TokenId,
      approval_id: Option<u64>,  // approved ID can transfer token. 
      memo: Option<String>,
    ) -> Token {
      let token = self.tokens_by_id.get(token_id).expect("No token");

      // if sender not owner, panic. 
      if sender_id != &token.owner_id {
        if !token.approved_account_ids.contains_key(sender_id) {
          env::panic_str("Unauthorized transaction");
        }
        
        // if included approval_id, check if sender's actual approval_id
        // is the same as the one included
        if let Some(enforced_approval_id) = approval_id {
          let actual_approval_id = token
                  .approved_account_ids
                  .get(sender_id)  // if, get and return. 
                  .expect("Sender is not approved account");  // if not, panic. 

          require!(
            actual_approval_id == &enforced_approval_id,
            format!(
              "The actual approval_id {} differs from the given approval_id {}",
              actual_approval_id,
              enforced_approval_id,
            ),
          );
        }
      }

      // make sure sender isn't sending token to themselves. 
      require!(
        &token.owner_id != receiver_id,
        "The token owner and receiver should be different"
      );

      // remove token from it's current owner's set
      self.internal_remove_token_from_owner(&token.owner_id, token_id);

      // add token to receiver_id's set.
      self.internal_add_token_to_owner(receiver_id, token_id);

      // create new token struct
      let new_token = Token {
        owner_id: receiver_id.clone(),
        // reset approval account IDs
        approved_account_ids: Default::default(),
        next_approval_id: token.next_approval_id,
        royalty: token.royalty.clone(),
      };

      self.tokens_by_id.insert(token_id, &new_token);  // replace old entry

      // log memo if available
      if let Some(memo) = memo.as_ref() {
        env::log_str(&format!("Memo: {}", memo).to_string());
      }

      let mut authorized_id = None;
      if approval_id.is_some() {
        authorized_id = Some(sender_id.to_string());
      }

      let nft_transfer_log: EventLog = EventLog {
        standard: NFT_STANDARD_NAME.to_string(),
        version : NFT_METADATA_SPEC.to_string(),
        event   : EventLogVariant::NftTransfer(vec![NftTransferLog {
          authorized_id,
          old_owner_id: token.owner_id.to_string(),
          new_owner_id: receiver_id.to_string(),
          token_ids   : vec![token_id.to_string()],
          memo,
        }]),
      };

      // log serialized json
      env::log_str(&nft_transfer_log.to_string());

      // return previous token object that was transferred. 
      token
    }


}
