#!/usr/bin/env bash

./scripts/build-canister.sh icp_neuron &&
./scripts/generate-did.sh icp_neuron &&
# dfx deploy --network staging icp_neuron --argument '(record {test_mode = true; rewards_recipients = vec {}})' --mode reinstall
dfx deploy --network staging icp_neuron --argument '(record {test_mode = true; rewards_recipients = vec {}})'
