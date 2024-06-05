#!/usr/bin/env bash

./scripts/build-canister.sh cycles_manager &&
./scripts/generate-did.sh cycles_manager &&
./scripts/build-canister.sh cycles_manager &&
# TODO: change parameters to the valid
dfx deploy --network staging cycles_manager --argument '(record {sns_root_canister = opt principal "i7fbw-giaaa-aaaap-ab25q-cai"; min_cycles_balance = 20_000_000_000_000 : nat64; authorized_principals = vec { principal "pl7dv-exerb-4tciu-c7hf3-qjtxg-ba6gh-7w45s-3lgzu-5ww5j-yupk6-uae";}; canisters = vec { principal "a4tbr-q4aaa-aaaaa-qaafq-cai" }; max_top_up_amount = 500_000_000_000_000 : nat64; },)' --mode reinstall
