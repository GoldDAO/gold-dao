use crate::guards::caller_is_governance_principal;
use crate::jobs::claim_neuron_rewards::claim_rewards;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::manual_claim_rewards::{
    Args as ManualClaimRewardsArgs, Response as ManualClaimRewardsResponse,
};
use ic_cdk::query;
use ic_cdk::update;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manual_claim_rewards_validate(_args: ManualClaimRewardsArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&_args).map_err(|_| "invalid payload".to_string())
}
#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manual_claim_rewards(_args: ManualClaimRewardsArgs) -> ManualClaimRewardsResponse {
    ic_cdk::futures::spawn(claim_rewards());
}
