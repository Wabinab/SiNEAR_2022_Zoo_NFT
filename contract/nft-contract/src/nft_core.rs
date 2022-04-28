use crate::*;
use near_sdk::{ext_contract, log, Gas, PromiseResult, require};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

pub trait NonFungibleTokenCore {
    /// set other accounts
    fn set_accounts(
      &mut self,
      token_id: TokenId,
      share_accounts: Vec<AccountId>,
    );

    //transfers an NFT to a receiver ID
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
    );

    //transfers an NFT to a receiver and calls a function on the receiver ID's contract
    /// Returns `true` if the token was transferred from the sender's account.
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> ;

    //get information about the NFT token passed in
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
}

#[ext_contract(ext_non_fungible_token_receiver)]
trait NonFungibleTokenReceiver {
    //Method stored on the receiver contract that is called via cross contract call when nft_transfer_call is called
    /// Returns `true` if the token should be returned back to the sender.
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> Promise ;
}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    /*
        resolves the promise of the cross contract call to the receiver contract
        this is stored on THIS contract and is meant to analyze what happened in the cross contract call when nft_on_transfer was called
        as part of the nft_transfer_call method
    */
    fn nft_resolve_transfer(
        &mut self,
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: HashMap<AccountId, u64>,
        memo: Option<String>,
    ) -> bool;
}

/*
    resolves the promise of the cross contract call to the receiver contract
    this is stored on THIS contract and is meant to analyze what happened in the cross contract call when nft_on_transfer was called
    as part of the nft_transfer_call method
*/ 
trait NonFungibleTokenResolver {
    fn nft_resolve_transfer(
        &mut self,
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: HashMap<AccountId, u64>,
        memo: Option<String>,
    ) -> bool;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    /// Set accounts. 
    /// Payable to pay for storage; but limit how much deposit user can attach. 
    #[payable]
    fn set_accounts(
      &mut self,
      token_id: TokenId,
      share_accounts: Vec<AccountId>,
    ) {
      require!(
        env::attached_deposit() >= near_to_yoctonear(0.1),
        "Attached too little money. Min attach: 0.1 NEAR."
      );

      let owner_id = env::predecessor_account_id();

      let token = expect_lightweight(
        self.tokens_by_id.get(&token_id),
        "Cannot find token."
      );

      require!(
        token.owner_id == owner_id,
        "Only owner of token can set accounts."
      );

      let old_share_accounts = expect_lightweight(
        self.share_nfts.get(&token_id),
        "Token is not sharable"
      );

      require!(
        old_share_accounts.len() == share_accounts.len(),
        format!(
          "Expected share_accounts has {} items, but found {}. Length mismatch.",
          old_share_accounts.len(),
          share_accounts.len()
        )
      );

      // If we were to implement iterator like this: 
      // approved_account_ids.map(bytes_for_approved_account_id).sum();
      // it needs the iterator trait; One is too lazy to fight the compiler,
      // so we'll use for loops. 

      // Old used bytes
      let mut old_bytes = 0;
      for account_id in &old_share_accounts {
        old_bytes += bytes_for_approved_account_id(account_id);
      }
      
      // New going to use bytes
      let mut new_bytes = 0;
      for account_id in &share_accounts {
        new_bytes += bytes_for_approved_account_id(account_id);
      }

      if old_bytes > new_bytes {
        let refund_bytes = old_bytes - new_bytes;
        let _ = refund_storage_to_owner(
          token.owner_id, 
          refund_bytes
        );  // we didn't have callbacks in case refund fails, for simplicity. 
        // Might include in the future handling callbacks. 
      } else if new_bytes > old_bytes {
        let required_storage = new_bytes - old_bytes;
        refund_deposit(required_storage);
      }

      // else no changes, do nothing to storage. 
      
      // Change the values after sorting out the storage
      self.share_nfts.insert(&token_id, &share_accounts);

      // Ok, I think that's done? Anything I missed out? 
    }

    //implementation of the nft_transfer method. This transfers the NFT from the current owner to the receiver. 
    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
    ) {
        // assert user attached exactly 1 yoctoNEAR. This is for security
        // and that the user will be redirected to the NEAR wallet. 
        assert_one_yocto();

        // get sender to transfer token from sender to receiver. 
        let sender_id = env::predecessor_account_id();

        // call the internal transfer method
        // return previous token so we can refund the approved account IDs. 
        let previous_token = self.internal_transfer(
          &sender_id,
          &receiver_id,
          &token_id,
          Some(approval_id),
          memo,
        );

        // refund owner for releasing the used up storage by approved account IDs.
        refund_approved_account_ids(
          previous_token.owner_id.clone(),
          &previous_token.approved_account_ids,
        );
    }

    //implementation of the transfer call method. This will transfer the NFT and call a method on the receiver_id contract
    #[payable]
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        assert_one_yocto();

        // assert enough GAS
        let attached_gas = env::prepaid_gas();
        require!(
          attached_gas >= MIN_GAS_FOR_NFT_TRANSFER_CALL,
          format!(
            "You cannot attach less than {:?} Gas to nft_transfer_call",
            MIN_GAS_FOR_NFT_TRANSFER_CALL
          ),
        );

        let sender_id = env::predecessor_account_id();

        // transfer token and get previous token object
        let previous_token = self.internal_transfer(
          &sender_id,
          &receiver_id,
          &token_id,
          Some(approval_id),
          memo.clone(),
        );

        // we now need authorized ID to be passed in to 
        // function. 
        let mut authorized_id = None;

        // if sender not owner of token, set authorized ID = sender.
        if sender_id != previous_token.owner_id {
          authorized_id = Some(sender_id.to_string());
        }

        // initiating receiver's call and callback
        ext_non_fungible_token_receiver::nft_on_transfer(
          sender_id,
          previous_token.owner_id.clone(),
          token_id.clone(),
          msg,
          receiver_id.clone(),  // contract account to make the call to
          NO_DEPOSIT,  // attached deposit
          env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL,  // attached gas.
        )
        .then(ext_self::nft_resolve_transfer(
          authorized_id,
          previous_token.owner_id,
          receiver_id,
          token_id,
          previous_token.approved_account_ids,
          memo,
          env::current_account_id(),  // contract make call
          NO_DEPOSIT,  // attached deposit
          GAS_FOR_RESOLVE_TRANSFER,  // attached GAS. 
        )).into()
        
    }

    //get the information for a specific token ID
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
        // If there is some token ID in the tokens_by_id collection
        if let Some(token) = self.tokens_by_id.get(&token_id) {
          let metadata = self.token_metadata_by_id.get(&token_id).unwrap();

          let ticket_used = self.ticket_used.get(&token_id).unwrap();

          let mut shared_owners = vec![];
          if ticket_used.len() > 1 {
            shared_owners = self.share_nfts.get(&token_id).unwrap();
          }
          

          // return JsonToken (wrapped by Some since it's Option)
          Some(JsonToken {
            token_id,
            owner_id: token.owner_id,
            shared_owners,
            ticket_used,
            metadata,
            approved_account_ids: token.approved_account_ids,
            royalty: token.royalty,
          })
        } else {
          None  // no tokenID in collection. 
        }
    }
}

#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    //resolves the cross contract call when calling nft_on_transfer in the nft_transfer_call method
    //returns true if the token was successfully transferred to the receiver_id
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: HashMap<AccountId, u64>,
        memo: Option<String>,  // for logging transfer event. 
    ) -> bool {
        // whether receiver returns token back to sender, based on 
        // `nft_on_transfer` call result. 

        if let PromiseResult::Successful(value) = env::promise_result(0) {
          if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value) {
            // don't need to return token, simply return true. 
            // everything went fine. 
            if !return_token {
              // Since we've alredy transferred token and nft_on_transfer returns false,
              // we don't have to revert the original transfer, thus we can just return
              // true since nothing went wrong. We refund the owner for releasing the
              // storage used up by the approved account IDs. 
              refund_approved_account_ids(owner_id, &approved_account_ids);
              return true;
            }
          }
        }

        // get token object if got some token object
        let mut token = if let Some(token) = self.tokens_by_id.get(&token_id) {
          if token.owner_id != receiver_id {  // receiver_id is the receiver. 
            refund_approved_account_ids(owner_id, &approved_account_ids);
            return true;  
          }
          token
        } else {  // no token object, it was burned. 
          refund_approved_account_ids(owner_id, &approved_account_ids);
          return true;
        };

        // if we haven't return true, that means we should return
        // the token to it's original owner (receiver_id):
        log!("Return {} from @{} to @{}", token_id, receiver_id, owner_id);

        // remove token from receiver
        self.internal_remove_token_from_owner(&receiver_id, &token_id);

        // add token to original owner
        self.internal_add_token_to_owner(&owner_id, &token_id);

        // we change the token struct's owner to the original owner
        token.owner_id = owner_id.clone();

        // refund approved account IDs may have set on token. 
        refund_approved_account_ids(receiver_id.clone(), &token.approved_account_ids);

        // we insert the token back into the tokens_by_id collection
        self.tokens_by_id.insert(&token_id, &token);

        // Insert token back to tokens_by_id collection
        self.tokens_by_id.insert(&token_id, &token);

        // log reverted NFT transfer. 
        let nft_transfer_log: EventLog = EventLog {
          standard: NFT_STANDARD_NAME.to_string(),
          version : NFT_METADATA_SPEC.to_string(),
          event   : EventLogVariant::NftTransfer(vec![NftTransferLog {
            authorized_id,
            old_owner_id: receiver_id.to_string(),
            new_owner_id: owner_id.to_string(),
            token_ids: vec![token_id.to_string()],
            memo,
          }]),
        };

        env::log_str(&nft_transfer_log.to_string());

        // receiver_id didn't successfully receive the token. 
        false
    }
}