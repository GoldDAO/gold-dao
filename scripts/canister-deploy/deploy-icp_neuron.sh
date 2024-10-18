#!/usr/bin/env bash

NETWORK=$1

DEPLOYMENT_VIA="proposal"

. ./scripts/extract_commit_tag_data_and_commit_sha.sh icp_neuron $NETWORK

if [[ $REINSTALL == "reinstall" ]]; then

  if [[ $NETWORK =~ ^(local|staging)$ ]]; then
    TESTMODE=true
  elif [[ $NETWORK =~ ^(ic)$ ]]; then
    TESTMODE=false
  else
    echo "Error: unknown network for deployment. Found $NETWORK."
    exit 2
  fi

  ARGUMENTS="(variant { Init = record {
    test_mode = $TESTMODE;
    version = $BUILD_VERSION;
    commit_hash = \"$COMMIT_SHA\";
  }})"

else
  ARGUMENTS="(variant { Upgrade = record {
    version = $BUILD_VERSION;
    commit_hash = \"$COMMIT_SHA\";
  }})"
fi

. ./scripts/deploy_backend_canister.sh icp_neuron $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA $VERSION $REINSTALL
