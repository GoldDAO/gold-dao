#!/usr/bin/env bash

# . scripts/build-canister gldt_fee_compensation &&
# . scripts/generate-did gldt_fee_compensation &&
dfx deploy gldt_fee_compensation --network staging --argument '(opt record {
  fallback_timer_interval_secs=3600;
  execution_delay_secs=20;
  gldt_canister_id=principal "'"$(dfx canister id --network staging gldt_core)"'";
  gld_nft_canister_conf=vec{
    record { gld_nft_canister_id = principal "'"$(dfx canister id --network staging gldnft_backend_1g)"'";  weight=1;  last_query_index=1820};
    record { gld_nft_canister_id = principal "'"$(dfx canister id --network staging gldnft_backend_10g)"'";  weight=10;  last_query_index=286};
    };
  enabled= true;
  gldt_ledger_canister_id=principal "'"$(dfx canister id --network staging gldt_ledger)"'"
    })'


# dfx canister call --network staging gldt_fee_compensation set_compensation_enabled '(false)'

dfx canister call --network staging gldt_fee_compensation set_gld_nft_conf '( vec{
    record { gld_nft_canister_id = principal "'"$(dfx canister id --network staging gldnft_backend_1g)"'";  weight=1;  last_query_index=1820};
    record { gld_nft_canister_id = principal "'"$(dfx canister id --network staging gldnft_backend_10g)"'";  weight=10;  last_query_index=286}
    })'
