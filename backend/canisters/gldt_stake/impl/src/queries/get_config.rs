use crate::state::read_state;
pub use gldt_stake_api_canister::get_config::{
    Args as GetConfigArgs, Response as GetConfigResponse,
};
use gldt_stake_common::stake_position::GLDT_STAKE_DISSOLVE_DELAY_MS;
use gldt_stake_common::stake_position::GLDT_STAKE_EARLY_UNSTAKE_FEE_PERCENTAGE;
use gldt_stake_common::stake_position::MAXIMUM_STAKE_AMOUNT;
use gldt_stake_common::stake_position::MAX_ACTIVE_EVENTS_PER_POSITION;
use gldt_stake_common::stake_position::MINIMUM_STAKE_AMOUNT;
use ic_cdk::query;

#[query]
fn get_config(_args: GetConfigArgs) -> GetConfigResponse {
    let reward_tokens = read_state(|s| {
        s.data
            .stake_system
            .reward_types
            .iter()
            .map(|token_symbol| token_symbol.symbol().to_string())
            .collect()
    });

    GetConfigResponse {
        reward_tokens,
        unlock_delay: GLDT_STAKE_DISSOLVE_DELAY_MS,
        early_unlock_fee: GLDT_STAKE_EARLY_UNSTAKE_FEE_PERCENTAGE,
        stake_limit_min: MINIMUM_STAKE_AMOUNT,
        stake_limit_max: MAXIMUM_STAKE_AMOUNT,
        max_dissolve_events: MAX_ACTIVE_EVENTS_PER_POSITION,
    }
}
