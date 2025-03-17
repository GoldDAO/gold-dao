use candid::Principal;
use canister_tracing_macros::trace;
use futures::future::join_all;
pub use gldt_stake_api_canister::get_historic_positions_total_by_user::{
    Args as GetHistoricPositionsTotalByUserArgs,
    Response as GetHistoricPositionsTotalByUserResponse,
};
use gldt_stake_archive_c2c_client::get_item_indexes_for_user;
use gldt_stake_common::{archive::ArchiveCanister, stake_position::StakePositionId};
use ic_cdk::{caller, update};

use crate::state::read_state;

#[update]
#[trace]
async fn get_historic_positions_total_by_user(
    args: GetHistoricPositionsTotalByUserArgs,
) -> GetHistoricPositionsTotalByUserResponse {
    get_historic_positions_total_by_user_impl(args).await
}

async fn get_historic_positions_total_by_user_impl(
    args: GetHistoricPositionsTotalByUserArgs,
) -> GetHistoricPositionsTotalByUserResponse {
    let user = args.unwrap_or(caller());
    let item_ids_with_canister = fetch_all_position_ids_for_user(user).await;
    item_ids_with_canister.len()
}

async fn fetch_all_position_ids_for_user(user: Principal) -> Vec<(Principal, StakePositionId)> {
    let archive_canisters = read_state(|s| s.data.archive_system.get_archive_canisters());
    let item_ids_requests: Vec<_> = archive_canisters
        .clone()
        .into_iter()
        .map(|archive| get_item_indexes_for_user(archive.canister_id, &(user)))
        .collect();

    // Await all tasks
    let item_id_results = join_all(item_ids_requests).await;
    let mut item_ids_with_archive_id: Vec<(Principal, StakePositionId)> = item_id_results
        .into_iter()
        .enumerate()
        .filter_map(|(index, result)| {
            let archive: ArchiveCanister = archive_canisters.get(index).cloned()?;
            match result {
                Ok(ids) if !ids.is_empty() => Some((archive.canister_id, ids)),
                _ => None, // Skip empty or erroneous results
            }
        })
        .flat_map(|(principal, ids)| ids.into_iter().map(move |id| (principal, id)))
        .collect();

    item_ids_with_archive_id.sort_by_key(|&(_, id)| id);
    item_ids_with_archive_id
}
