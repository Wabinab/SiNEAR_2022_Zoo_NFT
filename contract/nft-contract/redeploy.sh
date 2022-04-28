#!/bin/bash

bash build.sh
export CONTRACT=zoo_nft.wabinab.testnet

near delete $CONTRACT wabinab.testnet
near create-account $CONTRACT --masterAccount wabinab.testnet --initialBalance 4.5

near deploy --accountId $CONTRACT --wasmFile res/output_s.wasm

near call $CONTRACT new_default_meta '{"owner_id": "'$CONTRACT'"}' --accountId $CONTRACT