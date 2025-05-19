use candid::Principal;
use ic_cdk_macros::query;
pub use management_api_canister::dex_transfer_position_validate::Args as DexTransferPositionValidateArgs;
pub use management_api_canister::dex_transfer_position_validate::Response as DexTransferPositionValidateResponse;

#[query]
pub fn dex_transfer_position_validate(
    args: DexTransferPositionValidateArgs,
) -> DexTransferPositionValidateResponse {
    // hardcode this because we only need it once and will be deleted after
    let whitelist_principal =
        Principal::from_text("4ssjd-rq3dn-htkn3-4rkh4-sau3m-ldfdv-yhnee-chxs4-cvh5f-d2pxp-qqe")
            .unwrap();

    let receiving_principal = args.1;

    if receiving_principal != whitelist_principal {
        return Err(format!(
            "The receiving principal {receiving_principal} is not whitelisted. The only whitelisted principal is {whitelist_principal}"
        ));
    }
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}
