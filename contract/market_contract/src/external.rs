use crate::*;

// External contract calls

/// Initiate a cross contract call to NFT contract. This will
/// transfer the token to the buyer and return a payout object
/// used for the market to distribute funds to appropriate accounts.
#[ext_contract(ext_contract)]
trait ExtContract {
    fn nft_transfer_payout(
      &mut self,
      receiver_id: AccountId,  // purchaser
      token_id: TokenId,  // token ID for transfer
      approval_id: u64,  // for transfer token on behalf of owner. 
      memo: String,
      balance: U128,  // price by owner + royalty. 
      max_len_payout: u32,  // max amount of accounts market can payout at once. 
    );

    fn nft_mint(
      &mut self,
      token_id: TokenId,
      metadata: TokenMetadata,
      receiver_id: AccountId,
      perpetual_royalties: Option<HashMap<AccountId, u16>>,
      size: Option<usize>,
      refund_to_signer: Option<AccountId>,
    );
}