#!/bin/bash

bash build.sh
export CONTRACT=greeter.wabinab.testnet

near delete $CONTRACT wabinab.testnet
near create-account $CONTRACT --masterAccount wabinab.testnet --initialBalance 3

near deploy --accountId $CONTRACT --wasmFile res/output_s.wasm