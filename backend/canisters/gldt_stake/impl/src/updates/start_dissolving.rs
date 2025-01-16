use candid::Principal;
use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::start_dissolving::{
    Args as StartDissolvingArgs, Response as StartDissolvingResponse,
};
use gldt_stake_common::stake_position::{DissolveState, StartDissolvingErrors};

use crate::state::{mutate_state, read_state};
use ic_cdk::{caller, update};

#[update]
#[trace]
async fn start_dissolving(position_id: StartDissolvingArgs) -> StartDissolvingResponse {
    start_dissolving_impl(position_id).await
}

async fn start_dissolving_impl(position_id: StartDissolvingArgs) -> StartDissolvingResponse {
    // 1. check user isn't anon
    let caller = caller();
    if caller == Principal::anonymous() {
        return Err(StartDissolvingErrors::InvalidPrincipal(format!(
            "You may not use an anonymous principal"
        )));
    }

    // find the position
    let mut position = read_state(|s| s.data.stake_system.get_stake_position(position_id)).ok_or(
        StartDissolvingErrors::NotFound(format!(
            "Cant find active stake position with ID : {position_id}"
        )),
    )?;

    if position.owned_by != caller {
        return Err(StartDissolvingErrors::NotAuthorized(format!(
            "You do not have permission to dissolve this stake position"
        )));
    }
    position
        .can_start_dissolving()
        .map_err(|e| StartDissolvingErrors::StakePositionError(e))?;

    position.dissolve_state = DissolveState::Dissolving;
    position.dissolved_date = Some(timestamp_millis() + position.dissolve_delay.as_millis() as u64);
    mutate_state(|s| {
        s.data
            .stake_system
            .update_stake_position(&position_id, position.clone())
    });
    Ok((position, timestamp_millis(), position_id).into())
}
