use crate::state::mutate_state;
use crate::state::read_state;
pub use gldt_stake_api_canister::_set_position_withdraw_state::{
    Args as SetPositionWithdrawStateArgs, Response as SetPositionWithdrawStateResponse,
};

#[cfg(feature = "inttest")]
use crate::guards::caller_is_governance_principal;
#[cfg(feature = "inttest")]
use ic_cdk::update;

#[update(guard = "caller_is_governance_principal")]
#[cfg(feature = "inttest")]
async fn _set_position_withdraw_state(
    args: SetPositionWithdrawStateArgs,
) -> SetPositionWithdrawStateResponse {
    _set_position_withdraw_state_impl(args).await
}

async fn _set_position_withdraw_state_impl(
    args: SetPositionWithdrawStateArgs,
) -> SetPositionWithdrawStateResponse {
    let mut stake_position =
        read_state(|s| s.data.stake_system.get_stake_position(&args.principal)).unwrap();

    stake_position.withdraw_state = args.state;

    mutate_state(|s| {
        s.data
            .stake_system
            .upsert_stake_position(args.principal, stake_position)
    });

    Ok(())
}
