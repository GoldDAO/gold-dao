use crate::guards::caller_is_governance_principal;
use crate::model::allocated_rewards_pool::AllocatedRewards;
use crate::state::read_state;
use bity_ic_canister_tracing_macros::trace;
pub use gldt_stake_api_canister::allocated_rewards_balance::{
    Args as AllocatedRewardsBalanceArgs, Response as AllocatedRewardsBalanceResponse,
};
use ic_cdk::update;
use std::collections::HashMap;
use tracing::error;

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn allocated_rewards_balance(
    _args: AllocatedRewardsBalanceArgs,
) -> AllocatedRewardsBalanceResponse {
    let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());
    let allocated_rewards_pool = read_state(|s| s.data.allocated_rewards_pool.clone());

    let mut result = HashMap::new();

    for reward_token in reward_types {
        let token_ledger = reward_token.get_token_info().ledger_id;
        match allocated_rewards_pool.balance(token_ledger).await {
            Ok(balance) => {
                result.insert(reward_token, Ok(balance));
            }
            Err(err) => {
                error!("Failed to get balance for {:?}: {}", reward_token, err);
                result.insert(reward_token, Err(err.to_string()));
            }
        }
    }

    result
}
