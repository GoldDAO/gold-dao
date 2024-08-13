#!/usr/bin/env bash

./scripts/build-canister.sh token_metrics &&
./scripts/generate-did.sh token_metrics &&
# dfx deploy --network staging token_metrics --argument '(record {test_mode = true})' --mode reinstall
dfx deploy --network staging token_metrics --argument '(record {test_mode = true})'
