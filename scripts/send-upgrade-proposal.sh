#!/usr/bin/env bash

show_help() {
  cat << EOF
Usage:
  scripts/send-pupgrade-proposal.sh [options] <PROPOSER_NEURON_ID> <CANISTER_ID> <WASM_PATH> <TITLE> <URL>

Arguments:
  PROPOSER_NEURON_ID  Self explanatory
  CANISTER_ID         The asset canister that is required to be upgraded
  WASM_PATH           Relative or absolute path to the wasm that would be deployed
  TITLE               The upgrade proposal title
  URL                 An URL to a detailed description (blog post, etc) for this upgrade

Options:
  -h, --help        Show this message and exit
EOF
}

if [[ $# -lt 4 ]]; then
  echo "Error: invalid argument number\n"
  show_help
  exit 1
fi

export PROPOSER_NEURON_ID=$1
export CANISTER_ID=$2
export WASM_PATH=$3
export UPGRADE_TITLE=$4
export DETAILS_URL=$5

quill sns make-upgrade-canister-proposal $PROPOSER_NEURON_ID \
  --target-canister-id $CANISTER_ID --wasm-path $WASM_PATH \
  --title $UPGRADE_TITLE --url $DETAILS_URL > msg.json && \
quill send --yes msg.json; rm msg.json
