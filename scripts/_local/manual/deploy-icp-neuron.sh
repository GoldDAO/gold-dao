#!/usr/bin/env bash

./scripts/build-canister.sh icp_neuron &&
./scripts/generate-did.sh icp_neuron &&
./scripts/build-canister.sh icp_neuron &&

# dfx deploy --network staging icp_neuron --argument '(opt record {test_mode = true} )' --mode reinstall
dfx deploy --network staging icp_neuron --argument '(null)'

# ./scripts/sns_testing/upgrade-canister.sh
