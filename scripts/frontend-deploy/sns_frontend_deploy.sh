#!/usr/bin/env bash

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

# Extract version info and commit sha from CICD pipeline variables
. ./scripts/extract_commit_tag_data_and_commit_sha.sh $CANISTER_NAME $NETWORK

if [ $? -ne 0 ]; then
  echo "Error in extract_commit_tag_data_and_commit_sha.sh"
  exit 1
fi

# Compiles the frontend code and uploads the assets to canister memory in prepare stage. Needs to be committed in the next stage
. ./scripts/frontend-deploy/sns_prepare_assets.sh $CANISTER_NAME $NETWORK

if [ $? -ne 0 ]; then
  echo "Error in sns_prepare_assets.sh"
  exit 1
fi

# Commits the previously prepared assets and creates a proposal on the SNS to vote about the upgrade.
./scripts/frontend-deploy/sns_commit_assets.sh $CANISTER_NAME $NETWORK $BATCH_ID $EVIDENCE $VERSION $COMMIT_SHA

if [ $? -ne 0 ]; then
  echo "Error in sns_commit_assets.sh"
  exit 1
fi
