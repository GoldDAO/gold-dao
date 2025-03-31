use crate::state::read_state;
use crate::types::neuron_metrics::NeuronWithMetric;
use crate::types::other_manager::OtherManager;
use crate::types::{OgyManager, WtnManager};
use crate::utils::{
    disburse_neuron_maturity, distribute_rewards, fetch_neurons, ClaimRewardResult,
};
use async_trait::async_trait;
use candid::{CandidType, Nat};
use enum_dispatch::enum_dispatch;
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use ledger_utils::compute_neuron_staking_subaccount_bytes;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::{
    manage_neuron::{
        claim_or_refresh::{By, MemoAndController},
        ClaimOrRefresh, Command,
    },
    manage_neuron_response, ManageNeuron, Neuron,
};
use tracing::{error, trace};
use types::{CanisterId, TimestampMillis};
use utils::env::Environment;
use utils::rand::generate_rand_nonce;

#[enum_dispatch]
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub enum NeuronManagerEnum {
    OgyManager(OgyManager),
    WtnManager(WtnManager),
    OtherManager(OtherManager),
}

#[enum_dispatch(NeuronManagerEnum)]
pub trait NeuronConfig {
    fn get_sns_governance_canister_id(&self) -> CanisterId;
    fn get_sns_ledger_canister_id(&self) -> CanisterId;
    fn get_neurons_mut(&mut self) -> &mut Neurons;
    fn get_neurons(&self) -> &Neurons;
}

#[async_trait]
#[enum_dispatch(NeuronManagerEnum)]
pub trait NeuronManager: NeuronConfig {
    async fn stake_sns_neuron(
        &self,
        amount: u64,
        add_disolve_delay_secs: Option<u32>,
    ) -> Result<Vec<u8>, String> {
        trace!("Starting stake_sns_neuron with amount: {}", amount);
        let nonce = generate_rand_nonce().await?;

        let governance_canister_id = self.get_sns_governance_canister_id();
        let ledger_canister_id = self.get_sns_ledger_canister_id();
        let principal = ic_cdk::api::id();

        let subaccount = compute_neuron_staking_subaccount_bytes(principal, nonce);

        trace!(
            "Initiating transfer to governance canister: {:?}",
            governance_canister_id
        );

        let transfer_result = icrc_ledger_canister_c2c_client::icrc1_transfer(
            ledger_canister_id,
            &(TransferArg {
                from_subaccount: None,
                to: Account {
                    owner: governance_canister_id,
                    subaccount: Some(subaccount),
                },
                fee: None,
                created_at_time: None,
                memo: Some(nonce.into()),
                amount: amount.into(),
            }),
        )
        .await;

        if let Err(e) = transfer_result {
            let err_msg = format!("Transfer error: {:?}", e);
            error!("{err_msg}");
            return Err(err_msg);
        }

        trace!("Calling manage_neuron to claim the neuron");
        let response = sns_governance_canister_c2c_client::manage_neuron(
            governance_canister_id,
            &(ManageNeuron {
                subaccount: vec![],
                command: Some(Command::ClaimOrRefresh(ClaimOrRefresh {
                    by: Some(By::MemoAndController(MemoAndController {
                        controller: Some(principal),
                        memo: nonce,
                    })),
                })),
            }),
        )
        .await
        .map_err(|e| {
            let err_msg = format!("Network error: {e:?}");
            error!("{err_msg}");
            err_msg
        })?;

        let neuron_id = match response.command {
            Some(manage_neuron_response::Command::ClaimOrRefresh(c)) => {
                trace!("Neuron claimed successfully: {:?}", c.refreshed_neuron_id);
                c.refreshed_neuron_id.ok_or_else(|| {
                    let err_msg = "Failed to retrieve neuron ID".to_string();
                    error!("{}", err_msg);
                    err_msg
                })?
            }
            _ => {
                let err_msg = format!("Manage neuron error (while staking) {:?}", response);
                error!("{err_msg}");
                return Err(err_msg);
            }
        };

        if let Some(additional_dissolve_delay_seconds) = add_disolve_delay_secs {
            trace!("Increasing dissolve delay for neuron: {:?}", neuron_id);

            let response = sns_governance_canister_c2c_client::manage_neuron(
                governance_canister_id,
                &(ManageNeuron {
                    subaccount: neuron_id.id.clone(),
                    command: Some(Command::Configure(
                        sns_governance_canister::types::manage_neuron::Configure {
                            operation: Some(
                                sns_governance_canister::types::manage_neuron::configure::Operation::IncreaseDissolveDelay(
                                    sns_governance_canister::types::manage_neuron::IncreaseDissolveDelay {
                                        additional_dissolve_delay_seconds,
                                    },
                                ),
                            ),
                        },
                    )),
                }),
            )
            .await
            .map_err(|e| {
                let err_msg = format!("Failed to increase dissolve delay: {:?}", e);
                error!("{err_msg}");
                err_msg
            })?;

            match response.command {
                Some(manage_neuron_response::Command::Configure(_)) => {
                    trace!("Dissolve delay increased successfully");
                }
                _ => {
                    let err_msg = format!("Failed to increase dissolve delay: {:?}", response);
                    error!("{}", err_msg);
                    return Err(err_msg);
                }
            }
        }

        Ok(neuron_id.id)
    }

    async fn get_available_sns_rewards(&self) -> Nat {
        self.get_neurons()
            .all_neurons
            .iter()
            .fold(Nat::from(0_u64), |sum, neuron| {
                sum + neuron.maturity_e8s_equivalent
            })
    }

    async fn fetch_and_sync_neurons(&mut self) -> Result<(), String> {
        let sns_governance_canister_id = self.get_sns_governance_canister_id();
        let is_test_mode = read_state(|s| s.env.is_test_mode());
        let canister_id = read_state(|s| s.env.canister_id());

        // Error is handled in fetch_neurons
        let neurons = fetch_neurons(sns_governance_canister_id, canister_id, is_test_mode).await?;

        self.get_neurons_mut().all_neurons = neurons.to_vec();
        Ok(())
    }

    fn get_neuron_metrics(&self) -> Vec<NeuronWithMetric> {
        self.get_neurons()
            .all_neurons
            .iter()
            .map(|n| {
                NeuronWithMetric::from_neuron_with_sns_gov_id(
                    n.clone(),
                    self.get_sns_governance_canister_id(),
                )
            })
            .collect()
    }
}

#[async_trait]
pub trait NeuronRewardsManager: NeuronManager {
    async fn get_available_rewards(&self) -> Nat;
    async fn claim_rewards(&self) -> ClaimRewardResult;
    async fn claim_sns_rewards(
        &self,
        rewards_destination: sns_governance_canister::types::Account,
    ) -> ClaimRewardResult {
        let neurons = &self.get_neurons().all_neurons;

        let mut neuron_ids = Vec::new();
        for neuron in neurons {
            if let Some(id) = &neuron.id {
                if let Ok(array) = id.clone().id.try_into() {
                    neuron_ids.push(array);
                }
            }
        }

        let disburse_result = disburse_neuron_maturity(
            self.get_sns_governance_canister_id(),
            neuron_ids,
            Some(rewards_destination),
        )
        .await;

        match disburse_result {
            Ok(_) => ClaimRewardResult::Succesfull,
            Err(error) => ClaimRewardResult::Partial(error.concat()),
        }
    }

    async fn distribute_rewards(&self) -> Result<(), String> {
        distribute_rewards(self.get_sns_ledger_canister_id()).await
    }
}

#[derive(CandidType, Serialize, Deserialize, Default, Clone)]
pub struct Neurons {
    pub timestamp: TimestampMillis,
    pub all_neurons: Vec<Neuron>,
}

impl Neurons {
    pub fn new(timestamp: TimestampMillis, all_neurons: Vec<Neuron>) -> Self {
        Neurons {
            timestamp,
            all_neurons,
        }
    }

    pub fn timestamp(&self) -> TimestampMillis {
        self.timestamp
    }
}

// AsRef for immutable access to the slice of neurons
impl AsRef<[Neuron]> for Neurons {
    fn as_ref(&self) -> &[Neuron] {
        &self.all_neurons
    }
}

// AsMut for mutable access to the slice of neurons
impl AsMut<[Neuron]> for Neurons {
    fn as_mut(&mut self) -> &mut [Neuron] {
        &mut self.all_neurons
    }
}
