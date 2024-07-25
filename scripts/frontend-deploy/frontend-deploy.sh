#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

show_help() {
  cat << EOF
Frontend canister deploy script via SNS proposal.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'preprod', 'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.

Usage:
  deploy-frontend.sh [options] <CANISTER-NAME> <NETWORK>

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
  echo "Error: missing <CANISTER-NAME> and/or <NETWORK> argument"
  exit 1
fi


CANISTER_NAME=$1
NETWORK=$2

if [[ ! $NETWORK =~ ^(staging|ic)$ ]]; then
  echo "Error: unknown network for deployment via SNS proposal"
  exit 2
fi


function prepare_assets() {
  LOG_FILE=console.log

  # ENV=$NETWORK dfx deploy --network staging --by-proposal $CANISTER_NAME 2>&1 | tee $LOG_FILE

  echo "Last line: $(tail -n 1 $LOG_FILE)"

  export BATCH_ID=$(tail -n 1 $LOG_FILE | awk '{print $5}')
  export EVIDENCE=$(tail -n 1 $LOG_FILE | awk '{print $8}' | sed "s/\.//" )

  echo "Batch number: $BATCH_ID, Evidence: $EVIDENCE"

  if [[ $BATCH_ID =~ "^[0-9]+$" ]]; then
    echo "Valid batch id: $BATCH_ID"
  else
    echo "Invalid batch id: $BATCH_ID"
    exit 1
  fi

  if [[ $EVIDENCE =~ "^[0-9a-f]{64}$" ]]; then
    echo "Valid evidence: $EVIDENCE"
  else
    echo "Invalid evidence: $EVIDENCE"
    exit 1
  fi
}


function commit_assets() {
  echo "Committing batch $BATCH_ID via proposal"
}



prepare_assets
. commit_assets $CANISTER_NAME $NETWORK $BATCH_ID $EVIDENCE
