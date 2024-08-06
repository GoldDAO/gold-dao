use async_trait::async_trait;
use candid::Nat;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::Neuron;
use types::{CanisterId, TimestampMillis};
use utils::env::Environment;

use crate::state::read_state;
use crate::types::neuron_metrics::NeuronWithMetric;
use crate::utils::ClaimRewardResult;
use crate::utils::{distribute_rewards, fetch_neurons};

pub trait NeuronConfig {
    fn get_sns_governance_canister_id(&self) -> CanisterId;
    fn get_sns_ledger_canister_id(&self) -> CanisterId;
    fn get_neurons_mut(&mut self) -> &mut Neurons;
    fn get_neurons(&self) -> &Neurons;
}

#[async_trait]
#[typetag::serde(tag = "type")]
pub trait NeuronManager: NeuronConfig {
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
#[typetag::serde(tag = "type")]
pub trait NeuronRewardsManager: NeuronManager {
    async fn get_available_rewards(&self) -> Nat;
    async fn claim_rewards(&self) -> ClaimRewardResult;
    async fn distribute_rewards(&self) -> Result<(), String> {
        distribute_rewards(self.get_sns_ledger_canister_id()).await
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
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
