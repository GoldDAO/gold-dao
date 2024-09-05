#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

show_help() {
  cat << EOF
byuback_burn canister deployment script.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.
The NETWORK argument should preferably be passed from the env variable that was previously defined
by the pre-deploy script (using the dot notation, or inside a macro deploy script).

The canister will always be reinstalled locally, and only upgraded in staging and production (ic).

Usage:
  scripts/deploy-cycles-manager.sh [options] <NETWORK>

Options:
  -h, --help        Show this message and exit
  -r, --reinstall   Completely reinstall the canister, instead of simply upgrade it
EOF
}

if [[ $# -gt 0 ]]; then
  while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do
    case $1 in
      -h | --help )
        show_help
        exit
        ;;
      -r | --reinstall )
        REINSTALL="--mode reinstall"
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
else
  echo "Error: missing <NETWORK> argument"
  exit 1
fi

if [[ ! $1 =~ ^(local|staging|ic)$ ]]; then
  echo "Error: unknown network for deployment"
  exit 2
fi


if [[ $1 =~ ^(local|staging)$ ]]; then
TESTMODE="true"
# GLDGOV_LEDGER_CANISTER_ID=irhm6-5yaaa-aaaap-ab24q-cai
GLDGOV_LEDGER_CANISTER_ID=tyyy3-4aaaa-aaaaq-aab7a-cai
# SNS_GOVERNANCE_CANISTER_ID=j3ioe-7iaaa-aaaap-ab23q-cai
AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
# 11 days
BURN_INTERVAL_IN_SECS=1_000_000
# 6 hours
SWAP_INTERVAL_IN_SECS=21_600
else
TESTMODE="false"
GLDGOV_LEDGER_CANISTER_ID=tyyy3-4aaaa-aaaaq-aab7a-cai
# SNS_GOVERNANCE_CANISTER_ID=tr3th-kiaaa-aaaaq-aab6q-cai
AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
# 6 hours
BURN_INTERVAL_IN_SECS=21_600
SWAP_INTERVAL_IN_SECS=21_600
fi

export BURN_RATE=33
export MIN_ICP_BURN_AMOUNT=30_000_000_000
export ICP_SWAP_CANISTER_ID=7eikv-2iaaa-aaaag-qdgwa-cai

# FIXME 
export COMMIT_HASH=hello

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

# Init args
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

# ARGS='(
#   variant {
#     Upgrade = record {
#       wasm_version = record {
#         major = 0 : nat32;
#         minor = 0 : nat32;
#         patch = 1 : nat32;
#       };
#       commit_hash = "'"$COMMIT_HASH"'";
#     }
# })'

echo "Deployment arguments: \n" $ARGS

# Deployment Logic
if [[ $1 == "local" ]]; then
  # Deploy to local environment
  dfx deploy buyback_burn --network $1 ${REINSTALL} --argument "$ARGS" -y
elif [[ $1 == "staging" ]]; then
  # Deploy to staging environment - works both manually and from CI/CD
  if [[ -n "$CI_COMMIT_REF_NAME" ]]; then
    # This block runs in CI/CD
    echo "Deploying to staging from CI/CD on branch: $CI_COMMIT_REF_NAME"
    dfx deploy buyback_burn --network $1 ${REINSTALL} --argument "$ARGS" -y
  else
    # This block runs when deploying manually
    echo "Deploying to staging manually"
    dfx deploy buyback_burn --network $1 ${REINSTALL} --argument "$ARGS" -y
  fi
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $1 == "ic" && $CI_COMMIT_TAG =~ ^buyback_burn-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then
  # Deploy to production (ic) environment if tag matches the pattern
  dfx deploy buyback_burn --network $1 ${REINSTALL} --argument "$ARGS" -y
else
  echo "Error: Invalid deployment context or missing environment variables."
  exit 4
fi