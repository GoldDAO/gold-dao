use crate::state::{mutate_state, read_state};
use crate::updates::manage_nns_neuron::manage_nns_neuron_impl;
use bity_ic_canister_time::{run_now_then_interval, DAY_IN_MS, MINUTE_IN_MS};
use candid::Nat;
use icp_ledger_canister::account_balance::Args as AccountBalanceArgs;

use bity_ic_ledger_utils::icrc_account_to_legacy_account_id;
use ic_ledger_types::AccountIdentifier;
use icp_ledger_canister_c2c_client::account_balance;
use icp_neuron_common::neurons::Neurons;
use icp_neuron_common::outstanding_payments::{PaymentStatus, PaymentsList};
use nns_governance_canister::types::{
    manage_neuron::{disburse::Amount, Command, Disburse, Spawn},
    Neuron,
};
use nns_governance_canister::types::{AccountIdentifier as NNSAccountIdentifier, ListNeurons};
use std::time::Duration;
use tracing::{error, info, warn};
use types::Milliseconds;
use utils::{consts::E8S_PER_ICP, env::Environment};

// Refresh daily to distribute potential rewards but add 1 minute offset to leave enough time in case a neuron is spawned
const REFRESH_NEURONS_INTERVAL: Milliseconds = DAY_IN_MS + MINUTE_IN_MS;

const SPAWN_LIMIT_ICP: u64 = 1000;

// The NNS rejects a spawn when the maturity, after the worst case maturity modulation of -5%,
// would mint less than the minimum neuron stake of 1 ICP. 1.1 ICP keeps a small margin above that.
const MIN_SPAWNABLE_MATURITY_E8S: u64 = 110_000_000;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(REFRESH_NEURONS_INTERVAL), run);
}

pub fn run() {
    ic_cdk::futures::spawn(run_async());
}

async fn run_async() {
    let nns_governance_canister_id = read_state(|state| state.data.nns_governance_canister_id);

    match nns_governance_canister_c2c_client::list_neurons(
        nns_governance_canister_id,
        &(ListNeurons {
            neuron_ids: Vec::new(),
            include_neurons_readable_by_caller: true,
        }),
    )
    .await
    {
        Ok(response) => {
            let now = read_state(|state| state.env.now());

            let neurons_to_spawn: Vec<_> = response
                .full_neurons
                .iter()
                .filter(|n| should_spawn(n))
                .filter_map(|n| n.id.as_ref().map(|id| id.id))
                .collect();

            let neurons_to_disburse: Vec<Neuron> = response
                .full_neurons
                .iter()
                .filter(|n| n.is_dissolved(now) && n.cached_neuron_stake_e8s > 0)
                .cloned()
                .collect();

            mutate_state(|state| {
                let mut active_neurons = Vec::new();
                let mut spawning_neurons = Vec::new();
                let mut disbursed_neurons = Vec::new();
                for neuron in response.full_neurons.into_iter() {
                    if neuron.maturity_e8s_equivalent == 0 && neuron.cached_neuron_stake_e8s == 0 {
                        if let Some(neuron_id) = neuron.id {
                            disbursed_neurons.push(neuron_id.id);
                        }
                    } else if neuron.spawn_at_timestamp_seconds.is_some() {
                        spawning_neurons.push(neuron);
                    } else {
                        active_neurons.push(neuron);
                    }
                }

                state.data.neurons = Neurons {
                    timestamp: now,
                    active_neurons,
                    spawning_neurons,
                    disbursed_neurons,
                };
            });

            let mut neurons_updated = false;

            if !neurons_to_spawn.is_empty() {
                spawn_neurons(neurons_to_spawn).await;
                neurons_updated = true;
            }

            if !neurons_to_disburse.is_empty() {
                disburse_neurons(neurons_to_disburse).await;
                neurons_updated = true;
            }

            mutate_state(|s| s.data.outstanding_payments.cleanup());

            if neurons_updated {
                // Refresh the neurons again given that they've been updated (spawned neurons and disbursed neurons)
                // Add a delay of 5 minutes to give enough time for transactions to pass
                ic_cdk_timers::set_timer(Duration::from_millis(5 * MINUTE_IN_MS), || {
                    ic_cdk::futures::spawn(run_async())
                });
            }
        }
        Err(err) => {
            error!("Error fetching neuron list: {err:?}")
        }
    }
}

/// The maturity of a neuron is spawned once it exceeds the spawn limit. A neuron without stake
/// (e.g. one that was merged into another neuron and received its trailing voting rewards
/// afterwards) cannot earn anything more, so its maturity is spawned as soon as the NNS accepts
/// the amount. Neurons that are already spawning are skipped.
fn should_spawn(neuron: &Neuron) -> bool {
    if neuron.spawn_at_timestamp_seconds.is_some() {
        return false;
    }
    let over_spawn_limit = neuron.maturity_e8s_equivalent > SPAWN_LIMIT_ICP * E8S_PER_ICP;
    let stakeless_leftover = neuron.cached_neuron_stake_e8s == 0
        && neuron.maturity_e8s_equivalent >= MIN_SPAWNABLE_MATURITY_E8S;
    over_spawn_limit || stakeless_leftover
}

async fn spawn_neurons(neuron_ids: Vec<u64>) {
    for neuron_id in neuron_ids {
        info!(neuron_id, "Spawning neuron from maturity");
        match manage_nns_neuron_impl(neuron_id, Command::Spawn(Spawn::default())).await {
            Ok(_) => info!("Successfully spawned neuron {neuron_id}."),
            Err(err) => warn!("Error spawning neuron {neuron_id}: {err}"),
        }
    }
}

async fn disburse_neurons(mut neurons: Vec<Neuron>) {
    let rewards_recipients = read_state(|state| state.data.rewards_recipients.clone());

    let cycle_management_accounts = read_state(|state| state.data.cycle_management_account.clone());
    if cycle_management_accounts.is_empty() {
        warn!("No cycle management account defined, skipping disbursement to cycle management account and disbursing normally.");
    } else {
        for account in cycle_management_accounts {
            if neurons.is_empty() {
                break;
            }
            match fetch_cycle_management_icp_balance(account.clone()).await {
                Ok(amount) => {
                    if amount < Nat::from(100_000_000_000u64) {
                        let result =
                            disburse_to_cycle_management_account(neurons.pop(), account.clone())
                                .await;
                        info!("{result:?}");
                    }
                }
                Err(e) => info!(e),
            }
        }
    }
    if rewards_recipients.is_empty() {
        warn!("Skipping disbursement of neurons because no reward recipients are defined.");
        return;
    }

    for neuron in neurons {
        let neuron_id: u64;
        if let Some(id) = neuron.id {
            neuron_id = id.id;
            info!(id.id, "Disbursing neuron.");
        } else {
            warn!("Empty neuron id. Cannot disburse and continuing to next one.");
            continue;
        }

        let total_amount = neuron.cached_neuron_stake_e8s;

        let mut payments_list: PaymentsList;
        // If there are previous pending payments, take those, otherwise prepare the new list
        // This may happen if the payment cycle is interrupted because of an upgrade and then the disburse_neurons()
        // call would rerun on the same neuron. If a payment has already been made, some addresses could receive
        // double payments and others receive less than they should.
        if let Some(previous_list) = read_state(|s| {
            s.data
                .outstanding_payments
                .get_outstanding_payments(neuron_id)
        }) {
            payments_list = previous_list;
        } else {
            payments_list = match rewards_recipients.split_amount_to_each_recipient(total_amount) {
                Ok(list) => PaymentsList::new(list),
                Err(err) => {
                    error!(
                        "Error splitting amount to each recipient for neuron {neuron_id}. Error: {err}"
                    );
                    continue;
                }
            };
            // write to state to make sure its stored
            mutate_state(|s| {
                if let Err(previous_list) = s
                    .data
                    .outstanding_payments
                    .insert(neuron_id, payments_list.clone())
                {
                    // This means that there was already an entry in the list for this neuron.
                    // This should not be possible as we previously checked if outstanding payments are left.
                    // However, to handle this gracefully, we continue with the previous list and log a warning.
                    warn!(
                        "Previous payment found for {neuron_id} although it was previously checked. Continuing but this should not happen."
                    );
                    payments_list = previous_list;
                }
            });
        }

        // would occur if no rewards_recipients are defined
        if payments_list.has_none() {
            continue;
        }

        for (&account, payment) in payments_list.list.iter() {
            if payment.is_complete() {
                continue;
            }
            let icp_ledger_account = nns_governance_canister::types::AccountIdentifier {
                hash: icrc_account_to_legacy_account_id(account).as_ref().to_vec(),
            };
            match manage_nns_neuron_impl(
                neuron_id,
                Command::Disburse(Disburse {
                    to_account: Some(icp_ledger_account),
                    amount: Some(Amount {
                        e8s: payment.get_amount(),
                    }),
                }),
            )
            .await
            {
                Ok(_) => {
                    mutate_state(|s| {
                        s.data.outstanding_payments.update_status_of_entry_in_list(
                            neuron_id,
                            account,
                            PaymentStatus::Complete,
                        )
                    });
                }
                Err(err) => {
                    error!(
                        "Error processing disburse payment for neuron {neuron_id}. Error: {err}"
                    );
                }
            }
        }

        mutate_state(|s| {
            if payments_list.all_complete() {
                s.data.outstanding_payments.remove_from_list(neuron_id);
            }
        });
    }
}

async fn fetch_cycle_management_icp_balance(
    cycle_management_account: AccountIdentifier,
) -> Result<Nat, String> {
    let icp_ledger = read_state(|s| s.data.icp_ledger_canister_id);

    match account_balance(
        icp_ledger,
        AccountBalanceArgs {
            account: cycle_management_account,
        }
    )
    .await {
        Ok(amount) => Ok(Nat::from(amount.e8s)),
        Err(e) => Err(format!("ERROR :: fetch_cycle_management_icp_balance :: error fetching icp balance of account :: {e:?}"))
    }
}

async fn disburse_to_cycle_management_account(
    neuron: Option<Neuron>,
    cycle_management_account: AccountIdentifier,
) -> Result<(), String> {
    let neuron = neuron.ok_or_else(|| {
        "WARNING :: disburse_to_cycle_management_account :: neuron is a none value"
    })?;

    let cycle_management_account: Vec<u8> = cycle_management_account.as_bytes().try_into().map_err(|e| format!("ERROR :: disburse_to_cycle_management_account :: failed to convert account into hex with error - {e:?}"))?;

    let neuron_id = neuron
        .id
        .ok_or_else(|| {
            "ERROR :: disburse_to_cycle_management_account :: neuron doesnt have an ID - {neuron:?}"
        })?
        .id;

    match manage_nns_neuron_impl(
        neuron_id,
        Command::Disburse(Disburse {
            to_account: Some(NNSAccountIdentifier {
                hash: cycle_management_account,
            }),
            amount: Some(Amount {
                e8s: neuron.cached_neuron_stake_e8s,
            }),
        }),
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(format!(
            "ERROR :: disburse_to_cycle_management_account :: error disbursing neuron :: {e:?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{should_spawn, MIN_SPAWNABLE_MATURITY_E8S, SPAWN_LIMIT_ICP};
    use ic_ledger_types::AccountIdentifier;
    use nns_governance_canister::types::{Neuron, NeuronId};
    use std::collections::HashMap;
    use utils::consts::E8S_PER_ICP;

    use crate::state::{init_state, mutate_state, read_state, RuntimeState};

    fn init_runtime_state() {
        init_state(RuntimeState::default());
    }

    #[test]
    fn test_multiple_cycles_management_accounts() {
        init_runtime_state();

        let account1 = AccountIdentifier::from_hex(
            "a51ceabd4d86c16c94936db0422d9b814b4f20e58fa013aeace0053af2305e8c",
        )
        .unwrap();
        let account2 = AccountIdentifier::from_hex(
            "8fab530a08fc70fd40140c5b4896fca7a8b8dab1e7ff2f3d60aa21f248a256e9",
        )
        .unwrap();

        mutate_state(|s| {
            s.data.cycle_management_account.push(account1.clone());
            s.data.cycle_management_account.push(account2.clone());
        });

        let accounts = read_state(|s| s.data.cycle_management_account.clone());
        assert_eq!(accounts.len(), 2);

        let cycle_management_account: Result<Vec<u8>, _> = accounts.first().unwrap().as_bytes().try_into().map_err(|e| format!("ERROR :: disburse_to_cycle_management_account :: failed to convert account into hex with error - {e:?}"));
        assert!(cycle_management_account.is_ok());

        assert_eq!(
            cycle_management_account.clone().unwrap(),
            account1.as_bytes()
        );
    }

    fn neuron(stake_e8s: u64, maturity_e8s: u64, spawn_at: Option<u64>) -> Neuron {
        Neuron {
            id: Some(NeuronId {
                id: 2_115_552_344_633_178_977,
            }),
            account: vec![],
            controller: None,
            hot_keys: vec![],
            cached_neuron_stake_e8s: stake_e8s,
            neuron_fees_e8s: 0,
            created_timestamp_seconds: 0,
            aging_since_timestamp_seconds: 0,
            spawn_at_timestamp_seconds: spawn_at,
            followees: HashMap::default(),
            recent_ballots: vec![],
            kyc_verified: false,
            maturity_e8s_equivalent: maturity_e8s,
            staked_maturity_e8s_equivalent: None,
            auto_stake_maturity: None,
            not_for_profit: false,
            joined_community_fund_timestamp_seconds: None,
            known_neuron_data: None,
            dissolve_state: None,
            voting_power_refreshed_timestamp_seconds: None,
            potential_voting_power: None,
            neuron_type: None,
            deciding_voting_power: None,
            visibility: None,
        }
    }

    #[test]
    fn spawns_leftover_maturity_of_neuron_without_stake() {
        assert!(should_spawn(&neuron(0, 8_888_590_316, None)));
        assert!(should_spawn(&neuron(0, MIN_SPAWNABLE_MATURITY_E8S, None)));
    }

    #[test]
    fn does_not_spawn_maturity_below_nns_minimum() {
        assert!(!should_spawn(&neuron(
            0,
            MIN_SPAWNABLE_MATURITY_E8S - 1,
            None
        )));
        assert!(!should_spawn(&neuron(0, 0, None)));
    }

    #[test]
    fn does_not_spawn_neuron_that_is_already_spawning() {
        assert!(!should_spawn(&neuron(
            0,
            8_888_590_316,
            Some(1_788_599_490)
        )));
        assert!(!should_spawn(&neuron(
            0,
            2000 * E8S_PER_ICP,
            Some(1_788_599_490)
        )));
    }

    #[test]
    fn staked_neuron_only_spawns_above_spawn_limit() {
        let stake = 55_588_837_310_215;
        assert!(!should_spawn(&neuron(stake, 17_852_254_223, None)));
        assert!(!should_spawn(&neuron(
            stake,
            SPAWN_LIMIT_ICP * E8S_PER_ICP,
            None
        )));
        assert!(should_spawn(&neuron(
            stake,
            SPAWN_LIMIT_ICP * E8S_PER_ICP + 1,
            None
        )));
    }
}
