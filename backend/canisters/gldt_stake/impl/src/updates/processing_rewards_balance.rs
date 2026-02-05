use crate::guards::caller_is_governance_principal;
use crate::model::processing_rewards_pool::ProcessingRewards;
use crate::state::read_state;
use bity_ic_canister_tracing_macros::trace;
pub use gldt_stake_api_canister::processing_rewards_balance::{
    Args as ProcessingRewardsBalanceArgs, Response as ProcessingRewardsBalanceResponse,
};
use ic_cdk::update;
use std::collections::HashMap;
use tracing::error;

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn processing_rewards_balance(
    _args: ProcessingRewardsBalanceArgs,
) -> ProcessingRewardsBalanceResponse {
    let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());
    let processing_rewards_pool = read_state(|s| s.data.processing_rewards_pool.clone());

    let mut result = HashMap::new();

    for reward_token in reward_types {
        let token_ledger = reward_token.get_prod_token_info().ledger_id;
        match processing_rewards_pool.balance(token_ledger).await {
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
