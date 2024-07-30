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
  scripts/deploy-sns-neuron-controller.sh [options] <NETWORK>

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
SNS_REWARDS_CANISTER_ID=tw2vt-hqaaa-aaaaq-aab6a-cai
OGY_SNS_GOVERNANCE_CANISTER_ID=jtpnb-waaaa-aaaal-ajc6q-cai
OGY_SNS_LEDGER_CANISTER_ID=j5naj-nqaaa-aaaal-ajc7q-cai
OGY_SNS_REWARDS_CANISTER_ID=fpmqz-aaaaa-aaaag-qjvua-cai
AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
else
TESTMODE="false"
SNS_REWARDS_CANISTER_ID=tw2vt-hqaaa-aaaaq-aab6a-cai
OGY_SNS_GOVERNANCE_CANISTER_ID=lnxxh-yaaaa-aaaaq-aadha-cai
OGY_SNS_LEDGER_CANISTER_ID=lkwrt-vyaaa-aaaaq-aadhq-cai
OGY_SNS_REWARDS_CANISTER_ID=yuijc-oiaaa-aaaap-ahezq-cai
AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
fi

ARGS='(
  record {
    test_mode = '"$TESTMODE"';
        authorized_principals = vec {
      principal "'"$AUTHORIZED_PRINCIPAL"'";
    };
    sns_rewards_canister_id = principal "'"$SNS_REWARDS_CANISTER_ID"'";
    ogy_sns_governance_canister_id = principal "'"$OGY_SNS_GOVERNANCE_CANISTER_ID"'";
    ogy_sns_ledger_canister_id = principal "'"$OGY_SNS_LEDGER_CANISTER_ID"'";
    ogy_sns_rewards_canister_id = principal "'"$OGY_SNS_REWARDS_CANISTER_ID"'";
  },
)'

echo "Deployment arguments: \n" $ARGS

if [[ $1 == "local" ]]; then
  dfx deploy sns_neuron_controller --network $1 ${REINSTALL} --argument "$ARGS" -y
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $1 == "ic" && $CI_COMMIT_TAG =~ ^sns_neuron_controller-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then

  # This is for direct deployment via CICD identity
  dfx deploy sns_neuron_controller --network $1 ${REINSTALL} --argument "$ARGS" -y
fi