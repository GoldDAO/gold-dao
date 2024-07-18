use candid::{Nat, Principal};
use futures::future::join_all;
use icrc_ledger_types::icrc1::{
    account::{Account, Subaccount},
    transfer::TransferArg,
};
use sns_governance_canister::types::NeuronId;
use tracing::{error, info};
use utils::consts::SNS_REWARDS_CANISTER_ID;

pub async fn transfer_token(
    from_sub_account: Subaccount,
    to_account: Account,
    ledger_id: Principal,
    amount: Nat,
) -> Result<(), String> {
    match icrc_ledger_canister_c2c_client::icrc1_transfer(
        ledger_id,
        &(TransferArg {
            from_subaccount: Some(from_sub_account),
            to: to_account,
            fee: None,
            created_at_time: None,
            amount,
            memo: None,
        }),
    )
    .await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(error)) => Err(format!("Transfer error: {error:?}")),
        Err(error) => Err(format!("Network error: {error:?}")),
    }
}

pub async fn fetch_neuron_reward_balance(
    ledger_canister_id: Principal,
    ogy_sns_rewards_canister_id: Principal,
    neuron_id: &NeuronId,
) -> Nat {
    match icrc_ledger_canister_c2c_client::icrc1_balance_of(
        ledger_canister_id,
        &(Account {
            owner: ogy_sns_rewards_canister_id,
            subaccount: Some(neuron_id.into()),
        }),
    )
    .await
    {
        Ok(t) => t,
        Err(e) => {
            error!(
                "Failed to fetch token balance of ledger canister id {} with ERROR : {:?}",
                ledger_canister_id, e
            );
            Nat::from(0u64)
        }
    }
}

use sns_governance_canister::types::ListNeurons;
use sns_governance_canister::types::Neuron;
use tracing::debug;
// Fetch all neurons from SNS governance canister
pub async fn fetch_neurons(
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
        // at a time. In fact, I'm not sure that we would exceed the limit in any case, but it's
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
                return Err(format!("Failed to obtain all neurons data {:?}", e));
            }
        }
    }
    Ok(neurons)
}

pub async fn calculate_available_rewards(
    neurons: &[Neuron],
    ogy_sns_rewards_canister_id: Principal,
    sns_ledger_canister_id: Principal,
) -> Nat {
    let futures: Vec<_> = neurons
        .iter()
        .filter_map(|neuron| {
            neuron.id.as_ref().map(|id| {
                fetch_neuron_reward_balance(sns_ledger_canister_id, ogy_sns_rewards_canister_id, id)
            })
        })
        .collect();

    let results = join_all(futures).await;

    let mut available_rewards_amount: Nat = Nat::from(0u64);
    for reward in results {
        available_rewards_amount += reward;
    }

    available_rewards_amount
}
// Function to claim rewards for each neuron
// TODO: handle an error here and make it parallel
pub async fn claim_rewards(neurons: &[Neuron], sns_governance_canister_id: Principal) {
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

pub async fn distribute_rewards(sns_ledger_canister_id: Principal, available_rewards_amount: Nat) {
    // Transfer all the tokens to sns_rewards to be distributed
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
