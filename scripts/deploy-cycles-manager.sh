#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

show_help() {
  cat << EOF
cycles_manager canister deployment script.
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

# TODO: add a --identity option ?? (See dfx deploy --identity)
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

ARGS=$(cat <<EOF
'(
  record {
    sns_root_canister = null;
    min_cycles_balance = 200000000000000 : nat64;
    authorized_principals = vec {
      principal "pl7dv-exerb-4tciu-c7hf3-qjtxg-ba6gh-7w45s-3lgzu-5ww5j-yupk6-uae";
    };
    canisters = vec {};
    max_top_up_amount = 500000000000000 : nat64;
    min_interval = 60 : nat64;
  },
)'
EOF
)

echo "Deployment arguments: \n" $ARGS

if [[ $1 == "local" ]]; then
  dfx deploy sns_rewards --network $1 ${REINSTALL} --argument "$ARGS" -y
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $1 == "ic" && $CI_COMMIT_TAG =~ ^sns_rewards-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then

  # This is for direct deployment via CICD identity
  dfx deploy sns_rewards --network $1 ${REINSTALL} --argument "$ARGS" -y


fi