#!/usr/bin/env bash

show_help() {
  cat << EOF
Frontend canister deploy script via SNS proposal.
Must be run from the repository's root folder. The networks are 'staging' and 'ic'.

The built frontend is uploaded to the asset canister as a proposed batch, then a
proposal is submitted that asks SNS governance to commit that batch. The new
version goes live when the proposal is adopted.

The frontend needs an entry in scripts/frontend-deploy/frontend_config.json.

Usage:
  sns_frontend_deploy.sh [options] <CANISTER-NAME> <NETWORK>

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

# Extract version info and commit sha from CICD pipeline variables
. ./scripts/extract_commit_tag_data_and_commit_sha.sh $CANISTER_NAME $NETWORK

if [ $? -ne 0 ]; then
  echo "Error in extract_commit_tag_data_and_commit_sha.sh"
  exit 1
fi

# Stops here, before anything is uploaded, if the proposal could not be submitted afterwards.
./scripts/frontend-deploy/sns_check_inputs.sh $CANISTER_NAME $NETWORK $VERSION

if [ $? -ne 0 ]; then
  echo "Error in sns_check_inputs.sh"
  exit 1
fi

# Uploads the built assets to the canister as a proposed batch. Exports BATCH_ID and EVIDENCE.
. ./scripts/frontend-deploy/sns_prepare_assets.sh $CANISTER_NAME $NETWORK

if [ $? -ne 0 ]; then
  echo "Error in sns_prepare_assets.sh"
  exit 1
fi

# Submits the SNS proposal that commits the proposed batch.
./scripts/frontend-deploy/sns_commit_assets.sh $CANISTER_NAME $NETWORK $BATCH_ID $EVIDENCE $VERSION $COMMIT_SHA

if [ $? -ne 0 ]; then
  echo "Error in sns_commit_assets.sh"
  exit 1
fi
