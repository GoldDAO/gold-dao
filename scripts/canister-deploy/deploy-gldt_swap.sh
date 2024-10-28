#!/usr/bin/env bash

NETWORK=$1

DEPLOYMENT_VIA="direct"

. ./scripts/extract_commit_tag_data_and_commit_sha.sh gldt_swap $NETWORK

if [[ $REINSTALL == "reinstall" ]]; then

  if [[ $NETWORK =~ ^(local|staging)$ ]]; then
    TESTMODE=true
    AUTHORIZED_PRINCIPALS="vec { principal \"465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae\"; principal \"$(dfx canister id --network $NETWORK sns_governance)\" }"
    GLD_NFT_CANISTERS="vec {
      record {
        0 = principal \"$(dfx canister id --network $NETWORK gldnft_backend_1g)\";
        1 = record {
          grams = 1:nat16;
        };
      };
      record {
        0 = principal \"$(dfx canister id --network $NETWORK gldnft_backend_10g)\";
        1 = record {
          grams = 10:nat16;
        };
      };
      record {
       0 = principal \"$(dfx canister id --network $NETWORK gldnft_backend_100g)\";
       1 = record {
         grams = 100;
       };
      };
      record {
       0 = principal \"$(dfx canister id --network $NETWORK gldnft_backend_1000g)\";
       1 = record {
         grams = 1000;
       };
      };
    }"
  elif [[ $NETWORK =~ ^(ic)$ ]]; then
    AUTHORIZED_PRINCIPALS="vec { principal \"$(dfx canister id --network $NETWORK sns_governance)\" }"
    TESTMODE=false
    GLD_NFT_CANISTERS="vec {
      record {
        0 = principal \"$(dfx canister id --network $NETWORK gldnft_backend_1g)\";
        1 = record {
          grams = 1:nat16;
        };
      };
      record {
        0 = principal \"$(dfx canister id --network $NETWORK gldnft_backend_10g)\";
        1 = record {
          grams = 10:nat16;
        };
      };
      record {
        0 = principal \"$(dfx canister id --network $NETWORK gldnft_backend_100g)\";
        1 = record {
          grams = 100:nat16;
        };
      };
      record {
        0 = principal \"$(dfx canister id --network $NETWORK gldnft_backend_1000g)\";
        1 = record {
          grams = 1000:nat16;
        };
      };
    }"
  else
    echo "Error: unknown network for deployment. Found $NETWORK."
    exit 2
  fi

  GLDT_LEDGER_CANISTER_ID="$(dfx canister id --network $NETWORK gldt_ledger)"
  OGY_LEDGER_CANISTER_ID="$(dfx canister id --network $NETWORK ogy_ledger)"

  ARGUMENTS="(variant { Init = record {
    test_mode = $TESTMODE;
    commit_hash = \"$COMMIT_SHA\";
    version = $BUILD_VERSION;
    gldt_ledger_id = principal \"$GLDT_LEDGER_CANISTER_ID\";
    gldnft_canisters = $GLD_NFT_CANISTERS;
    ogy_ledger_id = principal \"$OGY_LEDGER_CANISTER_ID\";
    authorized_principals = $AUTHORIZED_PRINCIPALS;
  }})"

else
  ARGUMENTS="(variant { Upgrade = record {
    version = $BUILD_VERSION;
    commit_hash = \"$COMMIT_SHA\";
  }})"
fi

echo $ARGUMENTS

. ./scripts/deploy_backend_canister.sh gldt_swap $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA $VERSION $REINSTALL
