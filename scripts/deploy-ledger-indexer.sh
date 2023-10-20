dfx deploy --network staging gldt_ledger_indexer --argument '( opt variant{Init = record {ledger_id =principal "'"$(dfx canister id --network staging gldt_ledger)"'" }})'
