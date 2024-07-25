#!/usr/bin/env bash

# dfx deploy --network staging gld_dashboard --by-proposal
# dfx canister --network staging call gld_dashboard delete_batch '( record { batch_id = 2 } )'

# export OUTPUT=$(ENV=staging dfx deploy --network staging --by-proposal gld_dashboard 2>&1)
# export OUTPUT=$(ENV=staging dfx deploy --network staging --by-proposal gld_dashboard 2>&1 | tee output.log)

LOG_FILE=console.log

# ENV=staging dfx deploy --network staging --by-proposal gld_dashboard 2>&1 | tee $LOG_FILE

# LAST_LINE=$(tail -n 1 $LOG_FILE)
BATCH_ID=$(tail -n 1 $LOG_FILE | awk '{print $5}')
EVIDENCE=$(tail -n 1 $LOG_FILE | awk '{print $8}' | sed "s/\.//" )

# echo $BATCH_ID
# echo $EVIDENCE

# export LAST_LINE="$(echo $OUTPUT | tail -n 1)"
# export BATCH_NUMBER="$(echo $LAST_LINE | awk '{print $5}' )"
# export EVIDENCE="$(echo $LAST_LINE | awk '{print $8}' )"

# echo "Output: $OUTPUT"
# echo "Last line: $LAST_LINE"
echo "Batch number: $BATCH_ID, Evidence: $EVIDENCE"

if [[ $BATCH_ID =~ "^[0-9]+$" ]]; then
  echo "Valid batch id: $BATCH_ID"
else
  echo "Invalid batch id: $BATCH_ID"
fi


if [[ $EVIDENCE =~ "^[0-9a-f]{64}$" ]]; then
  echo "Valid evidence: $EVIDENCE"
else
  echo "Invalid evidence: $EVIDENCE"
fi


return
