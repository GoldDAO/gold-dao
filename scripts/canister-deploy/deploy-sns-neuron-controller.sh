#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

show_help() {
  cat << EOF
sns_neuron_controller canister deployment script.
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
SNS_ROOT_CANISTER_ID=i7fbw-giaaa-aaaap-ab25q-cai
AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
else
TESTMODE="false"
SNS_ROOT_CANISTER_ID=tw2vt-hqaaa-aaaaq-aab6a-cai
AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
fi

export MIN_CYCLES_BALANCE=20_000_000_000_000
export MAX_TOP_UP_AMOUNT=10_000_000_000_000
export ICP_BURN_AMOUNT=500_000_000
export ICP_LEDGER_CANISTER_ID=ryjl3-tyaaa-aaaaa-aaaba-cai
export CYCLES_MINTING_CANISTER_ID=rkp4c-7iaaa-aaaaa-aaaca-cai

ARGS='(
  record {
    test_mode = '"$TESTMODE"';
    sns_root_canister = principal "'"$SNS_ROOT_CANISTER_ID"'";
    authorized_principals = vec {
      principal "'"$AUTHORIZED_PRINCIPAL"'";
    };
    canisters = vec {};
    min_cycles_balance = '"$MIN_CYCLES_BALANCE"' : nat64;
    max_top_up_amount = '"$MAX_TOP_UP_AMOUNT"' : nat64;
    icp_burn_amount = record { e8s = '"$ICP_BURN_AMOUNT"' : nat64 };
    icp_ledger_canister = principal "'"$ICP_LEDGER_CANISTER_ID"'";
    cycles_minting_canister = principal "'"$CYCLES_MINTING_CANISTER_ID"'";
  },
)'

echo "Deployment arguments: \n" $ARGS

if [[ $1 == "local" ]]; then
  dfx deploy cycles_manager --network $1 ${REINSTALL} --argument "$ARGS" -y
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $1 == "ic" && $CI_COMMIT_TAG =~ ^cycles_manager-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then

  # This is for direct deployment via CICD identity
  dfx deploy cycles_manager --network $1 ${REINSTALL} --argument "$ARGS" -y


fi