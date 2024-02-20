#!/usr/bin/env bash

./scripts/build-canister.sh sns_rewards &&
./scripts/generate-did.sh sns_rewards &&
# dfx deploy --network staging sns_rewards --argument '(record {test_mode = true})' --mode reinstall
dfx deploy --network staging sns_rewards --argument '(record {test_mode = true})'
