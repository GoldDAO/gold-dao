use crate::{generate_pocket_query_call, generate_pocket_update_call};
use sns_root_canister::{get_sns_canisters_summary, register_dapp_canisters};

// Queries
generate_pocket_query_call!(get_sns_canisters_summary);

// Updates
generate_pocket_update_call!(register_dapp_canisters);
