#!/usr/bin/env bash

./scripts/build-canister.sh sns_rewards &&
./scripts/generate-did.sh sns_rewards &&
# dfx deploy --network staging sns_rewards --mode reinstall --argument '(record {sns_governance_canister = principal "tr3th-kiaaa-aaaaq-aab6q-cai"})'
dfx deploy --network staging sns_rewards --argument '(record {sns_governance_canister = principal "tr3th-kiaaa-aaaaq-aab6q-cai"})'
