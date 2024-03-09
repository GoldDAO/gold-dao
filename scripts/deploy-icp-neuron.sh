#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

show_help() {
  cat << EOF
icp_neuron canister deployment script.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.
The NETWORK argument should preferably be passed from the env variable that was previously defined
by the pre-deploy script (using the dot notation, or inside a macro deploy script).

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

if [[ $1 =~ ^(local|staging)$ ]]; then
  TESTMODE="true"
else
  TESTMODE="false"
fi

if [[ $1 == "local" ]]; then
  dfx deploy icp_neuron --network $1 ${REINSTALL} --argument '(opt record {test_mode = '$TESTMODE' })' -y
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $1 == "ic" && $CI_COMMIT_TAG =~ ^icp_neuron-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then
  if [[ $1 == "ic" ]]; then
    PROPOSER=$SNS_PROPOSER_NEURON_ID_PRODUCTION
  else
    PROPOSER=$SNS_PROPOSER_NEURON_ID_STAGING
  fi
  . scripts/prepare_sns_canister_ids.sh $1 && \
  . scripts/parse_proposal_details.sh && \
#  dfx deploy icp_neuron --network $1 ${REINSTALL} --argument '(opt record {test_mode = '$TESTMODE' })' --by-proposal -y && \
  quill sns --canister-ids-file sns_canister_ids.json make-upgrade-canister-proposal $PROPOSER \
    --pem-file $PEM_FILE \
    --canister-upgrade-arg '(opt record {test_mode = '$TESTMODE' })' \
    --target-canister-id $(cat canister_ids.json | jq -r .icp_neuron.$1) \
    --wasm-path /builds/gldt/gldt-swap/.dfx/local/canisters/icp_neuron/icp_neuron.wasm.gz \
    --title "Upgrade icp_neuron to ${CI_COMMIT_TAG}" \
    --url ${DETAILS_URL} --summary-path proposal.md | quill send --yes -
fi
return
