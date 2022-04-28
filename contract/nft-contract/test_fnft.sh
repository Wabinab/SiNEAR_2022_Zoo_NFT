export APPROVAL_NFT_CONTRACT_ID=zoo_nft.wabinab.testnet
export TOKEN_ID="multi-token"

# bash redeploy.sh
near call $APPROVAL_NFT_CONTRACT_ID nft_mint '{
  "token_id": "'$TOKEN_ID'", 
  "metadata": {"title": "FNFT Token", 
  "description": "try to add something more descriptive. ", 
  "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"
  }, 
  "receiver_id": "wabinab.testnet",
  "size": 4
}' --accountId wabinab.testnet --amount 0.1

near view $APPROVAL_NFT_CONTRACT_ID nft_token '{"token_id": "'$TOKEN_ID'"}'

# ========================================================
export TOKEN_ID="single-token"

near call $APPROVAL_NFT_CONTRACT_ID nft_mint '{
  "token_id": "'$TOKEN_ID'", 
  "metadata": {"title": "FNFT Token", 
  "description": "try to add something more descriptive. ", 
  "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"
  }, 
  "receiver_id": "wabinab.testnet",
}' --accountId wabinab.testnet --amount 0.1

near view $APPROVAL_NFT_CONTRACT_ID nft_token '{"token_id": "'$TOKEN_ID'"}'