use crate::*;
use near_sdk::require;

pub trait NonFungibleTokenCore {
    //calculates the payout for a token given the passed in balance. This is a view method
  	fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u16) -> Payout;
    
    //transfers the token to the receiver ID and returns the payout object that should be payed given the passed in balance. 
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: String,
        balance: U128,
        max_len_payout: u16,
    ) -> Payout;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    //calculates the payout for a token given the passed in balance. This is a view method
    fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u16) -> Payout {
        let token = self.tokens_by_id.get(&token_id).expect("No token.");

        let owner_id = token.owner_id;
        let mut total_perpetual = 0;  // perpetual royalties. 
        let balance_u128 = u128::from(balance);

        // keep track of payout object to send back
        let mut payout_object = Payout {
          payout: HashMap::new()
        };

        let royalty = token.royalty;  // royalty object from token

        // make sure we're not paying out to too many people (GAS limits this)
        require!(
          royalty.len() as u16 <= max_len_payout, 
          "Market cannot payout to that many receivers."
        );

        // go through each key and value in royalty object
        for (k, v) in royalty.iter() {
          let key = k.clone();

          // only insert payout if key isn't token owner (payout at end)
          if key != owner_id {
            payout_object.payout.insert(key, royalty_to_payout(*v, balance_u128));
            total_perpetual += *v;
          }
        }

        // payout to previous owner gets 100%: total perpetual royalties. 
        payout_object.payout.insert(
          owner_id, 
          royalty_to_payout(10000 - total_perpetual, balance_u128));

        payout_object
	}

    //transfers the token to the receiver ID and returns the payout object that should be payed given the passed in balance. 
    #[payable]
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: String,
        balance: U128,
        max_len_payout: u16,
    ) -> Payout {
        assert_one_yocto();
        
        let sender_id = env::predecessor_account_id();

        // transfer token to receiver and function returns previous token object. 
        let previous_token = self.internal_transfer(
          &sender_id,
          &receiver_id,
          &token_id,
          Some(approval_id),
          Some(memo),
        );

        // refund previous token owner for storage used up by previous approved
        // account IDs
        refund_approved_account_ids(
          previous_token.owner_id.clone(),
          &previous_token.approved_account_ids,
        );

        let owner_id = previous_token.owner_id;
        let mut total_perpetual = 0;  // keep track of total perpetual royalties.
        let balance_u128 = u128::from(balance);
        let mut payout_object = Payout {
          payout: HashMap::new()
        };
        let royalty = previous_token.royalty;

        require!(
          royalty.len() as u16 <= max_len_payout,
          "Market cannot payout to that many receivers"
        );

        for (k, v) in royalty.iter() {
          let key = k.clone();
          
          if key != owner_id {
            payout_object.payout.insert(
              key, 
              royalty_to_payout(*v, balance_u128)
            );
            total_perpetual += *v;
          }
        }

        // payout to previous owner
        payout_object.payout.insert(
          owner_id,
          royalty_to_payout(10000 - total_perpetual, balance_u128)
        );

        payout_object
    }
}
