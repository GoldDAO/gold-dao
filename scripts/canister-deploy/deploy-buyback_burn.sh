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
GLDGOV_LEDGER_CANISTER_ID=irhm6-5yaaa-aaaap-ab24q-cai
SNS_GOVERNANCE_CANISTER_ID=j3ioe-7iaaa-aaaap-ab23q-cai
AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
else
TESTMODE="false"
GLDGOV_LEDGER_CANISTER_ID=tyyy3-4aaaa-aaaaq-aab7a-cai
SNS_GOVERNANCE_CANISTER_ID=tr3th-kiaaa-aaaaq-aab6q-cai
AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
fi

export BURN_RATE=33
export MIN_ICP_BURN_AMOUNT=30_000_000_000
# 6 hours
export BURN_INTERVAL_IN_SECS=21_600 

ARGS='(
  record {
    test_mode = '"$TESTMODE"';
    authorized_principals = vec {
      principal "'"$AUTHORIZED_PRINCIPAL"'";
    };
    gldgov_ledger_canister_id = principal "'"$GLDGOV_LEDGER_CANISTER_ID"'";
    tokens = vec {};
    burn_rate = '"$BURN_RATE"' : nat64;
    min_icp_burn_amount = '"$MIN_ICP_BURN_AMOUNT"' : nat64;
    burn_interval_in_secs = record { e8s = '"$BURN_INTERVAL_IN_SECS"' : nat64 };
  },
)'


echo "Deployment arguments: \n" $ARGS

if [[ $1 == "local" ]]; then
  dfx deploy cycles_manager --network $1 ${REINSTALL} --argument "$ARGS" -y
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $1 == "ic" && $CI_COMMIT_TAG =~ ^cycles_manager-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then

  # This is for direct deployment via CICD identity
  dfx deploy cycles_manager --network $1 ${REINSTALL} --argument "$ARGS" -y
fi