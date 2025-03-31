#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="proposal"

. ./scripts/extract_commit_tag_data_and_commit_sha.sh sns_neuron_controller $NETWORK

if [[ $REINSTALL == "reinstall" ]]; then

  if [[ $NETWORK =~ ^(local|staging)$ ]]; then

    TESTMODE=true
    AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae

    OGY_SNS_GOVERNANCE_CANISTER_ID=jtpnb-waaaa-aaaal-ajc6q-cai
    OGY_SNS_LEDGER_CANISTER_ID=j5naj-nqaaa-aaaal-ajc7q-cai
    OGY_SNS_REWARDS_CANISTER_ID=fpmqz-aaaaa-aaaag-qjvua-cai

    WTN_SNS_GOVERNANCE_CANISTER_ID=jfnic-kaaaa-aaaaq-aadla-cai
    WTN_SNS_LEDGER_CANISTER_ID=jcmow-hyaaa-aaaaq-aadlq-cai

    SNS_REWARDS_CANISTER_ID=rbv23-fqaaa-aaaam-qbfma-cai
    ICP_LEDGER_CANISTER_ID=ete3q-rqaaa-aaaal-qdlva-cai

  elif [[ $NETWORK =~ ^(ic)$ ]]; then

    TESTMODE=false
    AUTHORIZED_PRINCIPAL=tr3th-kiaaa-aaaaq-aab6q-cai

    OGY_SNS_GOVERNANCE_CANISTER_ID=lnxxh-yaaaa-aaaaq-aadha-cai
    OGY_SNS_LEDGER_CANISTER_ID=lkwrt-vyaaa-aaaaq-aadhq-cai
    OGY_SNS_REWARDS_CANISTER_ID=yuijc-oiaaa-aaaap-ahezq-cai

    WTN_SNS_GOVERNANCE_CANISTER_ID=jfnic-kaaaa-aaaaq-aadla-cai
    WTN_SNS_LEDGER_CANISTER_ID=jcmow-hyaaa-aaaaq-aadlq-cai

    SNS_REWARDS_CANISTER_ID=iyehc-lqaaa-aaaap-ab25a-cai
    ICP_LEDGER_CANISTER_ID=ryjl3-tyaaa-aaaaa-aaaba-cai

  else
    echo "Error: unknown network for deployment. Found $NETWORK."
    exit 2
  fi

  OGY_REWARDS_THRESHOLD=100000000000000  # 1_000_000 tokens (as it was previously)
  ICP_REWARDS_THRESHOLD=100000000000000  # Example value for ICP (are not used yet)
  WTN_REWARDS_THRESHOLD=100000000000000  # Example value for WTN (are not used yet)

  ARGUMENTS="(variant { Init = record {
    test_mode = $TESTMODE;
    commit_hash = \"$COMMIT_SHA\";
    version = $BUILD_VERSION;
    authorized_principals = vec {
      principal \"$AUTHORIZED_PRINCIPAL\";
    };
    sns_rewards_canister_id = principal \"$SNS_REWARDS_CANISTER_ID\";
    ogy_manager_config = record {
      ogy_sns_governance_canister_id = principal \"$OGY_SNS_GOVERNANCE_CANISTER_ID\";
      ogy_sns_ledger_canister_id = principal \"$OGY_SNS_LEDGER_CANISTER_ID\";
      ogy_sns_rewards_canister_id = principal \"$OGY_SNS_REWARDS_CANISTER_ID\";
      ogy_rewards_threshold = $OGY_REWARDS_THRESHOLD : nat;
    };
    wtn_manager_config = record {
      wtn_sns_governance_canister_id = principal \"$WTN_SNS_GOVERNANCE_CANISTER_ID\";
      wtn_sns_ledger_canister_id = principal \"$WTN_SNS_LEDGER_CANISTER_ID\";
      icp_ledger = principal \"$ICP_LEDGER_CANISTER_ID\";
      icp_rewards_threshold = $ICP_REWARDS_THRESHOLD : nat;
      wtn_rewards_threshold = $WTN_REWARDS_THRESHOLD : nat;
    };
  }})"
else
  ARGUMENTS="(variant { Upgrade = record {
    version = $BUILD_VERSION;
    commit_hash = \"$COMMIT_SHA\";
  }})"
fi

. ./scripts/deploy_backend_canister.sh sns_neuron_controller $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA $VERSION $REINSTALL
