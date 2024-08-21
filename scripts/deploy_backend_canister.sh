#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

show_help() {
  cat << EOF
backend canister deployment script.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.
The NETWORK argument should preferably be passed from the env variable that was previously defined
by the pre-deploy script (using the dot notation, or inside a macro deploy script).

The canister will always be reinstalled locally, and only upgraded in staging and production (ic).

Usage:
  scripts/deploy-backend-canister.sh [options] <CANISTER> <NETWORK> <ARGUMENTS> <MODE>

Options:
  -h, --help        Show this message and exit
  -r, --reinstall   Completely reinstall the canister, instead of simply upgrade it
EOF
}

if [[ $# -gt 3 ]]; then
  while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do
    case $1 in
      -h | --help )
        show_help
        exit
        ;;
      -r | --reinstall )
        REINSTALL="--mode reinstall"
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
else
  echo "Error: missing <CANISTER> <NETWORK> <ARGUMENTS> <MODE> arguments"
  exit 1
fi

CANISTER=$1
NETWORK=$2
ARGUMENTS=$3
MODE=$4


echo -e "CANISTER: $CANISTER \nNETWORK: $NETWORK \nARGUMENTS: $ARGUMENTS \nMODE: $MODE \nTAG: $CI_COMMIT_TAG"

if [[ ! $NETWORK =~ ^(local|staging|ic)$ ]]; then
  echo "Error: unknown network for deployment"
  exit 2
fi

if [[ $NETWORK == ic && ! $CI_COMMIT_TAG =~ ^($CANISTER-v[0-9]+\.[0-9]+\.[0-9]+)$ ]]; then
  echo "Error: Enter valid commit tag to deploy to production"
  exit 2
fi

if [[ $MODE == "direct" ]]; then
  echo "Deploying $CANISTER directly via dfx."
  dfx deploy $CANISTER --network $NETWORK ${REINSTALL} --argument "$ARGUMENTS" -y
elif [[ $MODE == "proposal" ]]; then
  echo "Deploying $CANISTER via SNS proposal."
  if [[ $NETWORK == "ic" ]]; then
    PROPOSER=$SNS_PROPOSER_NEURON_ID_PRODUCTION
    UPGRADEVERSION="${CI_COMMIT_TAG#*-v}"
  else
    PROPOSER=$SNS_PROPOSER_NEURON_ID_STAGING
    UPGRADEVERSION=$CI_COMMIT_SHORT_SHA
  fi
  . scripts/extract_version_and_commit_sha.sh $CANISTER
  . scripts/prepare_sns_canister_ids.sh $NETWORK && \
  . scripts/prepare_proposal_summary.sh $CANISTER $VERSION backend && \

  echo "Sending proposal from proposer id $PROPOSER"

  quill sns --canister-ids-file sns_canister_ids.json make-upgrade-canister-proposal $PROPOSER \
    --pem-file $PEM_FILE \
    --canister-upgrade-arg "$ARGUMENTS" \
    --target-canister-id $(cat canister_ids.json | jq -r .$CANISTER.$NETWORK) \
    --wasm-path backend/canisters/$CANISTER/target/wasm32-unknown-unknown/release/${CANISTER}_canister.wasm.gz \
    --title "Upgrade $CANISTER to ${UPGRADEVERSION}" \
    --url ${DETAILS_URL} --summary-path proposal.md | quill send --yes -
else
  echo "Error: invalid deployment mode. Needs to be 'direct' or 'proposal'."
  exit 2
fi

return
