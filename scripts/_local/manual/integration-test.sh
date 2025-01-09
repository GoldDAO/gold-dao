#!/usr/bin/env bash

# if we're building gldt_swap then we must build the archive canister  aswell
if [ "$1" == "gldt_stake" ]; then
   ./scripts/build_canister.sh -it sns_rewards &&
    ./scripts/generate_did.sh -it sns_rewards
fi

./scripts/build_canister.sh --integration-test $1
./scripts/generate_did.sh --integration-test $1 

cargo test --features inttest "${1}_suite"