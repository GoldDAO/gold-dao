use crate::guards::caller_is_governance_principal;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::manual_token_transfer::Args as ManualTokenTransferArgs;
pub use gldt_stake_api_canister::manual_token_transfer::Response as ManualTokenTransferResponse;
use ic_cdk::query;
use ic_cdk::update;
use icrc_ledger_canister_c2c_client::icrc1_transfer;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manual_token_transfer_validate(args: ManualTokenTransferArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manual_token_transfer(args: ManualTokenTransferArgs) -> ManualTokenTransferResponse {
    icrc1_transfer(args.ledger_id, &args.transfer_args).await
}
