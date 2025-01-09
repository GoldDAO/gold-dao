use crate::guards::caller_is_governance_principal;
use crate::state::mutate_state;
use crate::state::read_state;
pub use gldt_stake_api_canister::_set_position_unstake_state::{
    Args as SetPositionUnstakeStateArgs, Response as SetPositionUnstakeStateResponse,
};
use gldt_stake_common::reward_round::RewardRoundStatus;
use ic_cdk::update;

#[update(guard = "caller_is_governance_principal")]
#[cfg(feature = "inttest")]
async fn _set_position_unstake_state(
    args: SetPositionUnstakeStateArgs,
) -> SetPositionUnstakeStateResponse {
    _set_position_unstake_state_impl(args).await
}

async fn _set_position_unstake_state_impl(
    args: SetPositionUnstakeStateArgs,
) -> SetPositionUnstakeStateResponse {
    let mut stake_position =
        read_state(|s| s.data.stake_system.get_stake_position(args.id)).unwrap();
    stake_position.unstake_state = args.state;

    mutate_state(|s| {
        s.data
            .stake_system
            .update_stake_position(&args.id, stake_position)
    });

    Ok(())
}
