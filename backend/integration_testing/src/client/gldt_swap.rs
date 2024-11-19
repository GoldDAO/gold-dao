use crate::{generate_pocket_query_call, generate_pocket_update_call};

use gldt_swap_api_archive::*;
use gldt_swap_api_canister::*;

generate_pocket_update_call!(insert_fake_swap);
generate_pocket_update_call!(insert_fake_bulk_swaps);
generate_pocket_query_call!(get_active_swaps_by_user);
generate_pocket_update_call!(swap_tokens_for_nft);
generate_pocket_update_call!(recover_stuck_swap);
generate_pocket_query_call!(get_active_swap_ids_by_user);
generate_pocket_update_call!(get_swap);
generate_pocket_update_call!(get_historic_swaps);
generate_pocket_update_call!(swap_nft_for_tokens);
generate_pocket_update_call!(remove_intent_to_swap);
generate_pocket_query_call!(get_archive_canisters);
generate_pocket_update_call!(get_history_total);
generate_pocket_update_call!(get_historic_swaps_by_user);
generate_pocket_query_call!(get_version);
generate_pocket_query_call!(get_archive_swaps);
generate_pocket_query_call!(get_owned_nfts);
generate_pocket_query_call!(http_request);
generate_pocket_update_call!(force_toggle_gldt_supply_cron);
