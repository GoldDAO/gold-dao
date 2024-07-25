use crate::state::read_state;
use candid::{Nat, Principal};
use futures::future::join_all;
use icrc_ledger_types::icrc1::{
    account::{Account, Subaccount},
    transfer::TransferArg,
};
use sns_governance_canister::types::ListNeurons;
use sns_governance_canister::types::Neuron;
use sns_governance_canister::types::NeuronId;
use tracing::debug;
use tracing::{error, info};

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

pub async fn ogy_fetch_neuron_reward_balance(
    ledger_canister_id: Principal,
    ogy_sns_rewards_canister_id: Principal,
    neuron_id: &NeuronId,
) -> Result<Nat, String> {
    match icrc_ledger_canister_c2c_client::icrc1_balance_of(
        ledger_canister_id,
        &(Account {
            owner: ogy_sns_rewards_canister_id,
            subaccount: Some(neuron_id.into()),
        }),
    )
    .await
    {
        Ok(t) => Ok(t),
        Err(e) => {
            let error_message = format!(
                "Failed to fetch token balance of ledger canister id {} with ERROR : {:?}",
                ledger_canister_id, e
            );
            error!("{}", error_message);
            Err(error_message)
        }
    }
}

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

pub enum RewardSumResult {
    Full(Nat),
    // Needed for update call in orer to return error
    Partial(Nat, String),
    Empty,
}

impl RewardSumResult {
    pub fn get_internal(self) -> Nat {
        match self {
            RewardSumResult::Full(nat) => nat,
            RewardSumResult::Partial(nat, _) => nat,
            RewardSumResult::Empty => Nat::from(0u8),
        }
    }
}

// NOTE: the following function calculates the general rewards as sum of all neurons rewards.
// If one of the rewards cannot be fetched, the general reward is calculated anyway, but it's
// defined as RewardSumResult::Partial
pub async fn ogy_calculate_available_rewards(
    neurons: &[Neuron],
    ogy_sns_rewards_canister_id: Principal,
    sns_ledger_canister_id: Principal,
) -> RewardSumResult {
    let futures: Vec<_> = neurons
        .iter()
        .filter_map(|neuron| {
            neuron.id.as_ref().map(|id| {
                ogy_fetch_neuron_reward_balance(
                    sns_ledger_canister_id,
                    ogy_sns_rewards_canister_id,
                    id,
                )
            })
        })
        .collect();

    let results = join_all(futures).await;

    let mut available_rewards_amount: Nat = Nat::from(0u64);
    let mut error_messages = Vec::new();
    for result in results {
        match result {
            Ok(reward) => {
                available_rewards_amount += reward;
            }
            Err(error) => {
                error!("Failed to fetch neuron reward balance: {error}");
                error_messages.push(error);
            }
        }
    }

    if error_messages.is_empty() {
        info!("Successfully got available rewards amount");
        RewardSumResult::Full(available_rewards_amount)
    } else {
        let error_message = error_messages.join("\n");
        // NOTE: uncomment to be able to debug the errors
        // error!(
        //     "Failed to get available rewards amount: {:?}",
        //     error_message
        // );
        if error_messages.len() >= neurons.len() {
            error!("Failed to get ALL neurons available rewards amount");
            RewardSumResult::Empty
        } else {
            error!("Failed to get SOME neurons available rewards amount");
            RewardSumResult::Partial(available_rewards_amount, error_message)
        }
    }
}

pub enum ClaimRewardResult {
    Succesfull,
    Partial(String),
    Failed,
}

impl ClaimRewardResult {
    pub fn is_not_failed(&self) -> bool {
        !matches!(self, ClaimRewardResult::Failed)
    }
}

// FIXME: handle an error like in calculate_available_rewards, use also Empty result
pub async fn ogy_claim_rewards(
    neurons: &[Neuron],
    sns_governance_canister_id: Principal,
) -> ClaimRewardResult {
    let futures: Vec<_> = neurons
        .iter()
        .filter_map(|neuron| {
            neuron.id.as_ref().map(|neuron_id| {
                let args = ogy_sns_rewards_api_canister::claim_reward::Args {
                    neuron_id: neuron_id.clone(),
                    token: String::from("OGY"),
                };

                async move {
                    match ogy_sns_rewards_c2c_client::claim_reward(
                        sns_governance_canister_id,
                        &args,
                    )
                    .await
                    {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!(
                            "Failed to claim rewards for Neuron ID {}: {:?}",
                            neuron_id, e
                        )),
                    }
                }
            })
        })
        .collect();

    let results = join_all(futures).await;

    let mut error_messages = Vec::new();
    for result in results {
        if let Err(e) = result {
            error_messages.push(e);
        }
    }

    if error_messages.is_empty() {
        info!("Successfully claimed rewards for all neurons");
        ClaimRewardResult::Succesfull
    } else {
        // NOTE: uncomment to be able to debug the errors
        // let error_message = error_messages.join("\n");
        // error!(
        //     "Failed to claim rewards for some neurons:\n{}",
        //     error_message
        // );
        error!("Failed to claim rewards for neurons");
        ClaimRewardResult::Partial(error_messages.join("\n"))
    }
}

// FIXME: think of outstanding payments struct in this context
pub async fn distribute_rewards(sns_ledger_canister_id: Principal) -> Result<(), String> {
    let sns_rewards_canister_id = read_state(|state| state.data.sns_rewards_canister_id);
    // Transfer all the tokens to sns_rewards to be distributed
    match icrc_ledger_canister_c2c_client::icrc1_balance_of(
        sns_ledger_canister_id,
        &(Account {
            owner: ic_cdk::api::id(),
            subaccount: None,
        }),
    )
    .await
    {
        Ok(balance) => {
            match transfer_token(
                [0; 32],
                sns_rewards_canister_id.into(),
                sns_ledger_canister_id,
                balance,
            )
            .await
            {
                Ok(_) => {
                    info!("Successfully transferred rewards");
                    Ok(())
                }
                Err(error_message) => {
                    let error_message = format!("Error during transfer rewards: {}", error_message);
                    error!(error_message);
                    Err(error_message)
                }
            }
        }
        Err(e) => {
            let error_message = format!(
                "Failed to fetch token balance of sns_neuron_controller from ledger canister id {} with ERROR : {:?}",
                sns_ledger_canister_id, e
            );
            error!("{}", error_message);
            Err(error_message)
        }
    }
}
