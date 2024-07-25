use crate::types::neuron_manager::NeuronConfig;
use crate::types::neuron_manager::NeuronManager;
use crate::types::neuron_manager::Neurons;
use async_trait::async_trait;
use candid::Principal;
use serde::{Deserialize, Serialize};
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

use crate::types::neuron_manager::NeuronRewardsManager;
use crate::utils::ogy_calculate_available_rewards;
use crate::utils::ogy_claim_rewards;
use crate::utils::ClaimRewardResult;
use candid::Nat;

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
        ogy_claim_rewards(neurons, self.get_sns_ledger_canister_id()).await
    }
}
