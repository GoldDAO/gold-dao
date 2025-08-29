use crate::guards::caller_is_governance_principal;
use crate::jobs::allocate_rewards::handle_process_and_allocation;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::manual_allocate_rewards::{
    Args as ManualAllocateRewardsArgs, Response as ManualAllocateRewardsResponse,
};
use ic_cdk::query;
use ic_cdk::update;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manual_allocate_rewards_validate(
    _args: ManualAllocateRewardsArgs,
) -> Result<String, String> {
    serde_json::to_string_pretty(&_args).map_err(|_| "invalid payload".to_string())
}
#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manual_allocate_rewards(
    _args: ManualAllocateRewardsArgs,
) -> ManualAllocateRewardsResponse {
    ic_cdk::futures::spawn(handle_process_and_allocation());
}
