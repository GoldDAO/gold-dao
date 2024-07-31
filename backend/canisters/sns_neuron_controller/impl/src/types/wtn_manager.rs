use crate::types::neuron_manager::NeuronConfig;
use crate::types::neuron_manager::NeuronManager;
use crate::types::neuron_manager::Neurons;
use async_trait::async_trait;
use candid::Principal;
use serde::{Deserialize, Serialize};
use types::CanisterId;

#[derive(Serialize, Deserialize, Clone)]
pub struct WtnManager {
    pub wtn_sns_governance_canister_id: CanisterId,
    pub wtn_sns_ledger_canister_id: CanisterId,
    pub neurons: Neurons,
}

impl Default for WtnManager {
    fn default() -> Self {
        Self {
            // TODO: Change to the valid params
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
