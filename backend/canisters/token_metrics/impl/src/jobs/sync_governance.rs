use candid::{ Nat, Principal };
use canister_time::{ now_millis, run_now_then_interval, timestamp_seconds, DAY_IN_MS };
use futures::future::join_all;
use ic_cdk::api::call::RejectionCode;
use icrc_ledger_types::icrc1::account::{ Account, Subaccount };
use sns_governance_canister::types::{ neuron::DissolveState, Neuron, NeuronId };
use super_stats_v3_api::stats::constants::SECONDS_IN_ONE_YEAR;
use token_metrics_api::token_data::{ GovernanceStats, LockedNeuronsAmount };
use std::collections::BTreeMap as NormalBTreeMap;
use std::time::Duration;
use tracing::{ debug, error, info };
use types::Milliseconds;

use crate::{
    jobs::{ sync_supply_data, update_balance_list },
    state::{ mutate_state, read_state, PrincipalDotAccountFormat },
};

const SYNC_NEURONS_INTERVAL: Milliseconds = DAY_IN_MS;

pub fn start_job() {
    debug!("Starting the governance sync job..");
    run_now_then_interval(Duration::from_millis(SYNC_NEURONS_INTERVAL), run)
}

pub fn run() {
    ic_cdk::spawn(sync_neurons_data())
}

pub async fn sync_neurons_data() {
    info!("sync_neurons_data..");
    let canister_id = read_state(|state| state.data.sns_governance_canister);

    mutate_state(|state| {
        state.data.sync_info.last_synced_start = now_millis();
    });

    let mut number_of_scanned_neurons = 0;
    let mut continue_scanning: bool = true;

    let mut args = sns_governance_canister::list_neurons::Args {
        limit: 100,
        start_page_at: None,
        of_principal: None,
    };

    // We want new empty structures when re-computing the data, otherwise it will
    // sum up with data from previous job
    // Q: BTreeMap is a stable structure? Do we want that for a temp variable like this?
    let mut temp_principal_with_neurons: NormalBTreeMap<
        Principal,
        Vec<NeuronId>
    > = NormalBTreeMap::new();
    let mut temp_principal_with_stats: NormalBTreeMap<
        Principal,
        GovernanceStats
    > = NormalBTreeMap::new();
    let mut temp_all_gov_stats: GovernanceStats = GovernanceStats::default();

    let mut temp_locked_neurons_amount: LockedNeuronsAmount = LockedNeuronsAmount::default();

    while continue_scanning {
        continue_scanning = false;

        match sns_governance_canister_c2c_client::list_neurons(canister_id, &args).await {
            Ok(response) => {
                // info!("{:?}", response);
                // Mutate the state to update the principal with governance data
                info!("Iterating neurons");
                response.neurons.iter().for_each(|neuron| {
                    update_principal_neuron_mapping(
                        &mut temp_principal_with_neurons,
                        &mut temp_principal_with_stats,
                        &mut temp_all_gov_stats,
                        neuron
                    );
                    update_locked_neurons_amount(&mut temp_locked_neurons_amount, neuron);
                });

                // Check if we hit the end of the list
                let number_of_received_neurons = response.neurons.len();
                if number_of_received_neurons == 100 {
                    args.start_page_at = response.neurons.last().map_or_else(
                        || {
                            error!("we should not be here, last neurons from response is missing?");
                            None
                        },
                        |n| {
                            continue_scanning = true;
                            n.id.clone()
                        }
                    );
                }
                number_of_scanned_neurons += number_of_received_neurons;
            }
            Err(err) => {
                let error_message = format!("{err:?}");
                error!(?error_message, "Error fetching neuron data");
            }
        }
    }
    info!("Successfully scanned {number_of_scanned_neurons} neurons.");

    let total_rewards_in_sns_canister = get_total_from_sns_rewards_canister().await;
    info!("Total rewards in the sns canister: {}", total_rewards_in_sns_canister);

    mutate_state(|state| {
        state.data.sync_info.last_synced_end = now_millis();
        state.data.sync_info.last_synced_number_of_neurons = number_of_scanned_neurons;

        let principal_with_neurons = &mut state.data.principal_neurons;
        let principal_with_stats = &mut state.data.principal_gov_stats;
        let all_gov_stats = &mut state.data.all_gov_stats;
        let locked_neurons_amount = &mut state.data.locked_neurons_amount;

        // Update the state with the newly computed data
        principal_with_neurons.clear();
        principal_with_stats.clear();
        for (key, value) in temp_principal_with_neurons {
            principal_with_neurons.insert(key, value);
        }
        for (key, value) in temp_principal_with_stats {
            principal_with_stats.insert(key, value);
        }
        *all_gov_stats = temp_all_gov_stats;
        *locked_neurons_amount = temp_locked_neurons_amount;

        all_gov_stats.total_rewards += total_rewards_in_sns_canister;
    });

    // After we have computed governance stats, update the total supply and circulating supply
    sync_supply_data::run();
}
fn check_locked_neurons_period(
    locked_neurons_amount: &mut LockedNeuronsAmount,
    value: u64,
    dissolve_delay: Option<u64>,
    end_timestamp: Option<u64>
) {
    let duration = if let Some(dissolve_delay) = dissolve_delay {
        dissolve_delay
    } else if let Some(ets) = end_timestamp {
        let current_timestamp_in_seconds = timestamp_seconds();
        ets - current_timestamp_in_seconds
    } else {
        0
    };
    if duration >= 5 * SECONDS_IN_ONE_YEAR {
        locked_neurons_amount.five_years += value;
    } else if duration >= 4 * SECONDS_IN_ONE_YEAR {
        locked_neurons_amount.four_years += value;
    } else if duration >= 3 * SECONDS_IN_ONE_YEAR {
        locked_neurons_amount.three_years += value;
    } else if duration >= 2 * SECONDS_IN_ONE_YEAR {
        locked_neurons_amount.two_years += value;
    } else if duration >= 1 * SECONDS_IN_ONE_YEAR {
        locked_neurons_amount.one_year += value;
    }
}
fn update_locked_neurons_amount(locked_neurons_amount: &mut LockedNeuronsAmount, neuron: &Neuron) {
    match neuron.dissolve_state.clone() {
        Some(DissolveState::DissolveDelaySeconds(dissolve_delay_in_seconds)) => {
            let staked_value =
                neuron.cached_neuron_stake_e8s + neuron.staked_maturity_e8s_equivalent.unwrap_or(0);
            check_locked_neurons_period(
                locked_neurons_amount,
                staked_value,
                Some(dissolve_delay_in_seconds),
                None
            )
        }
        Some(DissolveState::WhenDissolvedTimestampSeconds(end_timestamp)) => {
            let staked_value =
                neuron.cached_neuron_stake_e8s + neuron.staked_maturity_e8s_equivalent.unwrap_or(0);
            check_locked_neurons_period(
                locked_neurons_amount,
                staked_value,
                None,
                Some(end_timestamp)
            )
        }
        None => {}
    }
}
fn update_principal_neuron_mapping(
    principal_with_neurons: &mut NormalBTreeMap<Principal, Vec<NeuronId>>,
    principal_with_stats: &mut NormalBTreeMap<Principal, GovernanceStats>,
    all_gov_stats: &mut GovernanceStats,
    neuron: &Neuron
) {
    if let Some(permissioned_principal) = neuron.permissions.first() {
        if let Some(pid) = permissioned_principal.principal {
            // Update the array of neurons in the principal_neurons map
            principal_with_neurons
                .entry(pid)
                .and_modify(|neurons| {
                    if let Some(id) = &neuron.id {
                        if !neurons.contains(id) {
                            neurons.push(id.clone());
                        }
                    }
                })
                .or_insert_with(|| {
                    if let Some(id) = &neuron.id { vec![id.clone()] } else { vec![] }
                });

            let mut neuron_locked = 0;
            let mut neuron_unlocked = 0;

            let neuron_rewards = neuron.staked_maturity_e8s_equivalent.unwrap_or(0);

            match neuron.dissolve_state.clone() {
                Some(DissolveState::DissolveDelaySeconds(dissolve_delay_in_seconds)) => {
                    if dissolve_delay_in_seconds > 0 {
                        neuron_locked = neuron.cached_neuron_stake_e8s;
                    } else {
                        neuron_unlocked = neuron.cached_neuron_stake_e8s;
                    }
                }
                Some(DissolveState::WhenDissolvedTimestampSeconds(end_timestamp)) => {
                    let current_timestamp_in_seconds = timestamp_seconds();
                    if end_timestamp >= current_timestamp_in_seconds {
                        neuron_locked = neuron.cached_neuron_stake_e8s;
                    } else {
                        neuron_unlocked = neuron.cached_neuron_stake_e8s;
                    }
                }
                None => {}
            }

            // Update the governance stats of the Principal
            principal_with_stats
                .entry(pid)
                .and_modify(|stats| {
                    // Total staked is how much the principal staked at the begginging + how much of maturity they restaked
                    stats.total_staked += neuron_locked + neuron_unlocked + neuron_rewards;
                    // Total locked is the amount of tokens they have staked
                    stats.total_locked += neuron_locked;
                    // Total unlocked is `maturity_e8s_equivalent` which can be claimed
                    stats.total_unlocked += neuron_unlocked;
                    // Total rewards is what they have as maturity and what they have as staked_maturity
                    stats.total_rewards += neuron_rewards;
                })
                .or_insert_with(|| GovernanceStats {
                    total_staked: (neuron_locked + neuron_unlocked + neuron_rewards)
                        .try_into()
                        .unwrap(),
                    total_locked: neuron_locked.try_into().unwrap(),
                    total_unlocked: neuron_unlocked.try_into().unwrap(),
                    total_rewards: neuron_rewards.try_into().unwrap(),
                });
            all_gov_stats.total_staked += neuron_locked + neuron_unlocked + neuron_rewards;
            all_gov_stats.total_locked += neuron_locked;
            all_gov_stats.total_unlocked += neuron_unlocked;
            all_gov_stats.total_rewards += neuron_rewards;
        }
    }
}
async fn get_total_from_sns_rewards_canister() -> Nat {
    let sns_rewards_canister_id = read_state(|state| state.data.sns_rewards_canister);

    // Rewards pool are in the default subaccount of sns rewards
    let rewards_pool_subaccount = Account {
        owner: sns_rewards_canister_id,
        subaccount: None,
    };

    // Reserve pool are in the subaccount [1, 31x0] of sns rewards
    let reserve_pool_subaccount = Account {
        owner: sns_rewards_canister_id,
        subaccount: Some([
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ]),
    };

    let getter_futures: Vec<_> = vec![
        get_super_stats_balance_of(sns_rewards_canister_id.to_string(), false),
        get_super_stats_balance_of(rewards_pool_subaccount.to_principal_dot_account(), true),
        get_super_stats_balance_of(reserve_pool_subaccount.to_principal_dot_account(), true)
    ];

    let results = join_all(getter_futures).await;
    return results[0].clone() - results[1].clone() - results[2].clone();
}
async fn get_super_stats_balance_of(account: String, is_subaccount: bool) -> Nat {
    let super_stats_canister_id = read_state(|state| state.data.super_stats_canister);

    fn log_error(acc: String, err: (RejectionCode, String)) {
        let error_message = format!("{err:?}");
        info!(?error_message, "There has been an erorr while fetching the super_stats balance of {acc:?}");
    }

    let result = if is_subaccount {
        match
            super_stats_v3_c2c_client::get_account_overview(super_stats_canister_id, &account).await
        {
            Ok(response) => response,
            Err(err) => {
                log_error(account.clone(), err);
                None
            }
        }
    } else {
        match
            super_stats_v3_c2c_client::get_principal_overview(
                super_stats_canister_id,
                &account
            ).await
        {
            Ok(response) => response,
            Err(err) => {
                log_error(account.clone(), err);
                None
            }
        }
    };
    return match result {
        Some(res) => { Nat::from(res.balance) }
        None => Nat::from(0u64),
    };
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use canister_time::timestamp_seconds;
    use sns_governance_canister::types::{ neuron::DissolveState, Neuron, NeuronId };
    use super_stats_v3_api::stats::constants::SECONDS_IN_ONE_YEAR;
    use token_metrics_api::token_data::LockedNeuronsAmount;
    use types::NeuronInfo;

    use crate::state::{ init_state, mutate_state, read_state, RuntimeState };

    use super::update_locked_neurons_amount;

    #[test]
    fn test_update_locked_neurons_amount() {
        // Create test neurons
        let neuron_id_1 = NeuronId::new(
            "1a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_4 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_5 = NeuronId::new(
            "5a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let mut neuron_1 = Neuron::default();
        neuron_1.id = Some(neuron_id_1.clone());
        neuron_1.cached_neuron_stake_e8s = 10_000;
        neuron_1.staked_maturity_e8s_equivalent = Some(20_000);
        neuron_1.dissolve_state = Some(
            DissolveState::DissolveDelaySeconds(1 * SECONDS_IN_ONE_YEAR)
        );
        // Neuron 1: 30_000
        // Staked for 1 year

        let mut neuron_2 = Neuron::default();
        neuron_2.id = Some(neuron_id_2.clone());
        neuron_2.cached_neuron_stake_e8s = 30_000;
        neuron_2.staked_maturity_e8s_equivalent = Some(30_000);
        neuron_2.dissolve_state = Some(
            DissolveState::DissolveDelaySeconds(2 * SECONDS_IN_ONE_YEAR)
        );
        // Neuron 2: 60_000
        // Staked for 2 years

        let mut neuron_3 = Neuron::default();
        neuron_3.id = Some(neuron_id_3.clone());
        neuron_3.cached_neuron_stake_e8s = 40_000;
        neuron_3.staked_maturity_e8s_equivalent = Some(5_000);
        let now_in_seconds = timestamp_seconds();
        neuron_3.dissolve_state = Some(
            DissolveState::WhenDissolvedTimestampSeconds(now_in_seconds + 1 * SECONDS_IN_ONE_YEAR)
        );
        // Neuron 3: 45_000
        // Staked for 1 year

        let mut neuron_4 = Neuron::default();
        neuron_4.id = Some(neuron_id_4.clone());
        neuron_4.cached_neuron_stake_e8s = 20_000;
        neuron_4.staked_maturity_e8s_equivalent = Some(25_000);
        neuron_4.dissolve_state = Some(
            DissolveState::DissolveDelaySeconds(4 * SECONDS_IN_ONE_YEAR)
        );
        // Neuron 4: 45_000
        // Staked for 4 years

        let mut neuron_5 = Neuron::default();
        neuron_5.id = Some(neuron_id_5.clone());
        neuron_5.cached_neuron_stake_e8s = 50_000;
        neuron_5.staked_maturity_e8s_equivalent = Some(50_000);
        neuron_5.dissolve_state = Some(
            DissolveState::DissolveDelaySeconds(5 * SECONDS_IN_ONE_YEAR)
        );
        // Neuron 5: 100_000
        // Staked for 5 years

        // Update the stats locked neurons amount
        let mut locked_neurons_amount: LockedNeuronsAmount = LockedNeuronsAmount::default();
        update_locked_neurons_amount(&mut locked_neurons_amount, &neuron_1);
        assert_eq!(locked_neurons_amount.one_year, 30_000);
        update_locked_neurons_amount(&mut locked_neurons_amount, &neuron_2);
        assert_eq!(locked_neurons_amount.two_years, 60_000);
        update_locked_neurons_amount(&mut locked_neurons_amount, &neuron_3);
        assert_eq!(locked_neurons_amount.one_year, 75_000);
        update_locked_neurons_amount(&mut locked_neurons_amount, &neuron_4);
        assert_eq!(locked_neurons_amount.four_years, 45_000);
        update_locked_neurons_amount(&mut locked_neurons_amount, &neuron_5);
        assert_eq!(locked_neurons_amount.five_years, 100_000);
    }
}
