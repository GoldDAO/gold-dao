#!/usr/bin/env bash

CANISTER=$1
NETWORK=$2
ARGUMENTS=$3
DEPLOYMENT_VIA=$4
VERSION=$5
REINSTALL=$6


echo -e "CANISTER: $CANISTER \nNETWORK: $NETWORK \nARGUMENTS: $ARGUMENTS \nDEPLOYMENT_VIA: $DEPLOYMENT_VIA \nTAG: $CI_COMMIT_TAG"

# only allow deployments to local, staging and ic
if [[ ! $NETWORK =~ ^(local|staging|ic)$ ]]; then
  echo "Error: unknown network for deployment"
  exit 2
fi

# if deployment is to production/ic, the CI_COMMIT_TAG needs to match the expected pattern
if [[ $NETWORK == ic && ! $CI_COMMIT_TAG =~ ^($CANISTER-v[0-9]+\.[0-9]+\.[0-9]+(-reinstall)?)$ ]]; then
  echo "Error: Enter valid commit tag to deploy to production. Received $CI_COMMIT_TAG."
  exit 2
fi

WASM_PATH=backend/canisters/$CANISTER/target/wasm32-unknown-unknown/release/${CANISTER}_canister.wasm.gz
CHECKSUM_PATH=backend/canisters/$CANISTER/target/wasm32-unknown-unknown/release/${CANISTER}.checksum

# The wasm must be the one the staging or production build of this pipeline
# made. "integration-test build" writes its wasm (built with the inttest
# feature, which adds test endpoints) to the same path, and a deploy job that
# also downloads the artifacts of that job installs the test wasm. That happened
# in origyn-sns from April to November 2026, after the "needs" line of its
# production deploy was commented out. The staging and production builds record
# the sha256 of the wasm in the checksum file (scripts/build_canister.sh
# --checksum); the integration-test build writes no checksum file. Local
# deployments use a local build and are not checked.
if [[ $NETWORK != "local" ]]; then
  if [[ ! -f $WASM_PATH || ! -f $CHECKSUM_PATH ]]; then
    echo "Error: $WASM_PATH or $CHECKSUM_PATH is missing. The deploy job needs the artifacts of the $NETWORK build of $CANISTER."
    exit 1
  fi
  EXPECTED_SHA256=$(awk -v wasm="$WASM_PATH" '$2 == wasm { print $1 }' "$CHECKSUM_PATH")
  ACTUAL_SHA256=$(sha256sum "$WASM_PATH" | cut -d ' ' -f 1)
  if [[ -z $EXPECTED_SHA256 || $EXPECTED_SHA256 != "$ACTUAL_SHA256" ]]; then
    echo "Error: $WASM_PATH is not the wasm of the $NETWORK build of $CANISTER."
    echo "  sha256 of the file:       $ACTUAL_SHA256"
    echo "  sha256 the build recorded: ${EXPECTED_SHA256:-none, the checksum file has no line for this file}"
    echo "Check the \"needs\" of this deploy job: it must not get the artifacts of \"integration-test build\"."
    exit 1
  fi
  echo "$WASM_PATH is the wasm of the $NETWORK build, sha256 $ACTUAL_SHA256"
fi

if [[ $DEPLOYMENT_VIA == "direct" || $NETWORK != "ic" ]]; then

  if [[ $REINSTALL == "reinstall" ]]; then
    echo "Reinstalling $CANISTER directly via dfx with arguments: $ARGUMENTS"
    dfx canister install $CANISTER \
      -q \
      --network $NETWORK \
      --mode reinstall \
      --argument "$ARGUMENTS" \
      --wasm "$WASM_PATH" \
      -y
  else
    echo "Upgrading $CANISTER directly via dfx with arguments: $ARGUMENTS"

    echo "Attemping to stop canister $CANISTER";
    dfx canister stop $CANISTER --network $NETWORK;

    echo "Attemping to install canister $CANISTER"
    dfx canister install $CANISTER \
      -q \
      --network $NETWORK \
      --mode upgrade \
      --argument "$ARGUMENTS" \
      --wasm "$WASM_PATH" \
      -y


    echo "Attemping to start canister $CANISTER";
    dfx canister start $CANISTER --network $NETWORK;
  fi

elif [[ $DEPLOYMENT_VIA == "proposal" ]]; then

  INSTALL_MODE="upgrade"

  if [[ $REINSTALL == "reinstall" ]]; then
    INSTALL_MODE="reinstall"
  fi

  echo "Deploying $CANISTER via SNS proposal on $NETWORK to $INSTALL_MODE the canister."

  PROPOSER=$SNS_PROPOSER_NEURON_ID_PRODUCTION

  # if [[ $NETWORK == "ic" ]]; then
    # PROPOSER=$SNS_PROPOSER_NEURON_ID_PRODUCTION
    # UPGRADEVERSION="${CI_COMMIT_TAG#*-v}"
  # else
    # PROPOSER=$SNS_PROPOSER_NEURON_ID_STAGING
    # UPGRADEVERSION=$CI_COMMIT_SHORT_SHA
  # fi

  # # Extract version info and commit sha from CICD pipeline variables
  # . scripts/extract_commit_tag_data_and_commit_sha.sh $CANISTER $NETWORK
  # if [ $? -ne 0 ]; then
  #   echo "Error in extract_commit_tag_data_and_commit_sha.sh"
  #   exit 1
  # fi

  # Prepare prososal summary
  . scripts/prepare_proposal_summary.sh $CANISTER $VERSION backend
  if [ $? -ne 0 ]; then
    echo "Error in prepare_proposal_summary.sh"
    exit 1
  fi

  # Prepare SNS canister ids file needed for quill command
  . scripts/prepare_sns_canister_ids.sh $NETWORK

  echo "Sending proposal from proposer id $PROPOSER with following arguments: $ARGUMENTS"

  quill sns --canister-ids-file sns_canister_ids.json make-upgrade-canister-proposal $PROPOSER \
    --pem-file $PEM_FILE \
    --canister-upgrade-arg "$ARGUMENTS" \
    --mode $INSTALL_MODE \
    --target-canister-id $(cat canister_ids.json | jq -r .$CANISTER.$NETWORK) \
    --wasm-path "$WASM_PATH" \
    --title "Upgrade $CANISTER to version $VERSION" \
    --url ${DETAILS_URL} --summary-path proposal.md | quill send --yes - 2>&1 | tee console.log

  # quill exits with 0 even if governance answers with an error. It prints
  # "Successfully created new proposal with ID <n>" when the proposal exists and
  # "SNS governance error: ..." when governance refused it.
  if ! grep -q "Successfully created new proposal" console.log; then
    echo "Error: SNS governance did not accept the proposal to $INSTALL_MODE $CANISTER."
    exit 1
  fi
else
  echo "Error: invalid deployment mode. Needs to be 'direct' or 'proposal'."
  exit 2
fi
