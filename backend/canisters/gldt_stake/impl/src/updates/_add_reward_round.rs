use crate::guards::caller_is_governance_principal;
use crate::state::mutate_state;
use crate::state::read_state;
use crate::utils::commit_changes;
use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::_add_reward_round::{
    Args as AddRewardRoundArgs, Response as AddRewardRoundResponse,
};
use gldt_stake_common::reward_round::RewardRoundStatus;
use ic_cdk::update;

#[update(guard = "caller_is_governance_principal")]
#[cfg(feature = "inttest")]
async fn _add_reward_round(args: AddRewardRoundArgs) -> AddRewardRoundResponse {
    _add_reward_round_impl(args).await
}

async fn _add_reward_round_impl(args: AddRewardRoundArgs) -> AddRewardRoundResponse {
    args.into_iter().for_each(|(token_symbol, reward)| {
        mutate_state(|s| {
            s.data
                .reward_system
                .add_reward_round(reward, token_symbol, timestamp_millis())
        });
    });

    Ok(format!("fake round added successfully"))
}
