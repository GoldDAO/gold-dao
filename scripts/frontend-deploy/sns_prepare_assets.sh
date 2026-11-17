#!/usr/bin/env bash

# Uploads the built frontend to its asset canister as a proposed batch and
# exports BATCH_ID and EVIDENCE for the proposal. Nothing goes live here: the
# canister keeps serving the current version until SNS governance commits the
# batch.
#
# The identity needs the Prepare permission on the asset canister. Only one
# batch can be proposed at a time. A batch that an earlier run left behind is
# deleted when the job runs with the variable DELETE_PROPOSED_BATCH=true, see
# below.

CANISTER_NAME=$1
NETWORK=$2

LOG_FILE=console.log

echo "
**********************************************************************

Preparing assets for $CANISTER_NAME on $NETWORK

**********************************************************************
"

run_prepare() {
  if [[ "$(uname -s)" == "Darwin" ]]; then
    echo "Deploying from local system."
    ENV=$NETWORK VERSION="local-deploy" dfx deploy --network $NETWORK --by-proposal $CANISTER_NAME 2>&1 | tee $LOG_FILE
  else
    dfx deploy --network $NETWORK --by-proposal $CANISTER_NAME 2>&1 | tee $LOG_FILE
  fi
}

# The CI job shell runs with `set -eo pipefail`, so a failing dfx run would end the job before the
# stale-batch check below gets to read its output.
if ! run_prepare; then
  echo "dfx deploy --by-proposal failed, checking the output for a proposed batch left by an earlier run."
fi

# A batch left in "proposed" state by an earlier run (for example a deploy job that failed after the
# upload) blocks every new batch: "Batch N is already proposed. Delete or execute it to propose another."
# Delete it and retry only when the operator asks for it with the job variable DELETE_PROPOSED_BATCH=true,
# because a proposed batch may belong to a proposal that is still being voted on.
# `|| true`: under pipefail a grep with no match would otherwise end the job here.
STALE_BATCH=$(grep -o "Batch [0-9]* is already proposed" $LOG_FILE | awk '{print $2}' | head -n 1 || true)

if [[ -n $STALE_BATCH ]]; then
  if [[ $DELETE_PROPOSED_BATCH == "true" ]]; then
    echo "Deleting proposed batch $STALE_BATCH left by an earlier run (DELETE_PROPOSED_BATCH=true)."
    dfx canister call --network $NETWORK $CANISTER_NAME delete_batch "(record { batch_id = $STALE_BATCH : nat })"
    run_prepare
  else
    echo "Error: batch $STALE_BATCH on $CANISTER_NAME ($NETWORK) is already proposed. If no proposal is pending for it, run this job again with the variable DELETE_PROPOSED_BATCH=true to delete it first."
    exit 1
  fi
fi

# dfx reports "Proposed commit of batch <id> with evidence <hash>. Either commit it by proposal, or delete it."
# The line is searched in the whole output, so the job does not depend on it
# being the last line dfx prints.
PROPOSED=$(grep -oE 'Proposed commit of batch [0-9]+ with evidence [0-9a-f]+' $LOG_FILE | tail -n 1) || true

echo "Proposed batch line: $PROPOSED"

export BATCH_ID=$(echo "$PROPOSED" | awk '{print $5}')
export EVIDENCE=$(echo "$PROPOSED" | awk '{print $8}')

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
