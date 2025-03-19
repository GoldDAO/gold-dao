#!/usr/bin/env bash

CANISTER=$1
BASE_CANISTER_PATH=$2
INTTEST=$3  # $4 should be changed to $3 since it's the third argument passed

echo "Running prebuild scripts for $CANISTER"
echo "Flags passed $INTTEST"

# Check if INTTEST is set to 'inttest' and convert it to '--features inttest'

if [ "$CANISTER" == "gldt_swap" ]; then
    cargo build --target wasm32-unknown-unknown --target-dir $BASE_CANISTER_PATH/gldt_swap_archive/target --release --locked $INTTEST -p gldt_swap_archive
    ic-wasm $BASE_CANISTER_PATH/gldt_swap_archive/target/wasm32-unknown-unknown/release/gldt_swap_archive.wasm -o $BASE_CANISTER_PATH/gldt_swap_archive/target/wasm32-unknown-unknown/release/gldt_swap_archive.wasm shrink
	  ic-wasm $BASE_CANISTER_PATH/gldt_swap_archive/target/wasm32-unknown-unknown/release/gldt_swap_archive.wasm -o $BASE_CANISTER_PATH/gldt_swap_archive/target/wasm32-unknown-unknown/release/gldt_swap_archive.wasm optimize --inline-functions-with-loops O3
    gzip --no-name -9 -v -c $BASE_CANISTER_PATH/gldt_swap_archive/target/wasm32-unknown-unknown/release/gldt_swap_archive.wasm > $BASE_CANISTER_PATH/gldt_swap_archive/target/wasm32-unknown-unknown/release/gldt_swap_archive_canister.wasm.gz &&
	  gzip -v -t $BASE_CANISTER_PATH/gldt_swap_archive/target/wasm32-unknown-unknown/release/gldt_swap_archive_canister.wasm.gz &&
    mv $BASE_CANISTER_PATH/gldt_swap_archive/target/wasm32-unknown-unknown/release/gldt_swap_archive_canister.wasm.gz $BASE_CANISTER_PATH/gldt_swap/archive/wasm/gldt_swap_archive_canister.wasm.gz
fi


if [ "$CANISTER" == "gldt_stake" ]; then
    ./scripts/build_canister.sh -it sns_rewards &&
    ./scripts/generate_did.sh -it sns_rewards
    cargo build --target wasm32-unknown-unknown --target-dir $BASE_CANISTER_PATH/gldt_stake_archive/target --release --locked $INTTEST -p gldt_stake_archive
    ic-wasm $BASE_CANISTER_PATH/gldt_stake_archive/target/wasm32-unknown-unknown/release/gldt_stake_archive.wasm -o $BASE_CANISTER_PATH/gldt_stake_archive/target/wasm32-unknown-unknown/release/gldt_stake_archive.wasm shrink
	  ic-wasm $BASE_CANISTER_PATH/gldt_stake_archive/target/wasm32-unknown-unknown/release/gldt_stake_archive.wasm -o $BASE_CANISTER_PATH/gldt_stake_archive/target/wasm32-unknown-unknown/release/gldt_stake_archive.wasm optimize --inline-functions-with-loops O3
    gzip --no-name -9 -v -c $BASE_CANISTER_PATH/gldt_stake_archive/target/wasm32-unknown-unknown/release/gldt_stake_archive.wasm > $BASE_CANISTER_PATH/gldt_stake_archive/target/wasm32-unknown-unknown/release/gldt_stake_archive_canister.wasm.gz &&
	  gzip -v -t $BASE_CANISTER_PATH/gldt_stake_archive/target/wasm32-unknown-unknown/release/gldt_stake_archive_canister.wasm.gz &&
    mv $BASE_CANISTER_PATH/gldt_stake_archive/target/wasm32-unknown-unknown/release/gldt_stake_archive_canister.wasm.gz $BASE_CANISTER_PATH/gldt_stake/archive/wasm/gldt_stake_archive_canister.wasm.gz
fi
