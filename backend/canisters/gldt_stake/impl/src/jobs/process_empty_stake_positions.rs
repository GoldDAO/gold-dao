use crate::state::mutate_state;
use bity_ic_canister_time::DAY_IN_MS;
use bity_ic_canister_time::run_now_then_interval;
use std::time::Duration;
use tracing::info;

pub fn start_job() {
    run_now_then_interval(
        Duration::from_millis(7 * DAY_IN_MS),
        spawn_process_empty_stake_positions_job,
    );
}

fn spawn_process_empty_stake_positions_job() {
    ic_cdk::futures::spawn(process_empty_stake_positions_impl())
}

async fn process_empty_stake_positions_impl() {
    let _span = tracing::info_span!("PROCESS_EMPTY_STAKE_POSITIONS").entered();
    info!("start");

    let _removed_stake_positions =
        mutate_state(|s| s.data.stake_system.remove_all_empty_stake_positions());

    info!("finished");
}
