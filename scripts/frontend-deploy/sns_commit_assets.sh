#!/usr/bin/env bash

# Submits the SNS proposal that commits a proposed batch of a frontend canister.
# The proposal executes the frontend's generic function, which calls
# commit_proposed_batch on the asset canister with the batch id and evidence.

CONFIG_FRONTEND="scripts/frontend-deploy/frontend_config.json"
CANISTER_IDS_FILE="sns_canister_ids.json"

CANISTER_NAME=$1
NETWORK=$2
BATCH_ID=$3
EVIDENCE=$4
VERSION=$5
COMMIT_SHA=$6

# prepare_proposal_summary.sh reads these from the environment
export BATCH_ID EVIDENCE COMMIT_SHA

echo "
******************************************************************************************

Creating proposal to deploy $CANISTER_NAME on $NETWORK and upgrading to version $VERSION from commit $COMMIT_SHA.

******************************************************************************************

Committing batch $BATCH_ID with evidence $EVIDENCE.

******************************************************************************************
"

FID=$(jq -r --arg fe $CANISTER_NAME '.[$fe].sns_function_id' $CONFIG_FRONTEND)
URL=$(jq -r --arg fe $CANISTER_NAME '.[$fe].url' $CONFIG_FRONTEND)

if [[ ! $FID =~ ^[0-9]+$ ]]; then
  echo "Error: no sns_function_id for $CANISTER_NAME in $CONFIG_FRONTEND"
  exit 1
fi

echo "Function ID: $FID"
echo "URL: $URL"

# Candid encoded argument of commit_proposed_batch, as a Candid blob literal
EVIDENCE_BLOB=$(echo $EVIDENCE | sed 's/../\\&/g')

PAYLOAD="$(didc encode --format blob "(record {
    batch_id = $BATCH_ID : nat;
    evidence = blob \"$EVIDENCE_BLOB\"
})")"

if [[ $PAYLOAD != blob* ]]; then
  echo "Error: could not encode the proposal payload with didc"
  exit 1
fi

echo "Payload: $PAYLOAD"

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

./scripts/prepare_proposal_summary.sh $CANISTER_NAME $VERSION frontend

if [ $? -ne 0 ]; then
  echo "Error in prepare_proposal_summary.sh"
  exit 1
fi

./scripts/prepare_sns_canister_ids.sh $NETWORK

if [ $? -ne 0 ]; then
  echo "Error in prepare_sns_canister_ids.sh"
  exit 1
fi

# The summary goes into a Candid text literal: escape backslashes, double
# quotes and line breaks. A double quote in the changelog entry ended the
# literal early and the proposal could not be parsed.
PROPOSAL_SUMMARY=$(sed -e 's/\\/\\\\/g' -e 's/"/\\"/g' proposal.md | awk 'BEGIN { ORS = "\\n" } { print }')

PROPOSAL="(
    record {
        title = \"Upgrade $CANISTER_NAME to version $VERSION.\";
        url = \"$URL\";
        summary = \"$PROPOSAL_SUMMARY\";
        action = opt variant {
            ExecuteGenericNervousSystemFunction = record {
                function_id = $FID : nat64;
                payload = $PAYLOAD;
            }
        };
    }
)"

[ -e message.json ] && rm message.json

echo "
***********************************
Sending proposal
***********************************
"

quill sns \
    --canister-ids-file $CANISTER_IDS_FILE \
    --pem-file $PEM_FILE \
    make-proposal \
    $PROPOSER \
    --proposal "$PROPOSAL" > message.json

if [ $? -ne 0 ]; then
  echo "Error in signing the proposal. Batch $BATCH_ID is still proposed on the canister."
  exit 1
fi

RESPONSE=$(quill send message.json --yes 2>&1)
SEND_STATUS=$?

echo "$RESPONSE"

rm -f message.json $CANISTER_IDS_FILE

# quill exits with 0 even if governance answers with an error. It prints
# "Successfully created new proposal with ID <n>" when the proposal exists and
# "SNS governance error: ..." when governance refused it.
if [[ $SEND_STATUS -ne 0 ]] || ! echo "$RESPONSE" | grep -q "Successfully created new proposal"; then
  echo "Error: SNS governance did not accept the proposal. Batch $BATCH_ID is still proposed on the canister."
  exit 1
fi

PROPOSAL_ID=$(echo "$RESPONSE" | grep -oE 'Successfully created new proposal with ID [0-9]+' | grep -oE '[0-9]+$')

echo "
***********************************
Proposal $PROPOSAL_ID sent successfully. Batch $BATCH_ID is committed when the proposal is adopted.
***********************************
"
