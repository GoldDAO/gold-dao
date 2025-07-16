#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="direct"

. ./scripts/extract_commit_tag_data_and_commit_sha.sh gldt_stake $NETWORK

if [[ $REINSTALL == "reinstall" ]]; then
  if [[ $NETWORK =~ ^(local|staging)$ ]]; then
    TESTMODE=true
    AUTHORIZED_PRINCIPALS="vec {
      principal \"465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae\";
            principal \"$(dfx canister id --network $NETWORK sns_governance)\"
    }"
    WHITELIST="vec {
      principal \"465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae\";
            principal \"$(dfx canister id --network $NETWORK sns_governance)\"
    }"
    ICP_LEDGER_CANISTER_ID=ete3q-rqaaa-aaaal-qdlva-cai
    OGY_LEDGER_CANISTER_ID=j5naj-nqaaa-aaaal-ajc7q-cai
    GOLDAO_LEDGER_CANISTER_ID=irhm6-5yaaa-aaaap-ab24q-cai
    GOLDAO_SNS_GOVERNANCE_CANISTER_ID=j3ioe-7iaaa-aaaap-ab23q-cai
    GOLDAO_SNS_REWARDS_CANISTER_ID=rbv23-fqaaa-aaaam-qbfma-cai
    GLDT_LEDGER_ID=6uad6-fqaaa-aaaam-abovq-cai
    ALLOWED_REWARD_TOKENS="vec { variant { GOLDAO }; variant { ICP }; variant { OGY } }"
    ICRC3_CONFIG="record {
        supported_blocks = vec {
            record {
                block_type = \"transfer\";
                url = \"https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-3/transfer.md\";
            };
            record {
                block_type = \"mint\";
                url = \"https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-3/mint.md\";
            };
        };
        constants = record {
            tx_window = record { secs = 86400 : nat64; nanos = 0 : nat32; };
            max_transactions_in_window = 1000 : nat;
            max_memory_size_bytes = 1073741824 : nat;
            max_blocks_per_response = 100 : nat;
            initial_cycles = 100_000_000_000_000 : nat;
            reserved_cycles = 10_000_000_000_000 : nat;
            max_transactions_to_purge = 500 : nat;
            ttl_for_non_archived_transactions = record { secs = 120 : nat64; nanos = 0 : nat32; };
            max_unarchived_transactions = 1000 : nat;
        };
    }"
  elif [[ $NETWORK =~ ^(ic)$ ]]; then
    TESTMODE=false
    AUTHORIZED_PRINCIPALS="vec { principal \"tr3th-kiaaa-aaaaq-aab6q-cai\" }"
    WHITELIST="vec { principal \"tr3th-kiaaa-aaaaq-aab6q-cai\" }"
    ICP_LEDGER_CANISTER_ID=ryjl3-tyaaa-aaaaa-aaaba-cai
    OGY_LEDGER_CANISTER_ID=lkwrt-vyaaa-aaaaq-aadhq-cai
    GOLDAO_LEDGER_CANISTER_ID=tyyy3-4aaaa-aaaaq-aab7a-cai
    GOLDAO_SNS_GOVERNANCE_CANISTER_ID=tr3th-kiaaa-aaaaq-aab6q-cai
    GOLDAO_SNS_REWARDS_CANISTER_ID=iyehc-lqaaa-aaaap-ab25a-cai
    GLDT_LEDGER_ID=6c7su-kiaaa-aaaar-qaira-cai
    ALLOWED_REWARD_TOKENS="vec { variant { GOLDAO }; variant { ICP }; variant { OGY } }"
    ICRC3_CONFIG="record {
        supported_blocks = vec {
            record {
                block_type = \"transfer\";
                url = \"https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-3/transfer.md\";
            };
            record {
                block_type = \"mint\";
                url = \"https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-3/mint.md\";
            };
        };
        constants = record {
            tx_window = record { secs = 86400 : nat64; nanos = 0 : nat32; };
            max_transactions_in_window = 1000 : nat;
            max_memory_size_bytes = 1073741824 : nat;
            max_blocks_per_response = 100 : nat;
            initial_cycles = 100_000_000_000_000 : nat;
            reserved_cycles = 10_000_000_000_000 : nat;
            max_transactions_to_purge = 500 : nat;
            ttl_for_non_archived_transactions = record { secs = 120 : nat64; nanos = 0 : nat32; };
            max_unarchived_transactions = 1000 : nat;
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
    whitelist = $WHITELIST;
    gldt_ledger_id = principal \"$GLDT_LEDGER_ID\";
    goldao_ledger_id = principal \"$GOLDAO_LEDGER_CANISTER_ID\";
    gld_sns_rewards_canister_id = principal \"$GOLDAO_SNS_REWARDS_CANISTER_ID\";
    gld_sns_governance_canister_id = principal \"$GOLDAO_SNS_GOVERNANCE_CANISTER_ID\";
    allowed_reward_tokens = $ALLOWED_REWARD_TOKENS;
    icrc3_config = $ICRC3_CONFIG;
    }})"
else
  ARGUMENTS="(variant { Upgrade = record {
    version = $BUILD_VERSION;
    commit_hash = \"$COMMIT_SHA\";
  }})"
fi

. ./scripts/deploy_backend_canister.sh gldt_stake $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA $VERSION $REINSTALL

