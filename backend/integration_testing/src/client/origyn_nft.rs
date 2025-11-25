use crate::{generate_pocket_query_call, generate_pocket_update_call};
use origyn_nft_canister::*;

// ICRC10
generate_pocket_query_call!(icrc10_supported_standards);

// ICRC21
generate_pocket_query_call!(icrc21_canister_call_consent_message);

// ICRC3
generate_pocket_query_call!(icrc3_get_archives);
generate_pocket_query_call!(icrc3_get_blocks);
generate_pocket_query_call!(icrc3_get_properties);
generate_pocket_query_call!(icrc3_get_tip_certificate);
generate_pocket_query_call!(icrc3_supported_block_types);

// ICRC37
generate_pocket_update_call!(icrc37_approve_tokens);
generate_pocket_update_call!(icrc37_approve_collection);
generate_pocket_update_call!(icrc37_revoke_token_approvals);
generate_pocket_update_call!(icrc37_revoke_collection_approvals);
generate_pocket_update_call!(icrc37_transfer_from);
generate_pocket_query_call!(icrc37_is_approved);
generate_pocket_query_call!(icrc37_max_approvals_per_token_or_collection);
generate_pocket_query_call!(icrc37_max_revoke_approvals);
// generate_pocket_query_call!(icrc37_get_token_approvals);
// generate_pocket_query_call!(icrc37_get_collection_approvals);

// ICRC7
generate_pocket_update_call!(icrc7_transfer);
generate_pocket_update_call!(icrc7_atomic_batch_transfers);
generate_pocket_query_call!(icrc7_collection_metadata);
generate_pocket_query_call!(icrc7_balance_of);
generate_pocket_query_call!(icrc7_owner_of);
generate_pocket_query_call!(icrc7_token_metadata);
generate_pocket_query_call!(icrc7_default_take_value);
generate_pocket_query_call!(icrc7_max_memo_size);
generate_pocket_query_call!(icrc7_description);
generate_pocket_query_call!(icrc7_logo);
generate_pocket_query_call!(icrc7_max_query_batch_size);
generate_pocket_query_call!(icrc7_max_take_value);
generate_pocket_query_call!(icrc7_max_update_batch_size);
generate_pocket_query_call!(icrc7_name);
generate_pocket_query_call!(icrc7_permitted_drift);
generate_pocket_query_call!(icrc7_supply_cap);
generate_pocket_query_call!(icrc7_symbol);
generate_pocket_query_call!(icrc7_total_supply);
generate_pocket_query_call!(icrc7_tx_window);

// Management methods
generate_pocket_update_call!(mint);
generate_pocket_update_call!(burn_nft);
generate_pocket_update_call!(update_nft_metadata);
generate_pocket_update_call!(update_minting_authorities);
generate_pocket_update_call!(remove_minting_authorities);
generate_pocket_update_call!(update_authorized_principals);
generate_pocket_update_call!(remove_authorized_principals);
generate_pocket_update_call!(update_collection_metadata);
generate_pocket_update_call!(init_upload);
generate_pocket_update_call!(store_chunk);
generate_pocket_update_call!(finalize_upload);
generate_pocket_update_call!(cancel_upload);
generate_pocket_query_call!(get_upload_status);
// generate_pocket_query_call!(get_all_uploads);
