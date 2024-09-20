#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="proposal"

. ./scripts/extract_commit_tag_data_and_commit_sha.sh sns_rewards $NETWORK

if [[ $REINSTALL == "reinstall" ]]; then

  if [[ $NETWORK =~ ^(local|staging)$ ]]; then
    TESTMODE=true
    ICP_LEDGER_CANISTER_ID=ete3q-rqaaa-aaaal-qdlva-cai
    SNS_LEDGER_CANISTER_ID=irhm6-5yaaa-aaaap-ab24q-cai
    OGY_LEDGER_CANISTER_ID=j5naj-nqaaa-aaaal-ajc7q-cai
    SNS_GOVERNANCE_CANISTER_ID=j3ioe-7iaaa-aaaap-ab23q-cai
  elif [[ $NETWORK =~ ^(ic)$ ]]; then
    TESTMODE=false
    ICP_LEDGER_CANISTER_ID=ryjl3-tyaaa-aaaaa-aaaba-cai
    SNS_LEDGER_CANISTER_ID=tyyy3-4aaaa-aaaaq-aab7a-cai
    OGY_LEDGER_CANISTER_ID=lkwrt-vyaaa-aaaaq-aadhq-cai
    SNS_GOVERNANCE_CANISTER_ID=tr3th-kiaaa-aaaaq-aab6q-cai
  else
    echo "Error: unknown network for deployment. Found $NETWORK."
    exit 2
  fi

  ARGUMENTS="(variant { Init = record {
    test_mode = $TESTMODE;
    commit_hash = \"$COMMIT_SHA\";
    icp_ledger_canister_id = principal \"$ICP_LEDGER_CANISTER_ID\";
    sns_ledger_canister_id = principal \"$SNS_LEDGER_CANISTER_ID\";
    ogy_ledger_canister_id = principal \"$OGY_LEDGER_CANISTER_ID\";
    sns_gov_canister_id = principal \"$SNS_GOVERNANCE_CANISTER_ID\"
  }})"

else
  ARGUMENTS="(variant { Upgrade = record {
    wasm_version = $WASM_VERSION;
    commit_hash = \"$COMMIT_SHA\";
  }})"
fi

. ./scripts/deploy_backend_canister.sh sns_rewards $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA $VERSION $REINSTALL
