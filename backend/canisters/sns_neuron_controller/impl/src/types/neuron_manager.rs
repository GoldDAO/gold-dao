use crate::state::read_state;
use crate::utils::calculate_available_rewards;
use crate::utils::distribute_rewards;
use crate::utils::fetch_neurons;
use crate::utils::ogy_claim_rewards;
use candid::CandidType;
use candid::Nat;
use candid::Principal;
use serde::Deserialize;
use serde::Serialize;
use sns_governance_canister::types::Neuron;
use types::CanisterId;
use types::TimestampMillis;
use utils::env::Environment;

use async_trait::async_trait;

#[async_trait]
#[typetag::serde(tag = "type")]
pub trait NeuronManager: Send + Sync {
    fn get_governance_canister_id(&self) -> CanisterId;
    fn sync_neurons(&mut self, neurons: Vec<Neuron>) -> Result<(), String>;
    async fn fetch_and_sync_neurons(&mut self) -> Result<(), String> {
        let sns_governance_canister_id = self.get_governance_canister_id();
        let is_test_mode = read_state(|s| s.env.is_test_mode());
        let canister_id = read_state(|s| s.env.canister_id());

        let neurons = fetch_neurons(sns_governance_canister_id, canister_id, is_test_mode)
            .await
            .unwrap();
        let _ = self.sync_neurons(neurons.clone());
        Ok(())
    }
    async fn get_available_rewards(&self) -> Result<Nat, String>;
    async fn claim_rewards(&self) -> Result<(), String>;
    async fn distribute_rewards(&self) -> Result<(), String>;
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

#[async_trait]
#[typetag::serde]
impl NeuronManager for OgyManager {
    fn get_governance_canister_id(&self) -> CanisterId {
        self.ogy_sns_governance_canister_id
    }
    fn sync_neurons(&mut self, neurons: Vec<Neuron>) -> Result<(), String> {
        self.neurons.all_neurons = neurons;
        Ok(())
    }
    async fn get_available_rewards(&self) -> Result<Nat, String> {
        let neurons = &self.neurons.all_neurons;
        let available_rewards = calculate_available_rewards(
            neurons,
            self.ogy_sns_rewards_canister_id,
            self.ogy_sns_ledger_canister_id,
        )
        .await;
        // self.sync_available_rewards(available_rewards);
        Ok(available_rewards)
    }
    async fn claim_rewards(&self) -> Result<(), String> {
        ogy_claim_rewards(&self.neurons.all_neurons, self.ogy_sns_ledger_canister_id).await;
        Ok(())
    }
    async fn distribute_rewards(&self) -> Result<(), String> {
        let available_rewards = self.get_available_rewards().await.unwrap();
        distribute_rewards(self.ogy_sns_ledger_canister_id, available_rewards).await;
        Ok(())
    }
}

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

#[async_trait]
#[typetag::serde]
impl NeuronManager for WtnManager {
    fn get_governance_canister_id(&self) -> CanisterId {
        self.wtn_sns_governance_canister_id
    }
    fn sync_neurons(&mut self, _neurons: Vec<Neuron>) -> Result<(), String> {
        Ok(())
    }
    async fn get_available_rewards(&self) -> Result<Nat, String> {
        Ok(Nat::default())
    }
    async fn claim_rewards(&self) -> Result<(), String> {
        Ok(())
    }
    async fn distribute_rewards(&self) -> Result<(), String> {
        Ok(())
    }
}

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
