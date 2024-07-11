use crate::state::{mutate_state, read_state, Neurons};
use crate::utils::fetch_neuron_reward_balance;
use crate::utils::transfer_token;
use candid::Nat;
use candid::Principal;
use canister_time::{run_now_then_interval, DAY_IN_MS, MINUTE_IN_MS};
use sns_governance_canister::types::ListNeurons;
use sns_governance_canister::types::Neuron;
use std::time::Duration;
use tracing::debug;
use tracing::{error, info};
use types::Milliseconds;
use utils::consts::SNS_REWARDS_CANISTER_ID;
use utils::env::Environment;

// Refresh daily to distribute potential rewards but add 1 minute offset to leave enough time in case a neuron is spawned
const PROCESS_NEURONS_INTERVAL: Milliseconds = DAY_IN_MS + MINUTE_IN_MS;

const CLAIM_REWARDS_THRESHOLD: u64 = 100_000_000 * 1_000_000; // 1_000_000 tokens

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(PROCESS_NEURONS_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

async fn run_async() {
    let sns_governance_canister_id = read_state(|state| state.data.ogy_sns_governance_canister_id);
    let is_test_mode = read_state(|s| s.env.is_test_mode());
    let canister_id = read_state(|s| s.env.canister_id());

    // TODO: add neurons storing
    let neurons = fetch_neurons(sns_governance_canister_id, canister_id, is_test_mode)
        .await
        .unwrap();

    // Make a call to sns_ledger in order to check the sns_rewards balance destined for payment to a neuron
    let sns_ledger_canister_id = read_state(|state| state.data.ogy_sns_ledger_canister_id);
    let ogy_sns_rewards_canister_id = read_state(|state| state.data.ogy_sns_rewards_canister_id);
    let available_rewards = calculate_available_rewards(
        &neurons,
        ogy_sns_rewards_canister_id,
        sns_ledger_canister_id,
    )
    .await;

    // Q: Once the balance exceeds a certain threshold (e.g. 1 million OGY),
    // the rewards can be claimed and sent to the Gold DAO sns_rewards canister for distribution.
    if CLAIM_REWARDS_THRESHOLD > available_rewards {
        let _ = claim_rewards(&neurons, sns_ledger_canister_id).await;
        let _ = distribute_rewards(sns_governance_canister_id, available_rewards).await;
    }
}

// Fetch all neurons from SNS governance canister
async fn fetch_neurons(
    sns_governance_canister_id: Principal,
    canister_id: Principal,
    is_test_mode: bool,
) -> Result<Vec<Neuron>, String> {
    // NOTE: taken from sync_neurons: the max limit of 100 is given by the list_neurons call implementation. Cannot increase it.
    // TODO: research if this still works the same way
    let limit = 100;

    let mut args = ListNeurons {
        limit,
        start_page_at: None,
        // here we check only the neurons to which the canister has permissions
        of_principal: Some(canister_id),
    };

    let mut number_of_scanned_neurons = 0;
    let mut continue_scanning = true;

    let mut neurons = Vec::new();
    while continue_scanning {
        continue_scanning = false;
        debug!("Fetching neuron data");

        // NOTE: the reason why we need a loop here is that list_neurons can only return 100 neurons
        // at a time . In fact, I'm not sure that we would exceed the limit in any case, but it's
        // better to future proof it in case if it works that way.
        match sns_governance_canister_c2c_client::list_neurons(sns_governance_canister_id, &args)
            .await
        {
            Ok(response) => {
                let number_of_received_neurons = response.neurons.len();
                if (number_of_received_neurons as u32) == limit {
                    args.start_page_at = response.neurons.last().map_or_else(
                        || {
                            error!(
                                "Missing last neuron to continue iterating.
                                This should not be possible as the limits are checked. Stopping loop here."
                            );
                            None
                        },
                        |n| {
                            continue_scanning = true;
                            if is_test_mode && number_of_scanned_neurons == 400 {
                                continue_scanning = false;
                            }
                            n.id.clone()
                        }
                    );
                }
                neurons.extend(response.neurons);
                number_of_scanned_neurons += number_of_received_neurons;
            }
            Err(e) => {
                error!("Failed to obtain all neurons data {:?}", e);
            }
        }
    }
    Ok(neurons)
}

// Calculate total available rewards for given neurons
async fn calculate_available_rewards(
    neurons: &[Neuron],
    ogy_sns_rewards_canister_id: Principal,
    sns_ledger_canister_id: Principal,
) -> Nat {
    let mut available_rewards_amount: Nat = Nat::from(0u64);
    for neuron in neurons {
        if let Some(_) = &neuron.id {
            let neuron_rewrds = fetch_neuron_reward_balance(
                sns_ledger_canister_id,
                ogy_sns_rewards_canister_id,
                neuron.id.as_ref().unwrap(),
            )
            .await;
            available_rewards_amount = available_rewards_amount + neuron_rewrds;
        }
    }
    available_rewards_amount
}

// Function to claim rewards for each neuron
async fn claim_rewards(neurons: &[Neuron], sns_governance_canister_id: Principal) {
    for neuron in neurons {
        if let Some(neuron_id) = &neuron.id {
            let args = ogy_sns_rewards_api_canister::claim_reward::Args {
                neuron_id: neuron_id.clone(),
                token: String::from("OGY"),
            };

            match ogy_sns_rewards_c2c_client::claim_reward(sns_governance_canister_id, &args).await
            {
                Ok(_) => info!("Successfully claimed rewards for neuron {}", neuron_id),
                Err(e) => error!("Failed to claim rewards for neuron {}: {:?}", neuron_id, e),
            }
        } else {
            error!("Neuron has no ID, cannot claim rewards");
        }
    }
}

async fn distribute_rewards(sns_ledger_canister_id: Principal, available_rewards_amount: Nat) {
    match transfer_token(
        [0; 32],
        SNS_REWARDS_CANISTER_ID.into(),
        sns_ledger_canister_id,
        available_rewards_amount,
    )
    .await
    {
        Ok(_) => {
            info!("Successfully transferred rewards");
        }
        Err(error_message) => {
            error!("Error during transfer: {}", error_message);
        }
    }
}
