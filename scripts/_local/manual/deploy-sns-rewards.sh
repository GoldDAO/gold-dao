#!/usr/bin/env bash

./scripts/build_canister.sh sns_rewards &&
./scripts/generate_did.sh sns_rewards &&
./scripts/build_canister.sh sns_rewards &&
# dfx deploy --network staging sns_rewards --argument '(record {test_mode = true})' --mode reinstall
dfx deploy --network preprod sns_rewards --argument "(variant { Init = record {
  test_mode = true;
  version = record {
     major = 0:nat32;
     minor = 0:nat32;
     patch = 0:nat32;
  };
  commit_hash = \"manual\";
  icp_ledger_canister_id = principal \"ete3q-rqaaa-aaaal-qdlva-cai\";
  sns_ledger_canister_id = principal \"irhm6-5yaaa-aaaap-ab24q-cai\";
  ogy_ledger_canister_id = principal \"jwcfb-hyaaa-aaaaj-aac4q-cai\";
  sns_gov_canister_id = principal \"tr3th-kiaaa-aaaaq-aab6q-cai\"
  }})" --mode reinstall
