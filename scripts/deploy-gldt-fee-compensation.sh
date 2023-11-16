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
  scripts/deploy-gldt-fee-compensation.sh [options] <NETWORK>

Options:
  -h, --help        Show this message and exit
  -r, --reinstall   Completely reinstall the canister, instead of simply upgrade it
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

dfx deploy gldt_fee_compensation --network $1 ${REINSTALL} --argument '(opt record {
  fallback_timer_interval_secs=3600;
  execution_delay_secs=20;
  gldt_canister_id=principal "'"$(dfx canister id --network $1 gldt_core)"'";
  gld_nft_canister_conf=vec{
    record { gld_nft_canister_id = principal "'"$(dfx canister id --network $1 gldnft_backend_1g)"'";  weight=1;  last_query_index=1820};
    record { gld_nft_canister_id = principal "'"$(dfx canister id --network $1 gldnft_backend_10g)"'";  weight=10;  last_query_index=286};
    };
  enabled= true;
  gldt_ledger_canister_id=principal "'"$(dfx canister id --network $1 gldt_ledger)"'";
  compensation_factor=10
    })'

dfx canister call --network $1 gldt_fee_compensation set_gld_nft_conf '( vec{
    record { gld_nft_canister_id = principal "'"$(dfx canister id --network $1 gldnft_backend_1g)"'";  weight=1;  last_query_index=1820};
    record { gld_nft_canister_id = principal "'"$(dfx canister id --network $1 gldnft_backend_10g)"'";  weight=10;  last_query_index=286}
    })'

# TODO: add all NFTs types for production environment !
