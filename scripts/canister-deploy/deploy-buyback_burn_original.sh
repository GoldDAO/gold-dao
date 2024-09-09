#!/usr/bin/env bash

## Deployment script for the buyback_burn canister.

show_help() {
  cat << EOF
byuback_burn canister deployment script.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.
The NETWORK argument should preferably be passed from the env variable that was previously defined
by the pre-deploy script (using the dot notation, or inside a macro deploy script).

Usage:
  ./deploy-buyback_burn.sh <NETWORK> <MODE> <COMMIT_HASH> 
  ./deploy-buyback_burn.sh <NETWORK> upgrade <VERSION> <COMMIT_HASH>

Options:
  -h, --help        Show this message and exit

Examples:
  ./deploy-buyback_burn.sh local init "commit_hash"
  ./deploy-buyback_burn.sh local upgrade "1.0.1" "commit_hash"
EOF
}

# Parse options
if [[ $# -lt 3 ]]; then
  show_help
  exit 1
fi

# Check if help is requested
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
  show_help
  exit 0
fi

# Assign positional arguments
NETWORK=$1
MODE=$2

# Validate network argument
if [[ ! $NETWORK =~ ^(local|staging|ic)$ ]]; then
  echo "Error: unknown network for deployment"
  exit 2
fi

# Set deployment parameters based on the network
if [[ $NETWORK =~ ^(local|staging)$ ]]; then
  TESTMODE="true"
  GLDGOV_LEDGER_CANISTER_ID=tyyy3-4aaaa-aaaaq-aab7a-cai
  AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
  BURN_INTERVAL_IN_SECS=1_000_000  # 11 days
  SWAP_INTERVAL_IN_SECS=21_600     # 6 hours
else
  TESTMODE="false"
  GLDGOV_LEDGER_CANISTER_ID=tyyy3-4aaaa-aaaaq-aab7a-cai
  AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
  BURN_INTERVAL_IN_SECS=21_600
  SWAP_INTERVAL_IN_SECS=21_600
fi

export BURN_RATE=33
export MIN_ICP_BURN_AMOUNT=30_000_000_000
export ICP_SWAP_CANISTER_ID=7eikv-2iaaa-aaaag-qdgwa-cai

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

echo "Deployment arguments: \n" $ARGS


# Validate mode and parse related arguments
if [[ $MODE == "init" ]]; then
  if [[ $# -ne 3 ]]; then
    echo "Error: init mode requires <COMMIT_HASH>"
    exit 1
  fi
  COMMIT_HASH=$3
  # Arguments for init mode
  ARGS='(
    variant {
      Init = record {
        test_mode = '"$TESTMODE"';
        commit_hash = "'"$COMMIT_HASH"'";
        authorized_principals = vec {
          principal "'"$AUTHORIZED_PRINCIPAL"'";
        };
        icp_swap_canister_id = principal "'"$ICP_SWAP_CANISTER_ID"'";
        gldgov_token_info = '"$GLDGOV_TOKEN_INFO"';
        tokens = vec {'"$GLDGOV_ICP_POOL"'};
        burn_rate = '"$BURN_RATE"' : nat8;
        min_icp_burn_amount = record { e8s = '"$MIN_ICP_BURN_AMOUNT"' : nat64 };
        burn_interval_in_secs = '"$BURN_INTERVAL_IN_SECS"' : nat64;
        swap_interval_in_secs = '"$SWAP_INTERVAL_IN_SECS"' : nat64;
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
  # Arguments for upgrade mode
  ARGS='(
    variant {
      Upgrade = record {
        wasm_version = record {
          major = '"$(echo $VERSION | cut -d. -f1)"' : nat32;
          minor = '"$(echo $VERSION | cut -d. -f2)"' : nat32;
          patch = '"$(echo $VERSION | cut -d. -f3)"' : nat32;
        };
        commit_hash = "'"$COMMIT_HASH"'";
      }
    }
  )'
else
  echo "Error: mode must be either 'init' or 'upgrade'"
  exit 1
fi

# Deployment Logic
if [[ $NETWORK == "local" ]]; then
  # Deploy to local environment
  dfx deploy buyback_burn --network $NETWORK --argument "$ARGS"
elif [[ $NETWORK == "staging" ]]; then
  # Deploy to staging environment - works both manually and from CI/CD
  if [[ -n "$CI_COMMIT_REF_NAME" ]]; then
    echo "Deploying to staging from CI/CD on branch: $CI_COMMIT_REF_NAME"
    dfx deploy buyback_burn --network $NETWORK --argument "$ARGS"
  else
    echo "Deploying to staging manually"
    dfx deploy buyback_burn --network $NETWORK --argument "$ARGS"
  fi
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $NETWORK == "ic" && $CI_COMMIT_TAG =~ ^buyback_burn-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then
  dfx deploy buyback_burn --network $NETWORK --argument "$ARGS"
else
  echo "Error: Invalid deployment context or missing environment variables."
  exit 4
fi
