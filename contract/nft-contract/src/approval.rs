use crate::*;
use near_sdk::{ext_contract, Gas, require};

const GAS_FOR_NFT_APPROVE: Gas = Gas(10_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

pub trait NonFungibleTokenCore {
    //approve an account ID to transfer a token on your behalf
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>);

    //check if the passed in account has access to approve the token ID
	  fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;

    //revoke a specific account from transferring the token on your behalf
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);

    //revoke all accounts from transferring the token on your behalf
    fn nft_revoke_all(&mut self, token_id: TokenId);
}

#[ext_contract(ext_non_fungible_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    //cross contract call to an external contract that is initiated during nft_approve
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    //allow a specific account ID to approve a token on your behalf
    #[payable]
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) {
        // The user needs to attach enough to pay for storage on the contract.
        assert_at_least_one_yocto();

        // get token object from token ID
        let mut token = expect_lightweight(
          self.tokens_by_id.get(&token_id),
          "No token"
        );

        // Ensure person calling the function is the owner of the token. 
        require!(
          &env::predecessor_account_id() == &token.owner_id,
          "Predecessor must be the token owner."
        );

        // REDUNDANT CHECKS ABOVE. 

        // get next approval ID
        let approval_id: u64 = token.next_approval_id;
    
        // check account has been approved already for this token. 
        let is_new_approval = token
            .approved_account_ids
            .insert(account_id.clone(), approval_id)  // insert returns none if key not present
            .is_none();  // returns true if key NOT present, so it's a new approval. 
    
        // new approval requires calculate how much storaged being used to add the account.
        let storage_used = if is_new_approval {
          bytes_for_approved_account_id(&account_id)
        } else {
          0
        };
    
        token.next_approval_id += 1;
    
        // insert token back to tokens_by_id collection
        self.tokens_by_id.insert(&token_id, &token);
    
        // refund excess storage attached by user. If user didn't attach enough, panic. 
        refund_deposit(storage_used);

        // if message passed in, we initiate cross contract call on account we're 
        // giving access to. 
        if let Some(msg) = msg {
          ext_non_fungible_approval_receiver::nft_on_approve(
            token_id,
            token.owner_id,
            approval_id,
            msg,
            account_id,  // contract account we're calling
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
          )
          .as_return();
        }
    }

    //check if the passed in account has access to approve the token ID
	  fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        let token = self.tokens_by_id.get(&token_id).expect("No token.");

        let approval = token.approved_account_ids.get(&approved_account_id);

        // if there was some approval ID found
        if let Some(approval) = approval {
          // if a specific approval_id passed into function
          if let Some(approval_id) = approval_id {
            // return approval_id that matches actual approval ID for account
            approval_id == *approval
          } else {
            true  // if no approval_id, simply return true
          }
        } else {  // no approval ID, simply return false. 
          false
        }
    }

    //revoke a specific account from transferring the token on your behalf 
    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        assert_one_yocto(); 

        let mut token = self.tokens_by_id.get(&token_id).expect("No token.");

        // assert caller is owner of token. 
        let predecessor_account_id = env::predecessor_account_id();
        require!(&predecessor_account_id == &token.owner_id);

        // if account ID was in token's approval, remove it and 
        // if statement logic executes
        if token
            .approved_account_ids
            .remove(&account_id)
            .is_some()
        {
          // removing approved_account_id and refund funds. 
          refund_approved_account_ids_iter(predecessor_account_id, [account_id].iter());

          // insert token back to collection with removed account_id from approval list. 
          self.tokens_by_id.insert(&token_id, &token);
        }
    }

    //revoke all accounts from transferring the token on your behalf
    #[payable]
    fn nft_revoke_all(&mut self, token_id: TokenId) {
        assert_one_yocto();

        let mut token = self.tokens_by_id.get(&token_id).expect("No token.");

        let predecessor_account_id = env::predecessor_account_id();
        require!(&predecessor_account_id == &token.owner_id);

        // only revoke for token not empty approved account IDs. 
        if !token.approved_account_ids.is_empty() {
          refund_approved_account_ids(predecessor_account_id, &token.approved_account_ids);
          token.approved_account_ids.clear();
          self.tokens_by_id.insert(&token_id, &token);
        }
    }
}