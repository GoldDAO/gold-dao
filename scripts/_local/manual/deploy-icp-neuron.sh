#!/usr/bin/env bash

./scripts/build_canister.sh icp_neuron &&
./scripts/generate_did.sh icp_neuron &&
./scripts/build_canister.sh icp_neuron &&

# dfx deploy --network staging icp_neuron --argument '(opt record {test_mode = true} )' --mode reinstall
dfx deploy --network staging icp_neuron --argument '(null)'

# ./scripts/sns_testing/upgrade-canister.sh
