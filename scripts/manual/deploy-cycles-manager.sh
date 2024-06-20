#!/usr/bin/env bash
нуы
./scripts/build-canister.sh cycles_manager &&
./scripts/generate-did.sh cycles_manager &&
./scripts/build-canister.sh cycles_manager &&

# TODO: ensure that parameters are valid: check cycles minting canister, check ledger canister
dfx deploy --network local cycles_manager --argument '(record {test_mode = 'true'; sns_root_canister = principal "i7fbw-giaaa-aaaap-ab25q-cai"; authorized_principals = vec { principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; principal "pl7dv-exerb-4tciu-c7hf3-qjtxg-ba6gh-7w45s-3lgzu-5ww5j-yupk6-uae";}; canisters = vec { }; min_cycles_balance = 120_000_000_000 : nat64; max_top_up_amount = 1_000_000_000 : nat64; icp_burn_amount = record { e8s = 100_000_000_000 : nat64 }; ledger_canister = principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; cycles_minting_canister = principal "rkp4c-7iaaa-aaaaa-aaaca-cai";},)' --mode reinstall

