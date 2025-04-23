use std::collections::HashMap;

use candid::Principal;
use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
use futures::future::join_all;
pub use gldt_stake_api_canister::get_historic_positions_by_user::{
    Args as GetHistoricPositionsByUserArgs, Response as GetHistoricPositionsByUserResponse,
};
use gldt_stake_archive_c2c_client::{get_item_bulk, get_item_indexes_for_user};
use gldt_stake_common::{
    archive::ArchiveCanister,
    stake_position::{StakePositionId, StakePositionResponse},
};
use ic_cdk::update;

use crate::state::read_state;

#[update]
#[trace]
async fn get_historic_positions_by_user(
    args: GetHistoricPositionsByUserArgs,
) -> GetHistoricPositionsByUserResponse {
    get_historic_positions_by_user_impl(args).await
}

async fn get_historic_positions_by_user_impl(
    args: GetHistoricPositionsByUserArgs,
) -> GetHistoricPositionsByUserResponse {
    let GetHistoricPositionsByUserArgs { user, start, limit } = args;
    let item_ids_with_canister = fetch_all_position_ids_for_user(user).await;

    let items_paginated = item_ids_with_canister
        .into_iter()
        .skip(start)
        .take(limit)
        .collect();

    fetch_positions(group_by_archive_id(items_paginated)).await
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

async fn fetch_positions(
    grouped_positions: Vec<(Principal, Vec<StakePositionId>)>,
) -> Vec<StakePositionResponse> {
    // Create item requests
    let item_requests: Vec<_> = grouped_positions
        .into_iter()
        .map(|(archive_canister_id, ids)| get_item_bulk(archive_canister_id, ids))
        .collect();

    // Await all requests
    let item_results = join_all(item_requests).await;
    let now = timestamp_millis();

    // Process the results
    let mut positions: Vec<StakePositionResponse> = item_results
        .into_iter()
        .filter_map(|res| match res {
            Ok(positions) => Some(
                positions
                    .into_iter()
                    .map(|(id, position)| StakePositionResponse::from((position, now, id)))
                    .collect::<Vec<_>>(),
            ),
            Err(_) => None,
        })
        .flatten()
        .collect();

    positions.sort_by_key(|pos| pos.id);
    positions
}

fn group_by_archive_id(
    items_with_canister: Vec<(Principal, StakePositionId)>,
) -> Vec<(Principal, Vec<StakePositionId>)> {
    let mut map: HashMap<Principal, Vec<StakePositionId>> = HashMap::new();

    for (principal, id) in items_with_canister {
        map.entry(principal).or_insert_with(Vec::new).push(id);
    }

    map.into_iter().collect()
}
