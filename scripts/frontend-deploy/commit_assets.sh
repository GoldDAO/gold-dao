#!/bin/bash

# local testing
PEM_FILE="tmp.pem"
dfx identity export gitlab_ci_gldt_staging > $PEM_FILE
SNS_PROPOSER_NEURON_ID_STAGING="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"


CONFIG_FRONTEND="scripts/frontend-deploy/frontend_config.json"
CANISTER_IDS="sns_canister_ids.json"

FRONTEND=$1
NETWORK=$2

BATCH_ID=$3
EVIDENCE_RAW=$4

VERSION=$5
COMMIT_SHA=$6


FID=$(echo "$CONFIG_FRONTEND" | jq '.gld_dashboard.sns_function_id')
URL=$(echo "$CONFIG_FRONTEND" | jq '.gld_dashboard.url')


EVIDENCE_BLOB=$(echo "$EVIDENCE_RAW" | sed 's/../\\&/g')

export BLOB="$(didc encode --format blob "(record {
    batch_id = $BATCH_ID : nat;
    evidence = blob \"$EVIDENCE_BLOB\"
})")"

if [[ $NETWORK == "ic" ]]; then
    PROPOSER=$SNS_PROPOSER_NEURON_ID_PRODUCTION
    UPGRADEVERSION="${CI_COMMIT_TAG#*-v}"
else
    PROPOSER=$SNS_PROPOSER_NEURON_ID_STAGING
    UPGRADEVERSION=$CI_COMMIT_SHORT_SHA
fi

./scripts/parse_proposal_details.sh $FRONTEND frontend $BATCH_ID $EVIDENCE
./scripts/prepare_sns_canister_ids.sh $NETWORK

[ -e message.json ] && rm message.json

quill sns \
    --canister-ids-file $CANISTER_IDS \
    --pem-file $PEM_FILE \
    make-proposal \
    $PROPOSER \
    --proposal "(
    record {
        title=\"Upgrade $FRONTEND to version $VERSION.\";
        url=\"$URL\";
        summary=\"$(echo proposal.md)\";
        action= opt variant {
            ExecuteGenericNervousSystemFunction = record {
                function_id= ${FID}:nat64;
                payload = ${BLOB}
            }
        }
    }
)" > message.json

quill send message.json --dry-run

rm message.json && rm $CANISTER_IDS
