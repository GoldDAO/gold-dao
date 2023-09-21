#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

if [[ ! $1 =~ ^(local|staging|ic)$ ]]; then
  echo "Error: unknown network for deployment"
  exit 1
fi

# Change the variable to the account that can mint and burn tokens.
# export MINT_ACC=$(dfx ledger account-id --network ${1} --of-canister gldt_core)
export MINT_ACC=$(dfx canister id gldt_core --network $1)

# Change the variable to the principal that controls archive canisters.
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)

export TOKEN_NAME="Gold token"
export TOKEN_SYMBOL="GLDT"

if [[ $1 == "local" ]]; then
  dfx deploy --network $1 gldt_ledger --argument "(variant {Init = record {
    token_name = \"${TOKEN_NAME}\";
    token_symbol = \"${TOKEN_SYMBOL}\";
    minting_account = record {owner = principal \"${MINT_ACC}\"};
    metadata = vec {};
    transfer_fee = 10000;
    initial_balances = vec {};
    archive_options = record {
      trigger_threshold = 2000;
      num_blocks_to_archive = 1000;
      controller_id = principal \"${ARCHIVE_CONTROLLER}\";
      cycles_for_archive_creation = opt 10_000_000_000_000;
    }
  }})"
elif [[ $CI_COMMIT_REF_NAME == "develop" ]]; then
  echo "TODO: Continue with ledger deployment for staging"
  exit 1
elif [[ $CI_COMMIT_TAG =~ ^ledger-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ]]; then
  echo "TODO: Continue with ledger deployment for staging"
  exit 1
else
  echo "Error: no valid deployment conditions found."
  exit 1
fi
