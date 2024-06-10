#!/usr/bin/env bash

./scripts/build-canister.sh cycles_manager &&
./scripts/generate-did.sh cycles_manager &&
./scripts/build-canister.sh cycles_manager &&
# TODO: change parameters to the valid
dfx deploy --network staging cycles_manager --argument '(record {sns_root_canister = principal "i7fbw-giaaa-aaaap-ab25q-cai"; min_cycles_balance = 20_000_000_000_000 : nat64; authorized_principals = vec { principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae";}; canisters = vec { }; max_top_up_amount = 10_000_000_000_000 : nat64; },)' --mode reinstall
gg