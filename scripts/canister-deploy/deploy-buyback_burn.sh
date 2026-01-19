#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="proposal"


. ./scripts/extract_commit_tag_data_and_commit_sha.sh buyback_burn $NETWORK

if [[ $REINSTALL == "reinstall" ]]; then

  if [[ $NETWORK =~ ^(local|staging)$ ]]; then
    TESTMODE=true
    AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
    # 4 hours
    BUYBACK_INTERVAL_IN_SECS=$((4 * 3600))

  elif [[ $NETWORK =~ ^(ic)$ ]]; then
    TESTMODE=false
    AUTHORIZED_PRINCIPAL=$(dfx canister id --network $NETWORK sns_governance)
    # 4 hours
    BUYBACK_INTERVAL_IN_SECS=$((4 * 3600))

  else
    echo "Error: unknown network for deployment. Found $NETWORK."
    exit 2
  fi

  ICPSWAP_POOL_ID_GOLDAO="k46ek-4qaaa-aaaag-qcyzq-cai"
  ICPSWAP_POOL_ID_GLDT="4omhz-yiaaa-aaaag-qnalq-cai"
  MIN_SWAP_AMOUNT=10_000_000                # 0.1 tokens
  # Could be set if needed
  #MAX_SWAP_AMOUNT=

  EXCHANGE_CONFIG_GOLDAO="variant {
    ICPSwap = record {
      swap_canister_id = principal \"$ICPSWAP_POOL_ID_GOLDAO\";
      zero_for_one = true;
    }
  }"

  EXCHANGE_JOB_CONFIG_GOLDAO="record {
    token_to_sell = variant { ICP };
    token_to_buy = variant { GOLDAO };
    exchange = $EXCHANGE_CONFIG_GOLDAO;
    rate_per_interval = 793_650 : nat64;
    job_interval_ms = 14400 : nat64;
    source_subaccount = null;
    min_amount = record { e8s = $MIN_SWAP_AMOUNT : nat64 };
    max_amount = null;
    destination_account = null;
  }"

  EXCHANGE_CONFIG_GLDT="variant {
    ICPSwap = record {
      swap_canister_id = principal \"$ICPSWAP_POOL_ID_GLDT\";
      zero_for_one = false;
    }
  }"

  EXCHANGE_JOB_CONFIG_GLDT="record {
    token_to_sell = variant { ICP };
    token_to_buy = variant { GLDT };
    exchange = $EXCHANGE_CONFIG_GLDT;
    rate_per_interval = 3_571_428 : nat64;
    job_interval_ms = 21600 : nat64;
    source_subaccount = opt blob "'"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01"'";
    min_amount = record { e8s = $MIN_SWAP_AMOUNT : nat64 };
    max_amount = null;
    destination_account = opt "'"5aybl-v7aii-duvsu-ztemq-litdi-ly42r-iyf35-2k46p-ovynj-amtow-rae"'";
  }"

  # Combine all exchange configs into a vector
  EXCHANGE_CONFIGS="vec { $EXCHANGE_JOB_CONFIG_GOLDAO; $EXCHANGE_JOB_CONFIG_GLDT }"
  ICP_SWAP_CANISTER_ID="7eikv-2iaaa-aaaag-qdgwa-cai"


  ARGUMENTS="(variant { Init = record {
        test_mode = $TESTMODE;
        version = $BUILD_VERSION;
        commit_hash = \"$COMMIT_HASH\";
        authorized_principals = vec {
          principal \"$AUTHORIZED_PRINCIPAL\";
        };
        icp_swap_canister_id = principal \"$ICP_SWAP_CANISTER_ID\";
        exchange_configs = $EXCHANGE_CONFIGS;
        buyback_interval_in_secs = $BUYBACK_INTERVAL_IN_SECS : nat64;
      }
    }
  )"

else
  ARGUMENTS="(variant { Upgrade = record {
    version = $BUILD_VERSION;
    commit_hash = \"$COMMIT_SHA\";
  }})"
fi

. ./scripts/deploy_backend_canister.sh buyback_burn $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA $VERSION $REINSTALL

