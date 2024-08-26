#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="proposal"

if [[ $NETWORK =~ ^(local|staging)$ ]]; then
  TESTMODE=true
elif [[ $NETWORK =~ ^(ic)$ ]]; then
  TESTMODE=false
else
  echo "Error: unknown network for deployment. Found $NETWORK."
  exit 2
fi

ARGUMENTS="(record {
  test_mode = $TESTMODE;
})"

. ./scripts/deploy_backend_canister.sh icp_neuron $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA
