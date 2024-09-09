#!/usr/bin/env bash

CANISTER=$1
NETWORK=$2
ARGUMENTS=$3
DEPLOYMENT_VIA=$4


echo -e "CANISTER: $CANISTER \nNETWORK: $NETWORK \nARGUMENTS: $ARGUMENTS \nDEPLOYMENT_VIA: $DEPLOYMENT_VIA \nTAG: $CI_COMMIT_TAG"

# only allow deployments to local, staging and ic
if [[ ! $NETWORK =~ ^(local|staging|ic)$ ]]; then
  echo "Error: unknown network for deployment"
  exit 2
fi

# if deployment is to production/ic, the CI_COMMIT_TAG needs to match the expected pattern
if [[ $NETWORK == ic && ! $CI_COMMIT_TAG =~ ^($CANISTER-v[0-9]+\.[0-9]+\.[0-9]+)$ ]]; then
  echo "Error: Enter valid commit tag to deploy to productio. Received $CI_COMMIT_TAG."
  exit 2
fi

if [[ $DEPLOYMENT_VIA == "direct" ]]; then

  echo "Deploying $CANISTER directly via dfx."

  dfx deploy $CANISTER --network $NETWORK ${REINSTALL} --argument "$ARGUMENTS" -y

elif [[ $DEPLOYMENT_VIA == "proposal" ]]; then

  echo "Deploying $CANISTER via SNS proposal on $NETWORK."

  if [[ $NETWORK == "ic" ]]; then
    PROPOSER=$SNS_PROPOSER_NEURON_ID_PRODUCTION
    UPGRADEVERSION="${CI_COMMIT_TAG#*-v}"
  else
    PROPOSER=$SNS_PROPOSER_NEURON_ID_STAGING
    UPGRADEVERSION=$CI_COMMIT_SHORT_SHA
  fi

  # Extract version info and commit sha from CICD pipeline variables
  . scripts/extract_version_and_commit_sha.sh $CANISTER $NETWORK
  if [ $? -ne 0 ]; then
    echo "Error in extract_version_and_commit_sha.sh"
    exit 1
  fi

  # Prepare prososal summary
  . scripts/prepare_proposal_summary.sh $CANISTER $VERSION backend
  if [ $? -ne 0 ]; then
    echo "Error in prepare_proposal_summary.sh"
    exit 1
  fi

  # Prepare SNS canister ids file needed for quill command
  . scripts/prepare_sns_canister_ids.sh $NETWORK

  echo "Sending proposal from proposer id $PROPOSER with following arguments: \n $ARGUMENTS"

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
