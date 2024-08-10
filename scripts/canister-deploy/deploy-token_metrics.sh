#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

show_help() {
  cat << EOF
token_metrics canister deployment script.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.
The NETWORK argument should preferably be passed from the env variable that was previously defined
by the pre-deploy script (using the dot notation, or inside a macro deploy script).

The canister will always be reinstalled locally, and only upgraded in staging and production (ic).

Usage:
  scripts/deploy-token_metrics.sh [options] <NETWORK>

Options:
  -h, --help        Show this message and exit
EOF
}



if [[ $# -gt 0 ]]; then
  while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do
    case $1 in
      -h | --help )
        show_help
        exit
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
else
  echo "Error: missing <NETWORK> argument"
  exit 1
fi

NETWORK=$1
MODE="reinstall"

if [[ ! $NETWORK =~ ^(local|staging|ic)$ ]]; then
  echo "Error: unknown network for deployment"
  exit 2
fi

if [[ $NETWORK =~ ^(local|staging)$ ]]; then
  TESTMODE="true"
  OGY_LEDGER=$(dfx canister id sns_ledger --network staging)
  SNS_GOVERNANCE=$(dfx canister id sns_governance --network staging)
  SUPER_STATS=$(dfx canister id super_stats_v3 --network staging)
  SNS_REWARDS=$(dfx canister id sns_rewards --network staging)
  GOLD_TREASURY_ACCOUNT="$SNS_GOVERNANCE.7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce8"
else
  TESTMODE="false"
  OGY_LEDGER=$(dfx canister id sns_ledger --network $NETWORK)
  SNS_GOVERNANCE=$(dfx canister id sns_governance --network $NETWORK)
  SUPER_STATS=$(dfx canister id super_stats_v3 --network $NETWORK)
  SNS_REWARDS=$(dfx canister id sns_rewards --network $NETWORK)
  GOLD_TREASURY_ACCOUNT="$SNS_GOVERNANCE.7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce8"
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
  } )"


dfx deploy token_metrics --network $NETWORK --argument "$ARGUMENTS" --mode reinstall -y
