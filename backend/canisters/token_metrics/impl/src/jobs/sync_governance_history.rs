use canister_time::run_now_then_interval;
use super_stats_v3_api::{
    account_tree::HistoryData,
    stats::queries::{
        get_account_history::GetAccountHistoryArgs,
        get_principal_history::GetPrincipalHistoryArgs,
    },
};
use std::time::Duration;
use tracing::error;
use types::Milliseconds;
use crate::state::{ mutate_state, read_state };

const SYNC_GOVERNANCE_HISTORY_INTERVAL: Milliseconds = 3_600 * 1_000;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(SYNC_GOVERNANCE_HISTORY_INTERVAL), run)
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

    match
        super_stats_v3_c2c_client::get_principal_history(
            super_stats_canister_id,
            &principal_history_args
        ).await
    {
        Ok(principal_history) => {
            match
                super_stats_v3_c2c_client::get_account_history(
                    super_stats_canister_id,
                    &treasury_history_args
                ).await
            {
                Ok(treasury_history) => {
                    mutate_state(|state| {
                        state.data.gov_stake_history = balance_difference(
                            principal_history,
                            treasury_history
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
            error!(?message, "Error while getting the governance principal history");
        }
    }
}

fn balance_difference(
    vec1: Vec<(u64, HistoryData)>,
    vec2: Vec<(u64, HistoryData)>
) -> Vec<(u64, HistoryData)> {
    let mut result: Vec<(u64, HistoryData)> = Vec::new();
    for (index, item) in vec1.iter().enumerate() {
        let data1 = item.clone();
        let key1 = data1.0;
        let history1 = data1.1;

        let data2 = vec2[index].clone();
        let history2 = data2.1;
        result.push((key1, HistoryData { balance: history1.balance - history2.balance }));
    }

    result
}

pub fn sync_voting_stats_job() {
    // How can we get the staked amount history of origyn?
    // We will just use a hardcoded value for now
    let origyn_voting_power = 50_003_931_736_000_000u64;

    // This might be wrong, I think the total voting power also depends on age and other stuff
    let stake_history = read_state(|state| state.data.gov_stake_history.clone());

    let voting_power_ratio: Vec<(u64, u64)> = stake_history
        .iter()
        .map(|(timestamp, history_data)| {
            let ratio = (((origyn_voting_power as f64) / (history_data.balance as f64)) *
                10000.0) as u64;
            (*timestamp, ratio)
        })
        .collect();

    mutate_state(|state| {
        state.data.voting_power_ratio_history = voting_power_ratio;
    })
}
