use crate::guards::caller_is_governance_principal;
use crate::state::mutate_state;
use crate::state::RuntimeState;
use bity_ic_canister_tracing_macros::trace;
pub use gldt_stake_api_canister::set_dissolve_event_time::Args as SetDissolveEventTimeArgs;
pub use gldt_stake_api_canister::set_dissolve_event_time::Response as SetDissolveEventTimeResponse;
use ic_cdk::update;

#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn set_dissolve_event_time(args: SetDissolveEventTimeArgs) -> SetDissolveEventTimeResponse {
    let mut new_stake_position = mutate_state(|s: &mut RuntimeState| {
        s.data
            .stake_system
            .stakes
            .get(&args.stake_position_user)
            .unwrap()
            .clone()
    });

    let mut dissolve_events = new_stake_position
        .dissolve_events
        .remove(args.dissovle_event_id as usize);

    dissolve_events.dissolved_date = args.new_dissolve_timestamp;
    new_stake_position.dissolve_events.push(dissolve_events);

    mutate_state(|s: &mut crate::state::RuntimeState| {
        s.data
            .stake_system
            .upsert_stake_position(args.stake_position_user, new_stake_position.clone());
    });

    Ok(())
}
