#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

show_help() {
  cat << EOF
super_stats_v3 canister deployment script.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.
The NETWORK argument should preferably be passed from the env variable that was previously defined
by the pre-deploy script (using the dot notation, or inside a macro deploy script).

The canister will always be reinstalled locally, and only upgraded in staging and production (ic).

Usage:
  scripts/deploy-super_stats_v3.sh [options] <NETWORK>

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
else
  TESTMODE="false"
fi
ADMIN=$(dfx identity get-principal)
ARGUMENTS="(record {
  test_mode = $TESTMODE;
  admin = \"$ADMIN\";
  } )"


dfx deploy super_stats_v3 --network $NETWORK --argument "$ARGUMENTS" --mode $MODE -y

if [ "$MODE" = "reinstall" ]; then
  TOKEN_METRICS_CANISTER_ID=$(dfx canister id sns_rewards --network $NETWORK)
  LEDGER_CANISTER_ID=$(dfx canister id sns_ledger --network $NETWORK)
  INIT_ARGUMENTS="'(record {
      target = record {
          target_ledger = "$LEDGER_CANISTER_ID";
          hourly_size = 24;
          daily_size = 30;
      };
      index_type = variant { "DfinityIcrc2" }
  })'"
  dfx canister call super_stats_v3 --network $NETWORK init_target_ledger $INIT_ARGUMENTS
  dfx canister call super_stats_v3 --network $NETWORK start_processing_timer '(60: nat64)'
  dfx canister call super_stats_v3 --network $NETWORK add_authorised "2vxsx-fae"
  dfx canister call super_stats_v3 --network $NETWORK add_authorised "$TOKEN_METRICS_CANISTER_ID"
fi
