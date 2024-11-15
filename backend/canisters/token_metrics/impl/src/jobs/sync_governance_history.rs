use crate::{
    memory::VM,
    state::{mutate_state, read_state},
};
use canister_time::run_now_then_interval;
use ic_stable_structures::{DefaultMemoryImpl, Vec as SVec};
use std::time::Duration;
use super_stats_v3_api::{
    account_tree::HistoryData,
    stats::queries::{
        get_account_history::GetAccountHistoryArgs, get_principal_history::GetPrincipalHistoryArgs,
    },
};
use token_metrics_api::token_data::GovHistoryEntry;
use tracing::{error, info};
use types::Milliseconds;

const SYNC_GOVERNANCE_HISTORY_INTERVAL: Milliseconds = 3_600 * 1_000;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(SYNC_GOVERNANCE_HISTORY_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(sync_governance_history())
}

pub async fn sync_governance_history() {
    let super_stats_canister_id = read_state(|state| state.data.super_stats_canister);
    let sns_governance_canister_id = read_state(|state| state.data.sns_governance_canister);
    let treasury_account = read_state(|state| state.data.treasury_account.clone());

    let principal_history_args = GetPrincipalHistoryArgs {
        account: sns_governance_canister_id.to_string(),
        days: 2000,
    };

    let treasury_history_args = GetAccountHistoryArgs {
        account: treasury_account.to_string(),
        days: 2000,
    };

    match super_stats_v3_c2c_client::get_principal_history(
        super_stats_canister_id,
        &principal_history_args,
    )
    .await
    {
        Ok(principal_history) => {
            match super_stats_v3_c2c_client::get_account_history(
                super_stats_canister_id,
                &treasury_history_args,
            )
            .await
            {
                Ok(treasury_history) => {
                    mutate_state(|state| {
                        balance_difference(
                            principal_history,
                            treasury_history,
                            &mut state.data.gov_stake_history,
                        );
                    });
                    // We want to sync voting stats now because we rely on stake history
                    sync_voting_stats_job();
                }
                Err(err) => {
                    let message = format!("{err:?}");
                    error!(?message, "Error while getting the treasury history");
                }
            }
        }
        Err(err) => {
            let message = format!("{err:?}");
            error!(
                ?message,
                "Error while getting the governance principal history"
            );
        }
    }
}

fn balance_difference(
    vec1: Vec<(u64, HistoryData)>,
    vec2: Vec<(u64, HistoryData)>,
    target_vec: &mut SVec<GovHistoryEntry, VM>,
) {
    let mut result: Vec<(u64, HistoryData)> = Vec::new();
    for (index, item) in vec1.iter().enumerate() {
        let data1 = item.clone();
        let key1 = data1.0;
        let history1 = data1.1;

        let data2 = vec2[index].clone();
        let history2 = data2.1;
        result.push((
            key1,
            HistoryData {
                balance: history1.balance - history2.balance,
            },
        ));
    }

    for (index, value) in result.iter().enumerate() {
        let value = &GovHistoryEntry(value.clone().0, value.clone().1);
        if target_vec.len() > (index as u64) {
            target_vec.set(index as u64, value);
        } else {
            match target_vec.push(value) {
                Ok(_) => {}
                Err(e) => info!(
                    "balance_difference -> error when inserting to target_vec: {}",
                    e
                ),
            }
        }
    }
}

pub fn sync_voting_stats_job() {
    // How can we get the staked amount history of origyn?
    // We will just use a hardcoded value for now
    let origyn_voting_power = 50_003_931_736_000_000u64;

    mutate_state(|state| {
        // This might be wrong, I think the total voting power also depends on age and other stuff
        let voting_power_ratio: Vec<(u64, u64)> = state
            .data
            .gov_stake_history
            .iter()
            .map(|entry| {
                let ratio =
                    (((origyn_voting_power as f64) / (entry.1.balance as f64)) * 10000.0) as u64;
                (entry.0, ratio)
            })
            .collect();

        state.data.voting_power_ratio_history = voting_power_ratio;
    })
}
