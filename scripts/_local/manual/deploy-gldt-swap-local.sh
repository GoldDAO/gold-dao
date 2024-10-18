#!/usr/bin/env bash


###############
### FOR STAGING
###############

./scripts/build-canister.sh gldt_swap &&
./scripts/generate-did.sh gldt_swap &&
./scripts/build-canister.sh gldt_swap &&
# dfx deploy --network staging sns_rewards --argument '(record {test_mode = true})' --mode reinstall
dfx deploy gldt_swap --argument "(record {
    test_mode = true;
    version = \"0.0.1\";
    gldt_ledger_id = principal \"6uad6-fqaaa-aaaam-abovq-cai\";
    gldnft_canisters = vec {
      record {
        0 = principal \"obapm-2iaaa-aaaak-qcgca-cai\";
        1 = record {
          grams = 1;
        };
      };
      record {
        0 = principal \"xyo2o-gyaaa-aaaal-qb55a-cai\";
        1 = record {
          grams = 10;
        };
      };
    };
    ogy_ledger_id = principal \"j5naj-nqaaa-aaaal-ajc7q-cai\";
    authorized_principals = vec { principal \"465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae\" };
})"

