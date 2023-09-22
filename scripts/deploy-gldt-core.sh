#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

if [[ ! $1 =~ ^(local|staging|ic)$ ]]; then
  echo "Error: unknown network for deployment"
  exit 1
fi

if [[ $1 == "local" ]]; then
  dfx deploy gldt_core --network $1 --argument '(
  opt record {gldt_ledger_canister_id=principal "'"$(dfx canister id --network ${1} gldt_ledger)"'";
  gld_nft_canister_ids=vec{
    record { principal "'"$(dfx canister id --network ${1} gldnft_backend_1g)"'"; record { grams=1}};
    record { principal "'"$(dfx canister id --network ${1} gldnft_backend_10g)"'"; record { grams=10}}
    }})' --mode reinstall -y
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $1 == "ic" && $CI_COMMIT_TAG =~ ^core-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then
  dfx deploy gldt_core --network $1 --argument '(
    opt record {gldt_ledger_canister_id=principal "'"$(dfx canister id --network ${1} gldt_ledger)"'";
    gld_nft_canister_ids=vec{
      record { principal "'"$(dfx canister id --network ${1} gldnft_backend_1g)"'"; record { grams=1}};
      record { principal "'"$(dfx canister id --network ${1} gldnft_backend_10g)"'"; record { grams=10}}
      }})' --compute-evidence -y
fi
