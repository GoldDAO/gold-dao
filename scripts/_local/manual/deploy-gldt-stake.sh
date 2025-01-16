#!/usr/bin/env bash


###############
### FOR STAGING
###############

./scripts/build_canister.sh gldt_stake &&
./scripts/generate_did.sh gldt_stake &&
./scripts/build_canister.sh gldt_stake &&
# dfx deploy --network staging sns_rewards --argument '(record {test_mode = true})' --mode reinstall
dfx deploy --network staging gldt_stake --argument "(variant { Init = record {
    test_mode = true;
    version = record {
     major = 0:nat32;
     minor = 0:nat32;
     patch = 0:nat32;
    };
    commit_hash = \"stagingcommit\";
    authorized_principals = vec { principal \"465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae\" };
    gldt_ledger_id = principal \"6uad6-fqaaa-aaaam-abovq-cai\";
    gldgov_ledger_id = principal \"irhm6-5yaaa-aaaap-ab24q-cai\";
    gld_sns_rewards_canister_id = principal \"rbv23-fqaaa-aaaam-qbfma-cai\";
    gld_sns_governance_canister_id = principal \"j3ioe-7iaaa-aaaap-ab23q-cai\";
    reward_types = vec { 
        record {
            \"GLDGov\";
            1 = record {
                0 = principal \"irhm6-5yaaa-aaaap-ab24q-cai\";
                1 = 100000:nat;
            };
        };
        record {
            \"ICP\";
            1 = record {
                0 = principal \"ete3q-rqaaa-aaaal-qdlva-cai\";
                1 = 10000:nat;
            };
        };
        record {
            \"OGY\";
            1 = record {
                0 = principal \"jwcfb-hyaaa-aaaaj-aac4q-cai\";
                1 = 200000:nat; 
            };
        };
    };
}})" --mode reinstall

