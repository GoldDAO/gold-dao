#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown --target-dir canister/$1/target --release --locked -p $1 &&
ic-wasm canister/$1/target/wasm32-unknown-unknown/release/$1.wasm -o canister/$1/target/wasm32-unknown-unknown/release/${1}.wasm shrink -k
ic-wasm canister/$1/target/wasm32-unknown-unknown/release/$1.wasm -o canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm optimize --inline-functions-with-loops O3
gzip -9 -v -c canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm >canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm.gz &&
gzip -v -t canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm.gz &&
echo "$1 successfully built, optimized and compressed"
