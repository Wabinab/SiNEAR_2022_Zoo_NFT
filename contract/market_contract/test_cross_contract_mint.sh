#!/bin/bash

export MARKET_CONTRACT=zoo_marketplace.wabinab.testnet
export NFT_CONTRACT=zoo_nft.wabinab.testnet
export SELLER_ID=somebodyelse.testnet  # another of my account. 
export TOKEN_ID="zoo_movie_ticket_for_four"

# Simulate NFT cost 1 NEAR, so total paying 1.1 NEAR (plus storage). 
# We don't add perpetual royalties for simplicity. 
# Receiver ID is signer, so that's eliminated. 
# size is 4, since ticket for 4. 
# Attached gas is 70 TGas just in case. 
# Must attach about exact, accurate to 0.001 N. 

near call $MARKET_CONTRACT pay_and_mint '{
  "nft_contract_id": "'$NFT_CONTRACT'",
  "price": "1000000000000000000000000",
  "nft_seller_id": "'$SELLER_ID'",
  "token_id": "'$TOKEN_ID'",
  "metadata": {
    "title": "Zoo Movie Ticket",
    "description": "Some description",
    "media": "https://www.google.com"
  },
  "size": 4
}' --accountId wabinab.testnet --gas=70000000000000 --amount=1.1


# Check for minted NFT. 
near view $NFT_CONTRACT nft_token '{
  "token_id": "'$TOKEN_ID'"
}'

# ========================================================================

# Check explorer for how much gas is saved with unsafe version. 
# Unsafe MUST attach exact amount. 

export TOKEN_ID="zoo_movie_ticket_for_four_unsafe"

near call $MARKET_CONTRACT pay_and_mint_unsafe '{
  "nft_contract_id": "'$NFT_CONTRACT'",
  "price": "1000000000000000000000000",
  "nft_seller_id": "'$SELLER_ID'",
  "token_id": "'$TOKEN_ID'",
  "metadata": {
    "title": "Zoo Movie Ticket",
    "description": "Some description",
    "media": "https://www.google.com"
  },
  "size": 4
}' --accountId wabinab.testnet --gas=70000000000000 --amount=1.1


# Check for minted NFT. 
near view $NFT_CONTRACT nft_token '{
  "token_id": "'$TOKEN_ID'"
}'