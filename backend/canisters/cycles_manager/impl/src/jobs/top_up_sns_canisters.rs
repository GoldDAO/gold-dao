use crate::state::{mutate_state, read_state};
use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use sns_root_canister::get_sns_canisters_summary::CanisterSummary;
use std::time::Duration;
use tracing::error;
use types::{CanisterId, Cycles, Empty};
use utils::canister::deposit_cycles;

const INTERVAL: Duration = Duration::from_secs(60); // 12 hours

const T: Cycles = 1_000_000_000_000;
const TOP_UP_THRESHOLD: u64 = 200 * T;

pub fn start_job() {
    run_now_then_interval(INTERVAL, run);
}

fn run() {
    if let Some(canister_id) = read_state(|state| state.data.sns_root_canister) {
        ic_cdk::spawn(run_async(canister_id));
    }
}

#[trace]
async fn run_async(canister_id: CanisterId) {
    if let Ok(response) =
        sns_root_canister_c2c_client::get_sns_canisters_summary(canister_id, &Empty {}).await
    {
        ic_cdk::println!("Got SNS canisters summary: {:#?}", response);
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

        ic_cdk::println!("Canisters to TOP UP: {:#?}", to_top_up);

        if !to_top_up.is_empty() {
            let top_up_amount = read_state(|state| state.data.max_top_up_amount);

            for canister_id in to_top_up {
                let _ = deposit_cycles(canister_id, top_up_amount).await;
            }
        }
    } else {
        error!("Failed to get SNS canisters summary");
    }
}

// TODO: fix here: Panicked at 'called `Option::unwrap()` on a `None` value'
// fn requires_top_up(summary: &CanisterSummary) -> bool {
//     let cycles: Cycles = summary
//         .status
//         .as_ref()
//         .unwrap()
//         .cycles
//         .0
//         .clone()
//         .try_into()
//         .unwrap();

//     cycles < TOP_UP_THRESHOLD
// }

fn requires_top_up(summary: &CanisterSummary) -> bool {
    if let Some(status) = summary.status.as_ref() {
        let cycles: u64 = status.cycles.0.clone().try_into().unwrap();
        return cycles < TOP_UP_THRESHOLD;
    } else {
        false
    }
}
