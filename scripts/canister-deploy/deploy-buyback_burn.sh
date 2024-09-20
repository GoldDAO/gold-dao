#!/usr/bin/env bash


. ./scripts/extract_commit_tag_data_and_commit_sha.sh buyback_burn $NETWORK

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
      gldgov_ledger_canister_id = principal "'"$GLDGOV_LEDGER_CANISTER_ID"'";
      tokens = vec {};
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
