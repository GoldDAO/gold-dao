#!/usr/bin/env bash

./scripts/build_canister.sh token_metrics &&
./scripts/generate_did.sh token_metrics &&
# dfx deploy --network staging token_metrics --argument '(record {test_mode = true})' --mode reinstall
dfx deploy --network staging token_metrics --argument '(record {test_mode = true})'
