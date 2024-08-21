#!/bin/bash

show_help() {
  cat << EOF
SNS canister ids JSON file generation script.

Usage:
  scripts/prepare_sns_canister_ids.sh <NETWORK>

Options:
  -h, --help        Show this message and exit
EOF
}

if [[ $# -gt 0 ]]; then
  while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do
    case $1 in
      -h | --help )
        show_help
        exit
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
else
  echo "Error: missing <NETWORK> argument"
  exit 1
fi

if [[ ! $1 =~ ^(staging|ic)$ ]]; then
  echo "Error: unknown network for SNS preparation. Only staging and ic are valid."
  exit 2
fi

INPUT_FILE="canister_ids.json"
OUTPUT_FILE="sns_canister_ids.json"

jq --arg choice $1 '{
  governance_canister_id: .sns_governance[$choice],
  index_canister_id: .sns_index[$choice],
  ledger_canister_id: .sns_ledger[$choice],
  root_canister_id: .sns_root[$choice],
  swap_canister_id: .sns_swap[$choice],
  dapp_canister_id_list: []
}' $INPUT_FILE > $OUTPUT_FILE

echo "SNS canister ids file generation complete based on $1. Output saved to $OUTPUT_FILE."


echo "
******************************************
Canister ids:
"

cat $OUTPUT_FILE

echo "
******************************************
"
