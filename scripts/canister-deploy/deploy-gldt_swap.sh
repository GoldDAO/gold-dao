#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="proposal"

. ./scripts/extract_commit_tag_data_and_commit_sha.sh gldt_swap $NETWORK

if [[ $REINSTALL == "reinstall" ]]; then

  if [[ $NETWORK =~ ^(local|staging)$ ]]; then
    TESTMODE=true
    AUTHORIZED_PRINCIPALS="vec {
      principal \"465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae\";
      principal \"$(dfx canister id --network $NETWORK sns_governance)\";
    }"
    BUYBACK_BURN_ACCOUNT_OPT="buyback_burn_canister = \"m45be-jaaaa-aaaak-qcgnq-cai.0000000000000000000000000000000000000000000000000000000000000001\""
  elif [[ $NETWORK =~ ^(ic)$ ]]; then
    TESTMODE=false
    AUTHORIZED_PRINCIPALS="vec {
      principal \"$(dfx canister id --network $NETWORK sns_governance)\";
    }"
    BUYBACK_BURN_ACCOUNT_OPT=""
  else
    echo "Error: unknown network '$NETWORK'."
    exit 2
  fi

  # Ledger canisters
  GLDT_LEDGER_ID="$(dfx canister id --network $NETWORK gldt_ledger)"
  OGY_LEDGER_ID="$(dfx canister id --network $NETWORK ogy_ledger)"

  # NFT canisters
  NFT_1G_ID="$(dfx canister id --network $NETWORK gldnft_backend_1g)"
  NFT_10G_ID="$(dfx canister id --network $NETWORK gldnft_backend_10g)"
  NFT_100G_ID="$(dfx canister id --network $NETWORK gldnft_backend_100g)"
  NFT_1000G_ID="$(dfx canister id --network $NETWORK gldnft_backend_1000g)"

  # Swap configs
  SWAP_CONFIGS="vec {
    record {
      icrc7_canister_id = principal \"$NFT_1G_ID\";
      fractionalization_config = variant {
        General = record {
          division = 10_000_000_000:nat64;
          swap_fee = 90_000_000:nat;
          ledger_id = principal \"$GLDT_LEDGER_ID\";
        }
      };
    };
    record {
      icrc7_canister_id = principal \"$NFT_10G_ID\";
      fractionalization_config = variant {
        General = record {
          division = 100_000_000_000:nat64;
          swap_fee = 90_000_000:nat;
          ledger_id = principal \"$GLDT_LEDGER_ID\";
        }
      };
    };
    record {
      icrc7_canister_id = principal \"$NFT_100G_ID\";
      fractionalization_config = variant {
        General = record {
          division = 1_000_000_000_000:nat64;
          swap_fee = 90_000_000:nat;
          ledger_id = principal \"$GLDT_LEDGER_ID\";
        }
      };
    };
    record {
      icrc7_canister_id = principal \"$NFT_1000G_ID\";
      fractionalization_config = variant {
        General = record {
          division = 10_000_000_000_000:nat64;
          swap_fee = 90_000_000:nat;
          ledger_id = principal \"$GLDT_LEDGER_ID\";
        }
      };
    };
  }"

  # ICRC3 config
  ICRC3_SUPPORTED_BLOCKS="vec {
    record { block_type = \"forward_swap\"; url = \"https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-3/README.md#supported-block-types\"; };
    record { block_type = \"reverse_swap\"; url = \"https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-3/README.md#supported-block-types\"; };
    record { block_type = \"forward_swap_old\"; url = \"https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-3/README.md#supported-block-types\"; };
    record { block_type = \"reverse_swap_old\"; url = \"https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-3/README.md#supported-block-types\"; };
  }"

  ICRC3_CONSTANTS="record {
    tx_window = record { secs = 86400 : nat64; nanos = 0 : nat32; };
    max_transactions_in_window = 1000 : nat;
    max_memory_size_bytes = 1073741824 : nat;
    max_blocks_per_response = 100 : nat;
    initial_cycles = 5_000_000_000_000 : nat;
    reserved_cycles = 2_000_000_000_000 : nat;
    max_transactions_to_purge = 500 : nat;
    ttl_for_non_archived_transactions = record { secs = 120 : nat64; nanos = 0 : nat32; };
    max_unarchived_transactions = 1000 : nat;
  }"

  ICRC3_CONFIG="record {
    ledger_id = principal \"$OGY_LEDGER_ID\";
    supported_blocks = $ICRC3_SUPPORTED_BLOCKS;
    constants = $ICRC3_CONSTANTS;
  }"

  # Init args
  ARGUMENTS="(variant { Init = record {
    test_mode = $TESTMODE;
    commit_hash = \"$COMMIT_SHA\";
    version = $BUILD_VERSION;
    swap_configs = $SWAP_CONFIGS;
    authorized_principals = $AUTHORIZED_PRINCIPALS;
    $BUYBACK_BURN_ACCOUNT_OPT;
    icrc3_config = $ICRC3_CONFIG;
  }})"

else
  ARGUMENTS="(variant { Upgrade = record {
    version = $BUILD_VERSION;
    commit_hash = \"$COMMIT_SHA\";
  }})"
fi


echo $ARGUMENTS

. ./scripts/deploy_backend_canister.sh gldt_swap $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA $VERSION $REINSTALL
