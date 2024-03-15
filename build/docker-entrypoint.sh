#!/bin/sh

# Debug: Print the CANISTER_NAME to ensure it's being passed correctly
echo "----------------------------------------------------"
echo "Building canister $CANISTER_NAME"
echo "----------------------------------------------------"

# Your existing commands
./scripts/build-canister.sh --wasmonly $CANISTER_NAME
./scripts/generate-did.sh $CANISTER_NAME
./scripts/build-canister.sh $CANISTER_NAME
