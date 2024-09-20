#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="direct"

. ./scripts/extract_commit_tag_data_and_commit_sha.sh super_stats_v3 $NETWORK

if [[ $NETWORK =~ ^(local|staging)$ ]]; then
  TESTMODE=true
  ADMIN=$(dfx identity get-principal)
elif [[ $NETWORK =~ ^(ic)$ ]]; then
  TESTMODE=false
  ADMIN=$(dfx identity get-principal)
else
  echo "Error: unknown network for deployment. Found $NETWORK."
  exit 2
fi

ARGUMENTS="(record {
  test_mode = $TESTMODE;
  admin = \"$ADMIN\"
})"

. ./scripts/deploy_backend_canister.sh super_stats_v3 $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA

MODE=""

if [ "$MODE" = "reinstall" ]; then
  TOKEN_METRICS_CANISTER_ID=$(dfx canister id token_metrics --network $NETWORK)
  LEDGER_CANISTER_ID=$(dfx canister id sns_ledger --network ic) # pass prod canister
  INIT_ARGUMENTS="(record {
      target = record {
          target_ledger = \"$LEDGER_CANISTER_ID\";
          hourly_size = 24;
          daily_size = 30;
      };
      index_type = variant { \"DfinityIcrc2\" }
  })"
  echo "Initialising canister"
  dfx canister call super_stats_v3 --network $NETWORK init_target_ledger "$INIT_ARGUMENTS"
  echo "Start processing timer"
  dfx canister call super_stats_v3 --network $NETWORK start_processing_timer '(60: nat64)'
  echo "Adding authorized principals"
  dfx canister call super_stats_v3 --network $NETWORK add_authorised "2vxsx-fae"
  dfx canister call super_stats_v3 --network $NETWORK add_authorised "$TOKEN_METRICS_CANISTER_ID"
fi
