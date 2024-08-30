use canister_client::generate_candid_c2c_call;
use sns_ledger_canister::*;

// Queries
generate_candid_c2c_call!(get_transactions);

// FIXME: implement here interfaces to get icrc3 balance to check the rewards amount
// generate_candid_c2c_call!(icrc3_balance_of);
