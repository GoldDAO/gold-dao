#!/usr/bin/env bash

show_help() {
  cat << EOF
management canister deployment script.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.

The canister will always be reinstalled locally, and only upgraded in staging and production (ic).

Usage:
  scripts/deploy-icp-neuron.sh [options] <NETWORK>

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

if [[ $1 =~ ^(local|staging|preprod)$ ]]; then
  TESTMODE="true"
  AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
else
  TESTMODE="false"
  AUTHORIZED_PRINCIPAL=2we4k-xim55-asne3-m7o22-fliz6-lmu6q-5pwc5-evfit-4scxr-itg7g-xae
fi

ARGS='(record {
  test_mode = '"$TESTMODE"';
  authorized_principals = vec { principal "'"$AUTHORIZED_PRINCIPAL"'"; };
})'

echo "Deployment arguments: \n" $ARGS

if [[ $1 == "local" ]]; then
  dfx deploy management --network $1 ${REINSTALL} --argument "$ARGS" -y
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $1 == "ic" && $CI_COMMIT_TAG =~ ^management-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then

  # This is for direct deployment via CICD identity
  dfx deploy management --network $1 ${REINSTALL} --argument "$ARGS" -y

fi
return
