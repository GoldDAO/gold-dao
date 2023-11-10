#!/usr/bin/env bash

## As argument, preferably pass $1 previously defined by calling the pre-deploy script with the dot notation.

show_help() {
  cat << EOF
gldt_core canister deployment script.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.
The NETWORK argument should preferably be passed from the env variable that was previously defined
by the pre-deploy script (using the dot notation, or inside a macro deploy script).

The canister will always be reinstalled locally, and only upgraded in staging and production (ic).

Usage:
  scripts/deploy-gldt-core.sh [options] <NETWORK>

Options:
  -h, --help        Show this message and exit
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

if [[ $1 == "local" ]]; then
# Temporarily use canister ids from staging for NFTs. Use the correct local ids when local NFT deployment will be working.
  dfx deploy gldt_core --network $1 --argument '(
  opt record {gldt_ledger_canister_id=principal "'"$(dfx canister id --network ${1} gldt_ledger)"'";
  gld_nft_canister_ids=vec{
    record { principal "'"$(dfx canister id --network staging gldnft_backend_1g)"'"; record { grams=1}};
    record { principal "'"$(dfx canister id --network staging gldnft_backend_10g)"'"; record { grams=10}}
    };
  gldt_fee_compensation_canister_id=principal "'"$(dfx canister id --network ${1} gldt_fee_compensation)"'"
    })' --mode reinstall -y
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $1 == "ic" && $CI_COMMIT_TAG =~ ^core-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then
  dfx deploy gldt_core --network $1 --argument '(
    opt record {gldt_ledger_canister_id=principal "'"$(dfx canister id --network ${1} gldt_ledger)"'";
    gld_nft_canister_ids=vec{
      record { principal "'"$(dfx canister id --network ${1} gldnft_backend_1g)"'"; record { grams=1}};
      record { principal "'"$(dfx canister id --network ${1} gldnft_backend_10g)"'"; record { grams=10}}
    };
    gldt_fee_compensation_canister_id=principal "'"$(dfx canister id --network ${1} gldt_fee_compensation)"'"
      })' --no-wallet ${REINSTALL} -y
fi
