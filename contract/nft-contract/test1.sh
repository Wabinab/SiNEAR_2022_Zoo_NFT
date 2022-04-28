#!/bin/bash

export APPROVAL_NFT_CONTRACT_ID=zoo_nft.wabinab.testnet

bash redeploy.sh
near call $APPROVAL_NFT_CONTRACT_ID nft_mint '{
  "token_id": "approval-token", 
  "metadata": {"title": "Approval Token", 
  "description": "testing out the new approval extension of the standard", 
  "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, 
  "receiver_id": "'$1'"
}' --accountId $1 --amount 0.1

near call $APPROVAL_NFT_CONTRACT_ID nft_approve '{
  "token_id": "approval-token", 
  "account_id": "zoo_marketplace.wabinab.testnet",
  "msg": "Some idea?"
}' --accountId $1 --deposit 0.1

