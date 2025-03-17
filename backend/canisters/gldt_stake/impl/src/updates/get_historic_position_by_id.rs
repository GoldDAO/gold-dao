use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
use futures::future::join_all;
pub use gldt_stake_api_canister::get_historic_position_by_id::{
    Args as GetHistoricPositionByIdArgs, Response as GetHistoricPositionByIdResponse,
};
use gldt_stake_archive_c2c_client::get_archive_item;
use ic_cdk::update;

use crate::state::read_state;

#[update]
#[trace]
async fn get_historic_position_by_id(
    position_id: GetHistoricPositionByIdArgs,
) -> GetHistoricPositionByIdResponse {
    get_historic_position_by_id_impl(position_id).await
}

async fn get_historic_position_by_id_impl(
    position_id: GetHistoricPositionByIdArgs,
) -> GetHistoricPositionByIdResponse {
    let archive_canisters = read_state(|s| s.data.archive_system.get_archive_canisters());
    let tasks: Vec<_> = archive_canisters
        .into_iter()
        .map(|archive| async move { get_archive_item(archive.canister_id, position_id).await })
        .collect();

    // Await all tasks
    let results = join_all(tasks).await;
    for result in results {
        match result {
            Ok(position_option) => match position_option {
                Some((stake_position_id, stake_position)) => {
                    return Some((stake_position, timestamp_millis(), stake_position_id).into())
                }
                None => {}
            },
            Err(_) => {}
        }
    }
    None
}
