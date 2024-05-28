use crate::state::{mutate_state, read_state};
use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use sns_root_canister::get_sns_canisters_summary::CanisterSummary;
use std::time::Duration;
use tracing::error;
use types::{CanisterId, Cycles, Empty};
use utils::canister::deposit_cycles;
use utils::env::Environment;

const INTERVAL: Duration = Duration::from_secs(60); // Adjust as needed

const T: Cycles = 1_000_000_000_000;
const TOP_UP_THRESHOLD: u64 = 200 * T;

pub fn start_job() {
    run_now_then_interval(INTERVAL, run);
}

pub fn run() {
    if let Some(canister_id) = read_state(|state| state.data.sns_root_canister) {
        ic_cdk::spawn(run_async(canister_id));
    }
}

#[trace]
async fn run_async(canister_id: CanisterId) {
    match sns_root_canister_c2c_client::get_sns_canisters_summary(canister_id, &Empty {}).await {
        Ok(response) => {
            let canisters: Vec<_> = [
                response.root,
                response.governance,
                response.ledger,
                response.swap,
                response.index,
            ]
            .into_iter()
            .flatten()
            .chain(response.dapps)
            .chain(response.archives)
            .collect();

            // TODO: Fix with join_all()
            // Add SNS canisters to the whitelist
            mutate_state(|state| {
                let now = state.env.now();
                for canister_id in canisters.iter().flat_map(|c| c.canister_id) {
                    state.data.canisters.add(canister_id, now);
                }
            });

            let to_top_up: Vec<_> = canisters
                .into_iter()
                .filter(requires_top_up)
                .map(|s| s.canister_id.unwrap())
                .collect();

            if !to_top_up.is_empty() {
                let top_up_amount = read_state(|state| state.data.max_top_up_amount);

                let top_up_futures: Vec<_> = to_top_up
                    .clone()
                    .into_iter()
                    .map(|canister_id| deposit_cycles(canister_id, top_up_amount))
                    .collect();

                futures::future::join_all(top_up_futures).await;
            }
        }
        Err(e) => {
            error!("Failed to get SNS canisters summary: {:?}", e);
        }
    }
}

fn requires_top_up(summary: &CanisterSummary) -> bool {
    if let Some(status) = summary.status.as_ref() {
        let cycles = status.cycles.0.clone();
        cycles < TOP_UP_THRESHOLD.into()
    } else {
        false
    }
}
