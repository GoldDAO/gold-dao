#!/usr/bin/env bash

CANISTER_NAME=$1
NETWORK=$2

LOG_FILE=console.log

echo "
**********************************************************************

Preparing assets for $CANISTER_NAME on $NETWORK

**********************************************************************
"


# dfx canister call --network $NETWORK $CANISTER_NAME delete_batch '(record { batch_id = 5 : nat })'

if [[ "$(uname -s)" == "Darwin" ]]; then
  echo "Deploying from local systen."
  ENV=$NETWORK VERSION="local-deploy" dfx deploy --network $NETWORK --by-proposal $CANISTER_NAME 2>&1 | tee $LOG_FILE
else
  dfx deploy --network $NETWORK --by-proposal $CANISTER_NAME 2>&1 | tee $LOG_FILE
fi

echo "Last line: $(tail -n 1 $LOG_FILE)"

export BATCH_ID=$(tail -n 1 $LOG_FILE | awk '{print $5}')
export EVIDENCE=$(tail -n 1 $LOG_FILE | awk '{print $8}' | sed "s/\.//" )

echo "Batch number: $BATCH_ID, Evidence: $EVIDENCE"

if [[ $BATCH_ID =~ ^[0-9]+$ ]]; then
  echo "Valid batch id: $BATCH_ID"
else
  echo "Invalid batch id: $BATCH_ID"
  exit 1
fi

if [[ $EVIDENCE =~ ^[0-9a-f]{64}$ ]]; then
  echo "Valid evidence: $EVIDENCE"
else
  echo "Invalid evidence: $EVIDENCE"
  exit 1
fi

echo "

Finished preparing assets. Uploaded in batch $BATCH_ID with evidence $EVIDENCE

**********************************************************************
"
