#!/bin/bash
set -e

export WASM=zoo_nft.wasm

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/$WASM res/
wasm-opt -Os -o res/output_s.wasm res/$WASM
ls res -lh