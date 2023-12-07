#!/usr/bin/env bash

# Following this guide, only one command should be needed to submit the SNS proposal
# https://internetcomputer.org/docs/current/developer-docs/integrations/sns/launching/launch-steps-1proposal#3-submit-nns-proposal-to-create-sns

# Make sure to have your identity selected (dfx identity use <name>), which controls your neuron, to run the command from.
# At least 10 ICP need to be staked in the neuron to be able to submit a proposal
NEURON_ID="9409181943169202849" # dedicate neuron for this proposal

dfx sns propose --network ic --neuron $NEURON_ID sns_init.yaml
