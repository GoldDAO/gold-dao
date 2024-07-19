use async_trait::async_trait;
use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::Neuron;
use types::{CanisterId, TimestampMillis};
use utils::env::Environment;

use crate::state::read_state;
use crate::utils::ClaimRewardResult;
use crate::utils::{calculate_available_rewards, claim_rewards, distribute_rewards, fetch_neurons};

#[async_trait]
#[typetag::serde(tag = "type")]
pub trait NeuronManager: Send + Sync + NeuronConfig {
    fn sync_neurons(&mut self, neurons: &[Neuron]) -> Result<(), String> {
        self.get_neurons_mut().all_neurons = neurons.to_vec();
        Ok(())
    }

    async fn fetch_and_sync_neurons(&mut self) -> Result<(), String> {
        let sns_governance_canister_id = self.get_sns_governance_canister_id();
        let is_test_mode = read_state(|s| s.env.is_test_mode());
        let canister_id = read_state(|s| s.env.canister_id());

        // Error is handled in fetch_neurons
        let neurons = fetch_neurons(sns_governance_canister_id, canister_id, is_test_mode).await?;
        let _ = self.sync_neurons(&neurons);
        Ok(())
    }

    async fn get_available_rewards(&self) -> Nat {
        let neurons = self.get_neurons().as_ref();
        calculate_available_rewards(
            neurons,
            self.get_sns_rewards_canister_id(),
            self.get_sns_ledger_canister_id(),
        )
        .await
        .get_internal()
    }

    async fn claim_rewards(&self) -> ClaimRewardResult {
        let neurons = self.get_neurons().as_ref();
        claim_rewards(neurons, self.get_sns_ledger_canister_id()).await
    }

    async fn distribute_rewards(&self) -> Result<(), String> {
        distribute_rewards(self.get_sns_ledger_canister_id()).await
    }
}

pub trait NeuronConfig {
    fn get_sns_governance_canister_id(&self) -> CanisterId;
    fn get_sns_ledger_canister_id(&self) -> CanisterId;
    fn get_sns_rewards_canister_id(&self) -> CanisterId;
    fn get_neurons_mut(&mut self) -> &mut Neurons;
    fn get_neurons(&self) -> &Neurons;
}

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
    fn get_sns_rewards_canister_id(&self) -> CanisterId {
        self.ogy_sns_rewards_canister_id
    }
    fn get_neurons(&self) -> &Neurons {
        &self.neurons
    }
    fn get_neurons_mut(&mut self) -> &mut Neurons {
        &mut self.neurons
    }
}

#[async_trait]
#[typetag::serde]
impl NeuronManager for OgyManager {}

#[derive(Serialize, Deserialize, Clone)]
pub struct WtnManager {
    pub wtn_sns_governance_canister_id: CanisterId,
    pub wtn_sns_ledger_canister_id: CanisterId,
    pub neurons: Neurons,
}

impl Default for WtnManager {
    fn default() -> Self {
        Self {
            // FIXME: Change to the valid params
            wtn_sns_governance_canister_id: Principal::from_text("lnxxh-yaaaa-aaaaq-aadha-cai")
                .unwrap(),
            wtn_sns_ledger_canister_id: Principal::from_text("lkwrt-vyaaa-aaaaq-aadhq-cai")
                .unwrap(),
            neurons: Neurons::default(),
        }
    }
}

impl NeuronConfig for WtnManager {
    fn get_sns_governance_canister_id(&self) -> CanisterId {
        self.wtn_sns_governance_canister_id
    }
    fn get_sns_ledger_canister_id(&self) -> CanisterId {
        self.wtn_sns_ledger_canister_id
    }
    fn get_sns_rewards_canister_id(&self) -> CanisterId {
        Principal::anonymous()
    }
    fn get_neurons(&self) -> &Neurons {
        &self.neurons
    }
    fn get_neurons_mut(&mut self) -> &mut Neurons {
        &mut self.neurons
    }
}

#[async_trait]
#[typetag::serde]
impl NeuronManager for WtnManager {}

#[derive(CandidType, Deserialize, Clone)]
pub enum NeuronType {
    Ogy,
    Wtn,
    Other(CanisterId),
}

impl NeuronType {
    pub fn get_governance_canister_id(&self) -> CanisterId {
        match self {
            NeuronType::Ogy => read_state(|state| {
                state
                    .data
                    .neuron_managers
                    .ogy
                    .ogy_sns_governance_canister_id
            }),
            NeuronType::Wtn => read_state(|state| {
                state
                    .data
                    .neuron_managers
                    .wtn
                    .wtn_sns_governance_canister_id
            }),
            NeuronType::Other(canister_id) => *canister_id,
        }
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
