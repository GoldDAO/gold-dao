use candid::Nat;
pub use gldt_swap_api_canister::get_historic_swaps::{
    Args as GetHistoricSwapsArgs,
    Response as GetHistoricSwapsResponse,
};
use gldt_swap_api_canister::get_historic_swaps::GetHistoricSwapsError;
use gldt_swap_api_archive::get_archive_swaps::Args as GetArchiveSwapsArg;
use gldt_swap_archive_c2c_client::get_archive_swaps;
use ic_cdk::update;

use crate::state::read_state;

#[update]
async fn get_historic_swaps(args: GetHistoricSwapsArgs) -> GetHistoricSwapsResponse {
    let limit = args.limit.clone();
    let max_limit = 200usize;
    if &limit > &max_limit {
        return Err(
            GetHistoricSwapsError::LimitTooLarge(
                format!(
                    "The limit you passed ({limit}) is too large. The maximum limit is {max_limit}"
                )
            )
        );
    }
    // create a search index by taking the total and minus the args.start
    let mut total_swaps = read_state(|s| s.data.swaps.get_history_total());
    if total_swaps > Nat::from(0u64) {
        total_swaps -= Nat::from(1u64);
    }
    let swap_index_offset = args.page * limit; // 0 x 50 = 0 or 1 x 50 = 50 or 3 x 100 = start index of 300
    let mut start_index = total_swaps.clone();
    if total_swaps.clone() >= swap_index_offset {
        start_index -= swap_index_offset;
    } else {
        return Ok(vec![]);
    }

    // largest archive start index first
    let archives = read_state(|s| {
        let mut archives = s.data.swaps.get_archive_canisters().clone();
        archives.sort_by_key(|canister| canister.start_index.clone());
        archives.reverse();
        archives
    });
    // get the largest
    let initial_archive = archives.iter().find(|archive| start_index >= archive.start_index);

    let mut remaining = args.limit as usize;
    let mut swaps_to_return = Vec::new();
    let mut current_start_index = start_index.clone();

    if let Some(initial_archive) = initial_archive {
        // get the index of initial archive canister
        let mut archive_index = archives
            .iter()
            .position(|canister| canister.canister_id == initial_archive.canister_id)
            .unwrap_or(0);

        while remaining > 0 && archive_index < archives.len() {
            let archive = &archives[archive_index];

            match
                get_archive_swaps(
                    archive.canister_id,
                    &(GetArchiveSwapsArg {
                        start: current_start_index.clone(),
                        limit: remaining,
                        user_principal: None,
                    })
                ).await
            {
                Ok(mut swaps) => {
                    let count = swaps.len();
                    swaps_to_return.append(&mut swaps);
                    remaining = remaining.saturating_sub(count);

                    if remaining == 0 {
                        break;
                    }
                    if let Some(last_swap) = swaps_to_return.last() {
                        current_start_index = last_swap.0.1.clone();
                    }
                }
                Err(_) => {}
            }

            archive_index += 1;
        }
    }

    Ok(swaps_to_return)
}
