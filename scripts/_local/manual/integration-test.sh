#!/usr/bin/env bash

# if we're building gldt_swap then we must build the archive canister  aswell
if [ "$1" == "gldt_swap" ]; then
    ./scripts/build_canister.sh --integration-test gldt_swap_archive
    mkdir -p ./backend/canisters/gldt_swap/archive/wasm
    candid-extractor "backend/canisters/gldt_swap_archive/target/wasm32-unknown-unknown/release/gldt_swap_archive_canister.wasm" > ./backend/canisters/gldt_swap/api_archive/can.did
    mv ./backend/canisters/gldt_swap_archive/target/wasm32-unknown-unknown/release/gldt_swap_archive_canister.wasm.gz ./backend/canisters/gldt_swap/archive/wasm/gldt_swap_archive_canister.wasm.gz   
fi

./scripts/build_canister.sh --integration-test $1 
./scripts/generate_did.sh --integration-test $1 

cargo test "${1}_suite"