#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

show_help() {
  cat << EOF
sns_rewards canister deployment script.
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
  ICP_LEDGER_CANISTER_ID=ete3q-rqaaa-aaaal-qdlva-cai
  SNS_LEDGER_CANISTER_ID=irhm6-5yaaa-aaaap-ab24q-cai
  OGY_LEDGER_CANISTER_ID=jwcfb-hyaaa-aaaaj-aac4q-cai # needs to be updated when staging is available
  SNS_GOVERNANCE_CANISTER_ID=j3ioe-7iaaa-aaaap-ab23q-cai
else
  TESTMODE="false"
  ICP_LEDGER_CANISTER_ID=ryjl3-tyaaa-aaaaa-aaaba-cai
  SNS_LEDGER_CANISTER_ID=tyyy3-4aaaa-aaaaq-aab7a-cai
  OGY_LEDGER_CANISTER_ID=jwcfb-hyaaa-aaaaj-aac4q-cai # needs to be updated after sns
  SNS_GOVERNANCE_CANISTER_ID=tr3th-kiaaa-aaaaq-aab6q-cai
fi

ARGS='(record {
  test_mode = '$TESTMODE';
  icp_ledger_canister_id = principal "'$ICP_LEDGER_CANISTER_ID'";
  sns_ledger_canister_id = principal "'$SNS_LEDGER_CANISTER_ID'";
  ogy_ledger_canister_id = principal "'$OGY_LEDGER_CANISTER_ID'";
  sns_gov_canister_id = principal "'$SNS_GOVERNANCE_CANISTER_ID'"
  })'

if [[ $1 == "local" ]]; then
  dfx deploy sns_rewards --network $1 ${REINSTALL} --argument $ARGS -y
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $1 == "ic" && $CI_COMMIT_TAG =~ ^sns_rewards-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then

  # This is for direct deployment via CICD identity
  dfx deploy sns_rewards --network $1 ${REINSTALL} --argument $ARGS -y

  # The following lines are for deployment via SNS. Only activate when handing over the canister
  # TODO - make sure to improve this procedure, created issue #156 to address this

  # if [[ $1 == "ic" ]]; then
  #   PROPOSER=$SNS_PROPOSER_NEURON_ID_PRODUCTION
  #   UPGRADEVERSION=$CI_COMMIT_TAG
  # else
  #   PROPOSER=$SNS_PROPOSER_NEURON_ID_STAGING
  #   UPGRADEVERSION=$CI_COMMIT_SHORT_SHA
  # fi
  # . scripts/prepare_sns_canister_ids.sh $1 && \
  # . scripts/parse_proposal_details.sh && \
  # quill sns --canister-ids-file sns_canister_ids.json make-upgrade-canister-proposal $PROPOSER \
  #   --pem-file $PEM_FILE \
  #   --canister-upgrade-arg $ARGS \
  #   --target-canister-id $(cat canister_ids.json | jq -r .sns_rewards.$1) \
  #   --wasm-path backend/canisters/sns_rewards/target/wasm32-unknown-unknown/release/sns_rewards_canister.wasm.gz \
  #   --title "Upgrade sns_rewards to ${UPGRADEVERSION}" \
  #   --url ${DETAILS_URL} --summary-path proposal.md | quill send --yes -
fi
return
