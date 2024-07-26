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


. ./scripts/frontend-deploy/sns_prepare_assets.sh
. ./scripts/frontend-deploy/sns_commit_assets.sh $CANISTER_NAME $NETWORK $BATCH_ID $EVIDENCE
