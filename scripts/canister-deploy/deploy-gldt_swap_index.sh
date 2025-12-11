#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="proposal"

# Extract commit and version info
. ./scripts/extract_commit_tag_data_and_commit_sha.sh gldt_swap_index $NETWORK

if [[ ${REINSTALL:-} == "reinstall" ]]; then

  # Determine test mode and authorized principals based on network
  if [[ $NETWORK =~ ^(local|staging)$ ]]; then
      TESTMODE=true
      AUTHORIZED_PRINCIPALS="vec {
        principal \"465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae\";
        principal \"$(dfx canister id --network $NETWORK sns_governance)\";
      }"
  elif [[ $NETWORK =~ ^(ic)$ ]]; then
      TESTMODE=false
      AUTHORIZED_PRINCIPALS="vec {
        principal \"$(dfx canister id --network $NETWORK sns_governance)\";
      }"
  else
      echo "Error: unknown network '$NETWORK'."
      exit 2
  fi

  # Ledger canister ID
  LEDGER_CANISTER_ID="$(dfx canister id --network $NETWORK gldt_swap)"

  # Init arguments
  ARGUMENTS="(variant { Init = record {
    test_mode = $TESTMODE;
    commit_hash = \"$COMMIT_SHA\";
    version = $BUILD_VERSION;
    authorized_principals = $AUTHORIZED_PRINCIPALS;
    ledger_canister_id = principal \"$LEDGER_CANISTER_ID\";
  }})"

else
  # Upgrade arguments
  ARGUMENTS="(variant { Upgrade = record {
    version = $BUILD_VERSION;
    commit_hash = \"$COMMIT_SHA\";
  }})"
fi

# Print arguments for debugging
echo "$ARGUMENTS"

. ./scripts/deploy_backend_canister.sh gldt_swap_index $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA $VERSION $REINSTALL
