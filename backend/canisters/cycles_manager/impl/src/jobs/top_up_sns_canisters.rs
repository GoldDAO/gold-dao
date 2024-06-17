use crate::state::{mutate_state, read_state};
use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use sns_root_canister::get_sns_canisters_summary::CanisterSummary;
use std::time::Duration;
use tracing::error;
use types::{CanisterId, Empty};
use utils::canister::deposit_cycles;
use utils::env::Environment;

const INTERVAL: Duration = Duration::from_secs(60 * 60); // 1 hour

pub fn start_job() {
    run_now_then_interval(INTERVAL, run);
}

pub fn run() {
    let canister_id = read_state(|state| state.data.top_up_config.sns_root_canister);
    ic_cdk::spawn(top_up_canisters(canister_id));
}

pub async fn sync_canister_stats(canister_id: CanisterId) -> Result<Vec<CanisterSummary>, String> {
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

            // Add SNS canisters to the whitelist
            mutate_state(|state| {
                let now = state.env.now();
                for canister_id in canisters.iter().flat_map(|c| c.canister_id) {
                    state.data.canisters.add(canister_id, now);
                }
            });
            Ok(canisters)
        }
        Err(e) => {
            error!("Failed to get SNS canisters summary: {:?}", e);
            Err("Failed to get SNS canisters summary".to_string())
        }
    }
}

#[trace]
async fn top_up_canisters(canister_id: CanisterId) {
    match sync_canister_stats(canister_id).await {
        Ok(canisters) => {
            let top_up_threshold = read_state(|state| state.data.top_up_config.min_cycles_balance);

            let to_top_up: Vec<_> = canisters
                .into_iter()
                .filter(|s| requires_top_up(s, top_up_threshold))
                .filter_map(|s| s.canister_id)
                .collect();

            if !to_top_up.is_empty() {
                let cycles_balance = read_state(|state| state.env.cycles_balance());
                let top_up_amount = read_state(|state| state.data.top_up_config.max_top_up_amount);
                let canisters_amount: u64 = to_top_up.len().try_into().unwrap();
                let summary_top_up_amount = top_up_amount * canisters_amount;

                if summary_top_up_amount < cycles_balance {
                    let top_up_futures = to_top_up
                        .iter()
                        .map(|&canister_id| deposit_cycles(canister_id, top_up_amount));

                    let results = futures::future::join_all(top_up_futures).await;

                    mutate_state(|state| {
                        let now = state.env.now();
                        for (index, result) in results.into_iter().enumerate() {
                            let canister_id = to_top_up[index];
                            match result {
                                Ok(_) => {
                                    if let Some(canister) =
                                        state.data.canisters.get_mut(&canister_id)
                                    {
                                        canister.record_top_up(top_up_amount, now);
                                    } else {
                                        state.data.canisters.add(canister_id, now);
                                        if let Some(canister) =
                                            state.data.canisters.get_mut(&canister_id)
                                        {
                                            canister.record_top_up(top_up_amount, now);
                                        }
                                    }
                                }
                                Err(e) => {
                                    // TODO: add journaling here
                                    error!("Failed to top up canister {}: {:?}", canister_id, e);
                                }
                            }
                        }
                    });
                } else {
                    error!("Failed to top up canisters: the cycles manager canister balance is too low");
                }
            }
        }
        Err(e) => {
            error!("Failed to top up canisters: {}", e);
        }
    }
}

fn requires_top_up(summary: &CanisterSummary, top_up_threshold: u64) -> bool {
    if let Some(status) = summary.status.as_ref() {
        status.cycles.0 < top_up_threshold.into()
    } else {
        false
    }
}
