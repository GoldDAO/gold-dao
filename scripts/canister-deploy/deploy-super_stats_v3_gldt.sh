#!/usr/bin/env bash

NETWORK=$1

####################################################
# MANUAL DEPLOYMENT PROCESS, NOT VIA PIPELINE
####################################################

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

./scripts/build_canister.sh super_stats_v3
# 1. run build stage on CICD pipeline (can be develop/staging pipeline)
# 2. download wasm from pipeline for super_stats_v3 and paste into the wasm folder used below
# 3. deploy using manual command

dfx canister install super_stats_v3_gldt \
      --network $NETWORK \
      --argument "$ARGUMENTS" \
      --mode reinstall \
      --wasm backend/canisters/super_stats_v3/target/wasm32-unknown-unknown/release/super_stats_v3_canister.wasm.gz

# # 4. run init calls below

LEDGER_CANISTER_ID=$(dfx canister id gldt_ledger --network $NETWORK)
INIT_ARGUMENTS="(record {
    target = record {
        target_ledger = \"$LEDGER_CANISTER_ID\";
        hourly_size = 24;
        daily_size = 30;
    };
    index_type = variant { \"DfinityIcrc2\" }
})"
echo "Initialising canister"
dfx canister call super_stats_v3_gldt --network $NETWORK init_target_ledger "$INIT_ARGUMENTS"
echo "Start processing timer"
dfx canister call super_stats_v3_gldt --network $NETWORK start_processing_timer '(60: nat64)'
echo "Adding authorized principals"
dfx canister call super_stats_v3_gldt --network $NETWORK add_authorised "2vxsx-fae"
