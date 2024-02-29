#!/bin/bash

# Check if exactly one argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <staging|ic>"
    exit 1
fi

# Assign arguments to variables
CHOICE="$1"

INPUT_FILE="canister_ids.json"
OUTPUT_FILE="sns_canister_ids.json"

# Validate the choice
if [ "$CHOICE" != "staging" ] && [ "$CHOICE" != "ic" ]; then
    echo "Invalid choice: $CHOICE. Please choose 'staging' or 'ic'."
    exit 2
fi

jq --arg choice "$CHOICE" '{
  governance_canister_id: .sns_governance[$choice],
  index_canister_id: .sns_index[$choice],
  ledger_canister_id: .sns_ledger[$choice],
  root_canister_id: .sns_root[$choice],
  swap_canister_id: .sns_swap[$choice]
}' "$INPUT_FILE" > "$OUTPUT_FILE"

echo "Transformation complete based on $CHOICE. Output saved to $OUTPUT_FILE."
