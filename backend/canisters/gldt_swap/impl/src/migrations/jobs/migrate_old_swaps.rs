use crate::model::swap_transaction::SwapTransaction;
use crate::state::{mutate_state, read_state};
use bity_ic_canister_time::{run_once, timestamp_millis};
use candid::Nat;
use gldt_swap_api_archive::get_archive_swaps;
use gldt_swap_archive_c2c_client;
use tracing::{debug, error, info};

pub fn start_job() {
    run_once(run);
}

pub fn run() {
    ic_cdk::futures::spawn(run_async());
}

async fn run_async() {
    migrate_neuron_maturity_data().await;
}

pub async fn migrate_neuron_maturity_data() {
    info!("Migration from old archives started");

    if !read_state(|s| s.get_is_migrating()) {
        info!("Migration flag is off, skipping...");
        return;
    }

    migrate_old_swaps().await;

    mutate_state(|state| {
        state.data.migration_finished = Some(timestamp_millis());
    });
}

pub async fn migrate_old_swaps() {
    let archive_canister = read_state(|s| s.data.old_archive_canister);
    let is_test_mode = read_state(|s| s.env.is_test_mode());

    let mut number_of_scanned_swaps = 0;
    let limit: usize = 100;

    // Start from a high index — this means "give me the latest N swaps"
    let mut start = Nat::from(110_u64); // just a very high guess, API will cap automatically

    loop {
        let args = get_archive_swaps::Args {
            start: start.clone(),
            limit,
            user_principal: None,
        };

        let swaps =
            match gldt_swap_archive_c2c_client::get_archive_swaps(archive_canister, &args).await {
                Ok(swaps) => swaps,
                Err(err) => {
                    error!(?err, "Error fetching archive swaps");
                    return;
                }
            };

        if swaps.is_empty() {
            break; // reached the beginning of archive
        }

        debug!(
            count = swaps.len(),
            ?start,
            "Received batch of swaps (descending)"
        );

        // Reorder batch to ascending before adding
        let mut ascending_swaps = swaps.clone();
        ascending_swaps.reverse();

        for (swap_id, swap_info) in ascending_swaps {
            let transaction = SwapTransaction::migrate(swap_info);

            if let Err(e) = crate::state::icrc3_add_transaction(transaction) {
                error!(?swap_id, ?e, "Failed to add swap transaction to archive");
                return;
            }

            number_of_scanned_swaps += 1;
        }

        // Prepare next `start` (take the lowest index we got and subtract 1)
        if let Some((lowest_id, _)) = swaps.last() {
            if lowest_id.1 > Nat::from(0_u32) {
                start = lowest_id.1.clone() - Nat::from(1_u32);
            } else {
                break; // reached index 0
            }
        }

        // Limit scanning in test mode
        if is_test_mode && number_of_scanned_swaps >= 100 {
            info!("Stopped migration early due to test mode limit");
            break;
        }
    }

    info!("Successfully migrated {number_of_scanned_swaps} swaps.");
}
