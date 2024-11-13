use canister_client::generate_candid_c2c_call;
use icp_ledger_canister::*;

// Updates
generate_candid_c2c_call!(transfer);
generate_candid_c2c_call!(account_balance);
