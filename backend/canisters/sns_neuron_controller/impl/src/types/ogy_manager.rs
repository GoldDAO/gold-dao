use crate::types::neuron_manager::{NeuronConfig, NeuronManager, NeuronRewardsManager, Neurons};
use crate::utils::{ClaimRewardResult, RewardSumResult};
use async_trait::async_trait;
use candid::{Nat, Principal};
use futures::future::join_all;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::{Neuron, NeuronId};
use tracing::{error, info};
use types::CanisterId;

#[derive(Serialize, Deserialize, Clone)]
pub struct OgyManager {
    pub ogy_sns_governance_canister_id: CanisterId,
    pub ogy_sns_ledger_canister_id: CanisterId,
    pub ogy_sns_rewards_canister_id: CanisterId,
    pub neurons: Neurons,
}

// NOTE: ic network parameters
impl Default for OgyManager {
    fn default() -> Self {
        Self {
            ogy_sns_governance_canister_id: Principal::from_text("lnxxh-yaaaa-aaaaq-aadha-cai")
                .unwrap(),
            ogy_sns_ledger_canister_id: Principal::from_text("lkwrt-vyaaa-aaaaq-aadhq-cai")
                .unwrap(),
            ogy_sns_rewards_canister_id: Principal::from_text("yuijc-oiaaa-aaaap-ahezq-cai")
                .unwrap(),
            neurons: Neurons::default(),
        }
    }
}

impl NeuronConfig for OgyManager {
    fn get_sns_governance_canister_id(&self) -> CanisterId {
        self.ogy_sns_governance_canister_id
    }
    fn get_sns_ledger_canister_id(&self) -> CanisterId {
        self.ogy_sns_ledger_canister_id
    }
    fn get_neurons(&self) -> &Neurons {
        &self.neurons
    }
    fn get_neurons_mut(&mut self) -> &mut Neurons {
        &mut self.neurons
    }
}

impl OgyManager {
    fn get_sns_rewards_canister_id(&self) -> CanisterId {
        self.ogy_sns_rewards_canister_id
    }
}

#[async_trait]
#[typetag::serde]
impl NeuronManager for OgyManager {}

#[async_trait]
#[typetag::serde]
impl NeuronRewardsManager for OgyManager {
    async fn get_available_rewards(&self) -> Nat {
        let neurons = self.get_neurons().as_ref();
        ogy_calculate_available_rewards(
            neurons,
            self.get_sns_rewards_canister_id(),
            self.get_sns_ledger_canister_id(),
        )
        .await
        .get_internal()
    }

    async fn claim_rewards(&self) -> ClaimRewardResult {
        let neurons = self.get_neurons().as_ref();
        ogy_claim_rewards(neurons, self.get_sns_rewards_canister_id()).await
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

// NOTE: the following function calculates the general rewards as sum of all neuron rewards.
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

pub async fn ogy_claim_rewards(
    neurons: &[Neuron],
    sns_rewards_canister_id: Principal,
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
                    match ogy_sns_rewards_c2c_client::claim_reward(sns_rewards_canister_id, &args)
                        .await
                    {
                        Ok(response) => match response {
                            ogy_sns_rewards_api_canister::claim_reward::Response::Ok(_) => Ok(()),
                            error => Err(format!(
                                "Error claiming reward for Neuron ID {}: {:?}",
                                neuron_id, error
                            )),
                        },
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
