use bity_ic_canister_client::canister_client_macros::anyhow::anyhow;
use bity_ic_canister_client::generate_candid_c2c_call;
use bity_ic_canister_client::generate_candid_c2c_call_tuple_args;
use candid::Encode;
use candid::Principal;
use ic_cdk::call::Call;
use origyn_nft_canister::*;

// ICRC10
generate_candid_c2c_call!(icrc10_supported_standards);

// ICRC21
generate_candid_c2c_call!(icrc21_canister_call_consent_message);

// ICRC3
generate_candid_c2c_call!(icrc3_get_archives);
generate_candid_c2c_call!(icrc3_get_blocks);
generate_candid_c2c_call!(icrc3_get_properties);
generate_candid_c2c_call!(icrc3_get_tip_certificate);
generate_candid_c2c_call!(icrc3_supported_block_types);

// ICRC37
generate_candid_c2c_call!(icrc37_approve_tokens);
generate_candid_c2c_call!(icrc37_approve_collection);
generate_candid_c2c_call!(icrc37_revoke_token_approvals);
generate_candid_c2c_call!(icrc37_revoke_collection_approvals);
generate_candid_c2c_call!(icrc37_transfer_from);
generate_candid_c2c_call!(icrc37_is_approved);
generate_candid_c2c_call!(icrc37_max_approvals_per_token_or_collection);
generate_candid_c2c_call!(icrc37_max_revoke_approvals);

// TODO: Enable these calls once we have a use case for them
// generate_candid_c2c_call_tuple_args!(icrc37_get_token_approvals);
// generate_candid_c2c_call_tuple_args!(icrc37_get_collection_approval);
// icrc37_get_token_approvals
// icrc37_get_collection_approval

// ICRC7
generate_candid_c2c_call!(icrc7_transfer);
generate_candid_c2c_call!(icrc7_collection_metadata);
generate_candid_c2c_call!(icrc7_balance_of);
generate_candid_c2c_call!(icrc7_owner_of);
generate_candid_c2c_call!(icrc7_token_metadata);
generate_candid_c2c_call!(icrc7_atomic_batch_transfers);
generate_candid_c2c_call!(icrc7_default_take_value);
generate_candid_c2c_call!(icrc7_max_memo_size);
generate_candid_c2c_call!(icrc7_description);
generate_candid_c2c_call!(icrc7_logo);
generate_candid_c2c_call!(icrc7_max_query_batch_size);
generate_candid_c2c_call!(icrc7_max_take_value);
generate_candid_c2c_call!(icrc7_max_update_batch_size);
generate_candid_c2c_call!(icrc7_name);
generate_candid_c2c_call!(icrc7_permitted_drift);
generate_candid_c2c_call!(icrc7_supply_cap);
generate_candid_c2c_call!(icrc7_symbol);
generate_candid_c2c_call!(icrc7_total_supply);
generate_candid_c2c_call!(icrc7_tx_window);

generate_candid_c2c_call_tuple_args!(icrc7_tokens_of);

// Management methods
generate_candid_c2c_call!(mint);
generate_candid_c2c_call!(burn_nft);
generate_candid_c2c_call!(update_nft_metadata);
generate_candid_c2c_call!(update_minting_authorities);
generate_candid_c2c_call!(remove_minting_authorities);
generate_candid_c2c_call!(update_authorized_principals);
generate_candid_c2c_call!(remove_authorized_principals);
generate_candid_c2c_call!(get_upload_status);
generate_candid_c2c_call!(update_collection_metadata);
generate_candid_c2c_call!(init_upload);
generate_candid_c2c_call!(store_chunk);
generate_candid_c2c_call!(finalize_upload);
generate_candid_c2c_call!(cancel_upload);

pub async fn get_all_uploads(
    canister_id: Principal,
    start: get_all_uploads::Args0,
    limit: get_all_uploads::Args1,
) -> ::bity_ic_canister_client::Result<get_all_uploads::Response> {
    let args = Encode!(&start, &limit).map_err(|e| {
        anyhow!(
            "Failed to encode arguments for `get_all_uploads` for {:?}: {:?}",
            canister_id,
            e
        )
    })?;

    match Call::bounded_wait(canister_id, "get_all_uploads")
        .with_arg(&args)
        .await
    {
        Err(e) => Err(anyhow!(
            "Call `get_all_uploads` for {:?} failed: {:?}",
            canister_id,
            e
        )),
        Ok(response) => match response.candid::<get_all_uploads::Response>() {
            Ok(res) => Ok(res),
            Err(e) => Err(anyhow!(
                "Decoding `get_all_uploads::Response` for {:?} failed: {:?}",
                canister_id,
                e
            )),
        },
    }
}
