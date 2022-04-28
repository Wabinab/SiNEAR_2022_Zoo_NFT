#!/bin/bash

bash build.sh
export CONTRACT=zoo_marketplace.wabinab.testnet

near delete $CONTRACT wabinab.testnet
near create-account $CONTRACT --masterAccount wabinab.testnet --initialBalance 4.5

near deploy --accountId $CONTRACT --wasmFile res/output_s.wasm

near call $CONTRACT new '{"owner_id": "'$CONTRACT'"}' --accountId $CONTRACT