#!/usr/bin/env bash

show_help() {
  cat << EOF
Submit a new SNS proposal for an asset canister upgrade.  
PROPOSAL must be a string containing all the proposal details.
See 

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
  --network NETWORK The network where the commands will be executed (default is 'local')
EOF
}
export NETWORK="local"

if [[ $# -gt 4 ]]; then
  while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do
    case $1 in
      -h | --help )
        show_help
        exit
        ;;
      --network )
        shift; export NETWORK=$1
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
else
  echo "Error: invalid argument number"
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
