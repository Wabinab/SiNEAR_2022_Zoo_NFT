use crate::*;
use near_sdk::require;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        perpetual_royalties: Option<HashMap<AccountId, u16>>,
        size: Option<usize>,
        refund_to_signer: Option<AccountId>,
    ) {
      // measure the initial storage being used on contract.
        let initial_storage_usage = env::storage_usage();

        // create royalty map to store the token.
        let mut royalty = HashMap::new();

        // if perpetual royalties were passed into the function. 
        if let Some(perpetual_royalties) = perpetual_royalties {
          // our max payout is 7 people, otherwise not enough GAS. 
          require!(
            perpetual_royalties.len() < 7,
            "Cannot add more than 6 perpetual royalty amounts"
          );

          for (account, amount) in perpetual_royalties {
            royalty.insert(account, amount);
          }
        }

        // specify the token struct that contains the owner ID. 
        let token = Token {
          owner_id: receiver_id.clone(),
          approved_account_ids: Default::default(),  // default value is empty map.
          next_approval_id: 0,
          royalty,
        };

        // insert token ID and token struct and make sure token
        // doesn't exist. 
        require!(
          self.tokens_by_id.insert(&token_id, &token).is_none(),
          "Token already exists."
        );

        self.token_metadata_by_id.insert(&token_id, &metadata);

        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // ============================================================
        if let Some(size) = size {
          self.share_nfts.insert(&token_id, &vec![receiver_id; size - 1]);
          self.ticket_used.insert(&token_id, &vec![false; size]);
        } else {
          self.ticket_used.insert(&token_id, &vec![false; 1]);  // being explicit about size.
        }

        // ===========================================================

        // Log the minting as per events standard. 
        let nft_mint_log: EventLog = EventLog {
          standard: NFT_STANDARD_NAME.to_string(),
          version : NFT_METADATA_SPEC.to_string(),
          event   : EventLogVariant::NftMint(vec![NftMintLog {
            owner_id : token.owner_id.to_string(),
            token_ids: vec![token_id.to_string()],
            memo     : None,  // optional
          }]),
        };

        // log serialized json
        env::log_str(&nft_mint_log.to_string());

        // calculate required storage
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        // refund excess storage if user attached too much. 
        // Panic if they didn't attach enough. 
        if let Some(refund_to_signer) = refund_to_signer {
          refund_deposit(required_storage_in_bytes, refund_to_signer);
        } else {
          refund_deposit(required_storage_in_bytes, env::predecessor_account_id());
        }
        
    }
}