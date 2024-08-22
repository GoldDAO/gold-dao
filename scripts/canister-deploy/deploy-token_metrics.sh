#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="direct"

if [[ $NETWORK =~ ^(local|staging)$ ]]; then
  TESTMODE=true
  OGY_LEDGER=$(dfx canister id sns_ledger --network staging)
  SNS_GOVERNANCE=$(dfx canister id sns_governance --network staging)
  SUPER_STATS=$(dfx canister id super_stats_v3 --network staging)
  SNS_REWARDS=$(dfx canister id sns_rewards --network staging)
  GOLD_TREASURY_ACCOUNT="$SNS_GOVERNANCE.7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce8"
elif [[ $NETWORK =~ ^(ic)$ ]]; then
  TESTMODE=false
  OGY_LEDGER=$(dfx canister id sns_ledger --network $NETWORK)
  SNS_GOVERNANCE=$(dfx canister id sns_governance --network $NETWORK)
  SUPER_STATS=$(dfx canister id super_stats_v3 --network $NETWORK)
  SNS_REWARDS=$(dfx canister id sns_rewards --network $NETWORK)
  GOLD_TREASURY_ACCOUNT="$SNS_GOVERNANCE.7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce8"
else
  echo "Error: unknown network for deployment. Found $NETWORK."
  exit 2
fi

ARGUMENTS="(record {
  test_mode = $TESTMODE;
  ogy_new_ledger_canister_id = principal \"$OGY_LEDGER\";
  sns_governance_canister_id = principal \"$SNS_GOVERNANCE\";
  sns_rewards_canister_id = principal \"$SNS_REWARDS\";
  super_stats_canister_id = principal \"$SUPER_STATS\";
  treasury_account = \"$ORIGYN_TREASURY_ACCOUNT\";
  foundation_accounts = vec {
    \"$GOLD_TREASURY_ACCOUNT\"
  }
})"

. ./scripts/deploy_backend_canister.sh token_metrics $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA
