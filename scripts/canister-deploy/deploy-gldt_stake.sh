#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="direct"

. ./scripts/extract_commit_tag_data_and_commit_sha.sh gldt_stake $NETWORK

if [[ $REINSTALL == "reinstall" ]]; then

  if [[ $NETWORK =~ ^(local|staging)$ ]]; then
    TESTMODE=true
    AUTHORIZED_PRINCIPALS="vec { principal \"465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae\"; principal \"$(dfx canister id --network $NETWORK sns_governance)\" }"
    ICP_LEDGER_CANISTER_ID=ete3q-rqaaa-aaaal-qdlva-cai
    OGY_LEDGER_CANISTER_ID=j5naj-nqaaa-aaaal-ajc7q-cai
    GOLDAO_LEDGER_CANISTER_ID=irhm6-5yaaa-aaaap-ab24q-cai
    GOLDAO_SNS_GOVERNANCE_CANISTER_ID=j3ioe-7iaaa-aaaap-ab23q-cai
    GOLDAO_SNS_REWARDS_CANISTER_ID=rbv23-fqaaa-aaaam-qbfma-cai
    GLDT_LEDGER_ID=6uad6-fqaaa-aaaam-abovq-cai
    REWARD_TYPES="vec {
        record {
            \"GOLDAO\";
            1 = record {
                0 = principal \"irhm6-5yaaa-aaaap-ab24q-cai\";
                1 = 100000:nat;
            };
        };
        record {
            \"ICP\";
            1 = record {
                0 = principal \"ete3q-rqaaa-aaaal-qdlva-cai\";
                1 = 10000:nat;
            };
        };
        record {
            \"OGY\";
            1 = record {
                0 = principal \"jwcfb-hyaaa-aaaaj-aac4q-cai\";
                1 = 200000:nat;
            };
        };
    }"
  elif [[ $NETWORK =~ ^(ic)$ ]]; then
    TESTMODE=false
    AUTHORIZED_PRINCIPALS="vec { principal \"tr3th-kiaaa-aaaaq-aab6q-cai\" }"
    ICP_LEDGER_CANISTER_ID=ryjl3-tyaaa-aaaaa-aaaba-cai
    OGY_LEDGER_CANISTER_ID=lkwrt-vyaaa-aaaaq-aadhq-cai
    GOLDAO_LEDGER_CANISTER_ID=tyyy3-4aaaa-aaaaq-aab7a-cai
    GOLDAO_SNS_GOVERNANCE_CANISTER_ID=tr3th-kiaaa-aaaaq-aab6q-cai
    GOLDAO_SNS_REWARDS_CANISTER_ID=iyehc-lqaaa-aaaap-ab25a-cai
    GLDT_LEDGER_ID=6c7su-kiaaa-aaaar-qaira-cai
    REWARD_TYPES="vec {
        record {
            \"GOLDAO\";
            1 = record {
                0 = principal \"tyyy3-4aaaa-aaaaq-aab7a-cai\";
                1 = 100000:nat;
            };
        };
        record {
            \"ICP\";
            1 = record {
                0 = principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\";
                1 = 10000:nat;
            };
        };
        record {
            \"OGY\";
            1 = record {
                0 = principal \"lkwrt-vyaaa-aaaaq-aadhq-cai\";
                1 = 200000:nat;
            };
        };
    }"
  else
    echo "Error: unknown network for deployment. Found $NETWORK."
    exit 2
  fi

  ARGUMENTS="(variant { Init = record {
    test_mode = $TESTMODE;
    commit_hash = \"$COMMIT_SHA\";
    version = $BUILD_VERSION;
    authorized_principals = $AUTHORIZED_PRINCIPALS;
    gldt_ledger_id = principal \"$GLDT_LEDGER_ID\";
    goldao_ledger_id = principal \"$GOLDAO_LEDGER_CANISTER_ID\";
    gld_sns_rewards_canister_id = principal \"$GOLDAO_SNS_REWARDS_CANISTER_ID\";
    gld_sns_governance_canister_id = principal \"$GOLDAO_SNS_GOVERNANCE_CANISTER_ID\";
    reward_types = $REWARD_TYPES;
  }})"

else
  ARGUMENTS="(variant { Upgrade = record {
    version = $BUILD_VERSION;
    commit_hash = \"$COMMIT_SHA\";
  }})"
fi

. ./scripts/deploy_backend_canister.sh gldt_stake $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA $VERSION $REINSTALL
