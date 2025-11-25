use crate::{generate_pocket_query_call, generate_pocket_update_call};

use gldt_swap_index_api_canister::*;

generate_pocket_query_call!(ledger_id);
generate_pocket_query_call!(status);

generate_pocket_update_call!(get_blocks);
