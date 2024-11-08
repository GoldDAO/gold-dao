use std::collections::HashMap;

use candid::Principal;
pub use gldt_swap_api_canister::get_historic_swaps_by_user::{
    Args as GetHistoricSwapsByUserArgs, Response as GetHistoricSwapsByUserResponse,
};
use gldt_swap_common::{
    archive::ArchiveCanister,
    swap::{SwapId, SwapInfo},
};

use gldt_swap_api_canister::get_historic_swaps_by_user::GetHistoricSwapsByUserError;
use gldt_swap_archive_c2c_client::get_swap_bulk;
use ic_cdk::update;

use crate::utils::get_all_user_swap_ids;

#[update]
async fn get_historic_swaps_by_user(
    args: GetHistoricSwapsByUserArgs,
) -> GetHistoricSwapsByUserResponse {
    let limit = args.limit.clone();
    let max_limit = 200usize;
    let min_limit = 1usize;
    if &limit > &max_limit {
        return Err(
            GetHistoricSwapsByUserError::LimitTooLarge(
                format!(
                    "The limit you passed ({limit}) is too large. The minimum is {min_limit} and the maximum limit is {max_limit}"
                )
            )
        );
    }
    if &limit < &min_limit {
        return Err(
            GetHistoricSwapsByUserError::LimitTooSmall(
                format!(
                    "The limit you passed ({limit}) is too small. The minimum is {min_limit} and the maximum limit is {max_limit}"
                )
            )
        );
    }

    // archives largest index to smallest index

    let mut all_user_swap_ids: Vec<(SwapId, ArchiveCanister)> =
        match get_all_user_swap_ids(&args.user).await {
            Ok(swap_ids) => swap_ids,
            Err(e) => {
                return Err(e);
            }
        };

    // no swap ids to get
    if all_user_swap_ids.len() == 0 {
        return Ok(vec![]);
    }

    // sort the swap ids, largest to smallest
    all_user_swap_ids.sort_by(|(SwapId(_, a), _), (SwapId(_, b), _)| b.cmp(a));
    let mut swaps_to_return: Vec<(SwapId, SwapInfo)> = vec![];

    // chunk by page size
    let pages_of_swap_ids: Vec<Vec<(SwapId, ArchiveCanister)>> = all_user_swap_ids
        .chunks(limit)
        .map(|chunk| chunk.to_vec()) // Convert slices to Vec
        .collect();

    if let Some(page_of_swap_ids_to_get) = pages_of_swap_ids.get(args.page) {
        let swaps_grouped = group_by_archive_canister_id(page_of_swap_ids_to_get);

        for (archive_canister_id, swap_ids) in swaps_grouped {
            match get_swap_bulk(archive_canister_id, &swap_ids).await {
                Ok(swaps) => {
                    let with_swap_id: Vec<(SwapId, SwapInfo)> = swaps
                        .into_iter()
                        .map(|swap_info| (swap_info.get_swap_id(), swap_info))
                        .collect();
                    swaps_to_return.extend(with_swap_id);
                }
                Err(e) => {
                    return Err(GetHistoricSwapsByUserError::QueryCanisterError(format!(
                        "{e:?}"
                    )));
                }
            }
        }
    } else {
        return Ok(vec![]);
    }
    swaps_to_return.sort_by(|(SwapId(_, a), _), (SwapId(_, b), _)| b.cmp(a));
    Ok(swaps_to_return)
}

fn group_by_archive_canister_id(
    swap_ids_and_archives: &Vec<(SwapId, ArchiveCanister)>,
) -> HashMap<Principal, Vec<SwapId>> {
    let mut grouped: HashMap<Principal, Vec<SwapId>> = HashMap::new();

    for (swap_id, archive) in swap_ids_and_archives {
        grouped
            .entry(archive.canister_id)
            .or_insert_with(Vec::new)
            .push(swap_id.clone());
    }

    grouped
}
