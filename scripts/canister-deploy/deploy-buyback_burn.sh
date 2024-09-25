#!/usr/bin/env bash

NETWORK=$1
MODE=$2
DEPLOYMENT_VIA="direct"

if [[ $NETWORK =~ ^(local|staging)$ ]]; then
  TESTMODE="true"
  GLDGOV_LEDGER_CANISTER_ID=tyyy3aaa-aaaaq-aab7a-cai
  AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
  BUYBACK_BURN_INTERVAL_IN_SECS=21_600
elif [[ $NETWORK =~ ^(ic)$ ]]; then
  TESTMODE="false"
  GLDGOV_LEDGER_CANISTER_ID=tyyy3-4aaaa-aaaaq-aab7a-cai
  AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
  BUYBACK_BURN_INTERVAL_IN_SECS=21_600
else
  echo "Error: unknown network for deployment. Found $NETWORK."
  exit 2
fi

BURN_RATE=33
MIN_BURN_AMOUNT=30_000_000_000
ICP_SWAP_CANISTER_ID=7eikv-2iaaa-aaaag-qdgwa-cai
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

# Validate mode and parse related arguments
if [[ $MODE == "init" ]]; then
  if [[ $# -ne 4 ]]; then
    echo "Error: init mode requires <VERSION> and <COMMIT_HASH>"
    exit 1
  fi
  VERSION=$3
  COMMIT_HASH=$4

  # Extracting major, minor, and patch from the version
  IFS='.' read -r MAJOR MINOR PATCH <<< "$VERSION"

  if [[ -z $MAJOR || -z $MINOR || -z $PATCH ]]; then
    echo "Error: version format should be x.y.z"
    exit 1
  fi
  # Arguments for init mode
  ARGUMENTS='(
    variant {
      Init = record {
        test_mode = '"$TESTMODE"';
        version = record {
          major = '"$MAJOR"' : nat32;
          minor = '"$MINOR"' : nat32;
          patch = '"$PATCH"' : nat32;
        };
        commit_hash = "'"$COMMIT_HASH"'";
        authorized_principals = vec {
          principal "'"$AUTHORIZED_PRINCIPAL"'";
        };
        icp_swap_canister_id = principal "'"$ICP_SWAP_CANISTER_ID"'";
        gldgov_token_info = '"$GLDGOV_TOKEN_INFO"';
        tokens = vec {'"$GLDGOV_ICP_POOL"'};
        burn_rate = '"$BURN_RATE"' : nat8;
        min_burn_amount = record { e8s = '"$MIN_BURN_AMOUNT"' : nat64 };
        buyback_burn_interval_in_secs = '"$BUYBACK_BURN_INTERVAL_IN_SECS"' : nat64;
      }
    }
  )'

elif [[ $MODE == "upgrade" ]]; then
  if [[ $# -ne 4 ]]; then
    echo "Error: upgrade mode requires <VERSION> and <COMMIT_HASH>"
    exit 1
  fi
  VERSION=$3
  COMMIT_HASH=$4
    # Extracting major, minor, and patch from the version
  IFS='.' read -r MAJOR MINOR PATCH <<< "$VERSION"

  if [[ -z $MAJOR || -z $MINOR || -z $PATCH ]]; then
    echo "Error: version format should be x.y.z"
    exit 1
  fi
  # Arguments for upgrade mode
  ARGUMENTS='(
    variant {
      Upgrade = record {
        version = record {
          major = '"$MAJOR"' : nat32;
          minor = '"$MINOR"' : nat32;
          patch = '"$PATCH"' : nat32;
        };
        commit_hash = "'"$COMMIT_HASH"'";
      }
    }
  )'
else
  echo "Error: mode must be either 'init' or 'upgrade'"
  exit 1
fi

. ./scripts/deploy_backend_canister.sh buyback_burn $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA
