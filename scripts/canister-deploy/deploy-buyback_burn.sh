#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="direct"

. ./scripts/extract_commit_tag_data_and_commit_sha.sh buyback_burn $NETWORK

if [[ $REINSTALL == "reinstall" ]]; then

  if [[ $NETWORK =~ ^(local|staging)$ ]]; then
    TESTMODE=true
    GLDGOV_LEDGER_CANISTER_ID=$(dfx canister id --network ic sns_ledger)
    AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
    # 24 hours
    BUYBACK_BURN_INTERVAL_IN_SECS=$((6 * 3600))

  elif [[ $NETWORK =~ ^(ic)$ ]]; then
    TESTMODE=false
    GLDGOV_LEDGER_CANISTER_ID=$(dfx canister id --network $NETWORK sns_ledger)
    AUTHORIZED_PRINCIPAL=$(dfx canister id --network $NETWORK sns_governance)
    # 24 hours
    BUYBACK_BURN_INTERVAL_IN_SECS=$((6 * 3600))

  else
    echo "Error: unknown network for deployment. Found $NETWORK."
    exit 2
  fi

  BURN_RATE=33
  MIN_BURN_AMOUNT=30_000_000_000
  ICP_SWAP_CANISTER_ID="7eikv-2iaaa-aaaag-qdgwa-cai"

  GLDGOV_ICP_POOL='record {
      token = record {
          fee = 10_000 : nat64;
          decimals = 8 : nat64;
          ledger_id = principal "ryjl3-tyaaa-aaaaa-aaaba-cai";
      };
      swap_pool_id = principal "k46ek-4qaaa-aaaag-qcyzq-cai";
  };'

  GLDGOV_TOKEN_INFO='record {
      fee = 100_000 : nat64;
      decimals = 8 : nat64;
      ledger_id = principal "tyyy3-4aaaa-aaaaq-aab7a-cai";
  }'

  ARGUMENTS="(variant { Init = record {
        test_mode = $TESTMODE;
        version = $BUILD_VERSION;
        commit_hash = \"$COMMIT_HASH\";
        authorized_principals = vec {
          principal \"$AUTHORIZED_PRINCIPAL\";
        };
        icp_swap_canister_id = principal \"$ICP_SWAP_CANISTER_ID\";
        gldgov_token_info = $GLDGOV_TOKEN_INFO;
        tokens = vec {$GLDGOV_ICP_POOL};
        burn_rate = $BURN_RATE : nat8;
        min_burn_amount = record { e8s = $MIN_BURN_AMOUNT : nat64 };
        buyback_burn_interval_in_secs = $BUYBACK_BURN_INTERVAL_IN_SECS : nat64;
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
