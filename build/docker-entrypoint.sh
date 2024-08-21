#!/bin/sh

# Debug: Print the CANISTER_NAME to ensure it's being passed correctly
echo "----------------------------------------------------"
echo "Building canister $CANISTER_NAME"
echo "----------------------------------------------------"

# Your existing commands
./scripts/build_canister.sh --wasmonly $CANISTER_NAME
./scripts/generate_did.sh $CANISTER_NAME
./scripts/build_canister.sh $CANISTER_NAME
