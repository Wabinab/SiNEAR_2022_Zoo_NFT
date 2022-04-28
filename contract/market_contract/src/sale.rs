use crate::*;
use near_sdk::promise_result_as_success;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Sale {
    pub owner_id: AccountId,
    pub approval_id: u64,
    pub nft_contract_id: String,
    pub token_id: String,
    pub sale_conditions: SalePriceInYoctoNear,
}




#[near_bindgen]
impl Contract {

    /// Buy an mint-on-demand nft
    #[payable]
    pub fn pay_and_mint(
      &mut self,
      nft_contract_id: AccountId,
      price: U128,
      nft_seller_id: AccountId,
      token_id: TokenId,
      metadata: TokenMetadata,
      perpetual_royalties: Option<HashMap<AccountId, u16>>,
      size: Option<usize>
    ) {
      require!(
        env::attached_deposit() >= (u128::from(price) + near_to_yoctonear(0.1)),
        concat!(
          "You attached less than the amount needed to buy this NFT + storage deposit.",
          "Storage reserve 0.1N will MOSTLY BE RETURNED."
        )
      );

      require!(
        env::attached_deposit() <= (u128::from(price) + near_to_yoctonear(0.101)),
        "You attached too much near. This function requires EXACTLY price + 0.1N."
      );

      Promise::new(nft_seller_id).transfer(price.into()).then(
        ext_self::on_nft_mint(
          price,
          nft_contract_id,

          token_id,
          metadata,
          env::signer_account_id(),  // receiver of NFT is signer. 
          perpetual_royalties,
          size,
          Some(env::signer_account_id()),  // refund_to_signer
  
          env::current_account_id(),
          near_to_yoctonear(0.1),
          GAS_FOR_CALLBACK_AND_MINTING
        )
      );
      
    }


    /// Reduced gas method. In the future, we'll have a separate registration
    /// of token_minting-specific-information separately, so we don't have to 
    /// pass the variables around (which is the main cost of gas). These will 
    /// be LookupMap<CopyId, TokenMetadata> where we can retrieve a copy/clone
    /// of the metadata based on CopyId. 
    #[payable]
    pub fn pay_and_mint_unsafe(
      &mut self,
      nft_contract_id: AccountId,
      price: U128,
      nft_seller_id: AccountId,
      token_id: TokenId,
      metadata: TokenMetadata,
      perpetual_royalties: Option<HashMap<AccountId, u16>>,
      size: Option<usize>
    ) {
      require!(
        env::attached_deposit() >= (u128::from(price) + near_to_yoctonear(0.1)),
        concat!(
          "You attached less than the amount needed to buy this NFT + storage deposit.",
          "Storage reserve 0.1N will MOSTLY BE RETURNED."
        )
      );

      require!(
        env::attached_deposit() <= (u128::from(price) + near_to_yoctonear(0.101)),
        "You attached too much near. This function requires EXACTLY price + 0.1N."
      );

      
      Promise::new(nft_seller_id).transfer(price.into()).then(
        // unsafe as mint irregardless of success or fail transfer money. 

        ext_contract::nft_mint(
          token_id,
          metadata,
          env::signer_account_id(),  // receiver of NFT is signer. 
          perpetual_royalties,
          size,
          Some(env::signer_account_id()),  // refund_to_signer
  
          nft_contract_id,
          near_to_yoctonear(0.1),
          GAS_FOR_CALLBACK_AND_MINTING
        )
      );
    }

    /// removes a sale from the market
    #[payable]
    pub fn remove_sale(
      &mut self, 
      nft_contract_id: AccountId, 
      token_id: String
    ) {
      assert_one_yocto();
      let sale = self.internal_remove_sale(nft_contract_id.into(), token_id);
      let owner_id = env::predecessor_account_id();

      require!(
        owner_id == sale.owner_id,
        "Only sale owner can remove sale."
      );
    }

    /// updates the price for a sale on the market.
    #[payable]
    pub fn update_price(
      &mut self,
      nft_contract_id: AccountId,
      token_id: String,
      price: U128,
    ) {
      assert_one_yocto();

      let contract_id: AccountId = nft_contract_id.into();
      let contract_and_token_id = format!("{}{}{}", contract_id, DELIMITER, token_id);

      let mut sale = expect_lightweight(
        self.sales.get(&contract_and_token_id),
        "No Sale"
      );

      require!(
        env::predecessor_account_id() == sale.owner_id,
        "Only sale owner can update price."
      );

      sale.sale_conditions = price;
      self.sales.insert(&contract_and_token_id, &sale);
    }

    /// place an offer on a specific sale. Sale will go through as long as deposit
    /// is greater than or equals list price. 
    #[payable]
    pub fn offer(
      &mut self, 
      nft_contract_id: AccountId, 
      token_id: String
    ) {
      let deposit = env::attached_deposit();
      require!(
        deposit > 0,
        "Bidding price must be larger than 0 yoctoNEAR."
      );

      let contract_id: AccountId = nft_contract_id.into();
      let contract_and_token_id = format!("{}{}{}", contract_id, DELIMITER, token_id);

      let sale = expect_lightweight(
        self.sales.get(&contract_and_token_id),
        "No Sale"
      );

      let buyer_id = env::predecessor_account_id();
      require!(
        sale.owner_id != buyer_id,
        "Cannot bid on your own sale."
      );

      let price = sale.sale_conditions.0; 

      require!(
        deposit >= price,
        format!(
          "Insufficient price: You want to buy for ~{} N, but only attached ~{} N",
          yoctonear_to_near(price),
          yoctonear_to_near(deposit)
        ),
      );

      self.process_purchase(
        contract_id,
        token_id,
        U128(deposit),
        buyer_id,
      );
    }

    // ================ PRIVATE FUNCTIONS ======================= //
    
    /// Remove the sale, transfer and get payout from nft contract,
    /// then distribute royalties
    #[private]
    pub fn process_purchase(
      &mut self,
      nft_contract_id: AccountId,
      token_id: String,
      price: U128,
      buyer_id: AccountId
    ) -> Promise {
      let sale = self.internal_remove_sale(nft_contract_id.clone(), token_id.clone());

      // initiate cross-contract call. 
      // Transfer token to buyer and return payout object for distributing funds.
      ext_contract::nft_transfer_payout(
        buyer_id.clone(),
        token_id,
        sale.approval_id,
        "payout from market".to_string(),  // memo
        price,  // includes royalties
        10,  // max amount of accounts market can payout
        nft_contract_id,  // contract to initiate cross contract call to
        1,  // attached yoctoNEAR
        GAS_FOR_NFT_TRANSFER,
      )
      .then(ext_self::resolve_purchase(
        buyer_id,
        price,
        env::current_account_id(),  // invoking this function on current contract
        NO_DEPOSIT,
        GAS_FOR_ROYALTIES,
      ))
    }

    /// Resolve promise when calling nft_transfer_payout. Check for authenticity
    /// of payout object. Pay account if no problem; else refund buyer. 
    #[private]
    pub fn resolve_purchase(
      &mut self,
      buyer_id: AccountId,
      price: U128,
    ) -> U128 {
      // check payout info returned from nft_transfer_payout method. 
      let payout_option = promise_result_as_success().and_then(|value| {

        // if payout option None, something wrong, refund. 
        near_sdk::serde_json::from_slice::<Payout>(&value)
            .ok()
            // returns None if none; otherwise execute logic below. 
            .and_then(|payout_object| {  

              if payout_object.payout.len() > 10 || payout_object.payout.is_empty() {
                env::log_str("Either more than 10 royalties or nobody to payout.");
                None
              } else {

                let mut remainder = price.0;

                // loop through payout and subtract value from remainder.
                // this check for overflow or any errors and returns None if
                // there is problems. 
                for &value in payout_object.payout.values() {
                  remainder = remainder.checked_sub(value.0)?;
                }

                // check for faulty payout that requires us to pay more or too 
                // little. Remainder 0 if payout summed to total. Remainder 1 if
                // rounded off error, like 3333 + 3333 + 3333 = 10000 - 1, the 1. 
                if remainder == 0 || remainder == 1 {
                  Some(payout_object.payout)  // nothing wrong. 
                } else {
                  None  // other remainder means something wrong. 
                }
              }
            })
      });

      let payout = if let Some(payout_option) = payout_option {
        payout_option
      } else {
        // refund buyer
        Promise::new(buyer_id).transfer(u128::from(price));
        return price;  // leave function and return price refunded. 
      };

      // NEAR payouts
      for (receiver_id, amount) in payout {
        Promise::new(receiver_id).transfer(amount.0);
      }

      price  // return price being payed out. 
    }


    #[private]
    #[payable]
    pub fn on_nft_mint(
      &mut self,
      payment_amount: U128,
      nft_contract_id: AccountId,

      token_id: TokenId,
      metadata: TokenMetadata,
      receiver_id: AccountId,
      perpetual_royalties: Option<HashMap<AccountId, u16>>,
      size: Option<usize>,
      refund_to_signer: Option<AccountId>,
    ) -> Promise {
      if is_promise_success() {
        ext_contract::nft_mint(
          token_id,
          metadata,
          receiver_id,
          perpetual_royalties,
          size,
          refund_to_signer,

          nft_contract_id,
          near_to_yoctonear(0.1),
          GAS_FOR_MINTING
        )
      } else {
        Promise::new(env::signer_account_id()).transfer(payment_amount.into())
      }
    }
}

// cross contract call that we call on our own contract. 
/// resolve promise when calling nft_transfer_payout. 
#[ext_contract(ext_self)]
trait ExtSelf {
  fn resolve_purchase(
    &mut self,
    buyer_id: AccountId,
    price: U128,
  ) -> Promise;

  fn on_nft_mint(
    &mut self,
    payment_amount: U128,
    nft_contract_id: AccountId,

    token_id: TokenId,
    metadata: TokenMetadata,
    receiver_id: AccountId,
    perpetual_royalties: Option<HashMap<AccountId, u16>>,
    size: Option<usize>,
    refund_to_signer: Option<AccountId>,
  ) -> Promise;
}
