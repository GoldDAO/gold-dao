#!/usr/bin/env bash


CONFIG_FRONTEND="scripts/frontend-deploy/frontend_config.json"
CANISTER_IDS_FILE="sns_canister_ids.json"

CANISTER_NAME=$1
NETWORK=$2
BATCH_ID=$3
EVIDENCE_RAW=$4
VERSION=$5
COMMIT_SHA=$6

echo "
******************************************************************************************

Creating proposal to deploy $CANISTER_NAME on $NETWORK and upgrading to version $VERSION from commit $COMMIT_SHA.

******************************************************************************************

Committing batch $BATCH_ID with evidence $EVIDENCE.

******************************************************************************************
"

FID=$(cat $CONFIG_FRONTEND | jq --arg fe $CANISTER_NAME '.[$fe].sns_function_id')
URL=$(cat $CONFIG_FRONTEND | jq --arg fe $CANISTER_NAME '.[$fe].url')

echo "Function ID: $FID"
echo "URL: $URL"

EVIDENCE_BLOB=$(echo $EVIDENCE_RAW | sed 's/../\\&/g')

echo "Evidence blob: $EVIDENCE_BLOB"

export BLOB="$(didc encode --format blob "(record {
    batch_id = $BATCH_ID : nat;
    evidence = blob \"$EVIDENCE_BLOB\"
})")"

if [[ $NETWORK == "ic" ]]; then
    PROPOSER=$SNS_PROPOSER_NEURON_ID_PRODUCTION
else
    PROPOSER=$SNS_PROPOSER_NEURON_ID_STAGING
fi

if [[ "$(uname -s)" == "Darwin" ]]; then
    echo "Running locally on Mac. Overwriting proposer id."
    PROPOSER="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
    PEM_FILE="tmp.pem"
fi

./scripts/prepare_proposal_summary.sh $CANISTER_NAME $VERSION frontend $BATCH_ID $EVIDENCE_RAW

if [ $? -ne 0 ]; then
  echo "Error in prepare_proposal_summary.sh"
  exit 1
fi

./scripts/prepare_sns_canister_ids.sh $NETWORK

if [ $? -ne 0 ]; then
  echo "Error in prepare_sns_canister_ids.sh"
  exit 1
fi

PROPOSAL_SUMMARY=$(cat proposal.md)

[ -e message.json ] && rm message.json

echo "
***********************************
Sending proposal
***********************************
"

echo "BLOB: $BLOB"

quill sns \
    --canister-ids-file $CANISTER_IDS_FILE \
    --pem-file $PEM_FILE \
    make-proposal \
    $PROPOSER \
    --proposal "(
    record {
        title=\"Upgrade $CANISTER_NAME to version $VERSION.\";
        url="$URL";
        summary=\"$PROPOSAL_SUMMARY\";
        action= opt variant {
            ExecuteGenericNervousSystemFunction = record {
                function_id= ${FID}:nat64;
                payload = ${BLOB}
            }
        }
    }
)" > message.json

quill send message.json -y

if [ $? -ne 0 ]; then
  echo "Error in sending proposal."
  exit 1
fi

rm message.json && rm $CANISTER_IDS_FILE

echo "
***********************************
Proposal sent successfully.
***********************************
"
