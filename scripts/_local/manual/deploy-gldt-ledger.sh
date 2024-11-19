dfx deploy --network staging gldt_ledger --mode reinstall --argument '(
  variant {
    Init = record {
      decimals = opt (8 : nat8);
      token_symbol = "GLDT";
      transfer_fee = 1_000_000 : nat;
      metadata = vec {};
      minting_account = record {
        owner = principal "m45be-jaaaa-aaaak-qcgnq-cai";
        subaccount = null;
      };
      initial_balances = vec {};
      maximum_number_of_accounts = null;
      accounts_overflow_trim_quantity = null;
      fee_collector_account = opt record {
        owner = principal "m45be-jaaaa-aaaak-qcgnq-cai";
        subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01";
      };
      archive_options = record {
        num_blocks_to_archive = 1_000 : nat64;
        max_transactions_per_response = null;
        trigger_threshold = 1_000 : nat64;
        more_controller_ids = null;
        max_message_size_bytes = null;
        cycles_for_archive_creation = null;
        node_max_memory_size_bytes = null;
        controller_id = principal "6uad6-fqaaa-aaaam-abovq-cai";
      };
      max_memo_length = null;
      token_name = "Gold Token";
      feature_flags = null;
    }
  },
)'


dfx deploy --network staging gldt_ledger_indexer --mode reinstall --argument '(
  opt variant {
    Init = record {
      ledger_id = principal "6uad6-fqaaa-aaaam-abovq-cai"
    }
  }
)'
