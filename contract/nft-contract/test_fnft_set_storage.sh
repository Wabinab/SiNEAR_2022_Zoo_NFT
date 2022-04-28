export CONTRACT=zoo_nft.wabinab.testnet
export OWNER_ID=wabinab.testnet
export TOKEN_ID="multi-token"

bash redeploy.sh
near call $CONTRACT nft_mint '{
  "token_id": "'$TOKEN_ID'", 
  "metadata": {"title": "FNFT Token", 
  "description": "try to add something more descriptive. ", 
  "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"
  }, 
  "receiver_id": "'$OWNER_ID'",
  "size": 4
}' --accountId $OWNER_ID --amount 0.1

near view $CONTRACT nft_token '{"token_id": "'$TOKEN_ID'"}'

near call $CONTRACT set_accounts '{
  "token_id": "'$TOKEN_ID'",
  "share_accounts": ["somebodyelse.testnet", "'$OWNER_ID'", "'$OWNER_ID'"]
}' --accountId $OWNER_ID --amount 0.1

near view $CONTRACT nft_token '{"token_id": "'$TOKEN_ID'"}'