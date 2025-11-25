use crate::{guards::caller_is_governance_principal, state::mutate_state};
use bity_ic_canister_tracing_macros::trace;
use ic_cdk::{query, update};
use ic_ledger_types::AccountIdentifier;
pub use icp_neuron_api_canister::manage_cycle_management_account::{
    ManageCycleManagementAccountResponse, ManageCycleManagementRequest,
};

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn manage_cycle_management_account(
    args: ManageCycleManagementRequest,
) -> ManageCycleManagementAccountResponse {
    match manage_cycle_management_account_impl(args).await {
        Ok(_) => ManageCycleManagementAccountResponse::Success,
        Err(err) => ManageCycleManagementAccountResponse::InternalError(err),
    }
}

pub(crate) async fn manage_cycle_management_account_impl(
    args: ManageCycleManagementRequest,
) -> Result<(), String> {
    match validate_manage_cycle_management_account(&args) {
        Ok(valid_accounts) => {
            mutate_state(|s| s.data.cycle_management_account = valid_accounts);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manage_cycle_management_account_validate(
    args: ManageCycleManagementRequest,
) -> Result<String, String> {
    validate_manage_cycle_management_account(&args)?;
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

fn validate_manage_cycle_management_account(
    args: &ManageCycleManagementRequest,
) -> Result<Vec<AccountIdentifier>, String> {
    args.account_identifier
        .iter()
        .map(|id| AccountIdentifier::from_hex(id))
        .collect()
}
