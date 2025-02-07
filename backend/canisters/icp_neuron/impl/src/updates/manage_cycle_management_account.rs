use crate::{guards::caller_is_governance_principal, state::mutate_state};
use canister_tracing_macros::trace;
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
        Ok(valid_account) => {
            mutate_state(|s| s.data.cycle_management_account = Some(valid_account));
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
) -> Result<AccountIdentifier, String> {
    AccountIdentifier::from_hex(&args.account_identifier).map_err(|e| {
        format!("ERROR :: post_upgrade.rs :: cycle_management_account was not a valid hex string :: err - {e:?}")
    })
}
