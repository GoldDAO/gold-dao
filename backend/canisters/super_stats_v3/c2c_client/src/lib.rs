use canister_client::{ generate_candid_c2c_call, generate_candid_c2c_call_no_args };
use super_stats_v3_api::stats::queries::*;

// Queries
generate_candid_c2c_call!(get_top_account_holders);
generate_candid_c2c_call!(get_top_principal_holders);
generate_candid_c2c_call!(get_account_overview);
generate_candid_c2c_call!(get_principal_overview);
generate_candid_c2c_call!(get_principal_holders);
generate_candid_c2c_call!(get_account_holders);
generate_candid_c2c_call!(get_account_history);
generate_candid_c2c_call!(get_principal_history);
generate_candid_c2c_call_no_args!(get_total_holders);
generate_candid_c2c_call_no_args!(get_hourly_stats);
generate_candid_c2c_call_no_args!(get_daily_stats);

// Updates
