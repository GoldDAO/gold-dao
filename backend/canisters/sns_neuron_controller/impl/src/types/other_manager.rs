use crate::state::read_state;
use crate::types::neuron_manager::NeuronConfig;
use crate::types::neuron_manager::NeuronManager;
use crate::types::neuron_manager::NeuronRewardsManager;
use crate::types::neuron_manager::Neurons;
use async_trait::async_trait;
use candid::CandidType;
use candid::Nat;
use candid::Principal;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::Account;
use types::CanisterId;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct OtherManager {
    pub sns_governance_canister_id: CanisterId,
    pub sns_ledger_canister_id: CanisterId,
    pub rewards_threshold: Nat,
    pub rewards_destination: Principal,
    pub neurons: Neurons,
}

impl OtherManager {
    pub fn new(
        sns_governance_canister_id: CanisterId,
        sns_ledger_canister_id: CanisterId,
        rewards_threshold: Nat,
        rewards_destination: Principal,
    ) -> Self {
        Self {
            sns_governance_canister_id,
            sns_ledger_canister_id,
            rewards_threshold,
            rewards_destination,
            neurons: Neurons::default(),
        }
    }
}

impl NeuronConfig for OtherManager {
    fn get_sns_governance_canister_id(&self) -> CanisterId {
        self.sns_governance_canister_id
    }
    fn get_sns_ledger_canister_id(&self) -> CanisterId {
        self.sns_ledger_canister_id
    }
    fn get_neurons(&self) -> &Neurons {
        &self.neurons
    }
    fn get_neurons_mut(&mut self) -> &mut Neurons {
        &mut self.neurons
    }
}

#[async_trait]
impl NeuronManager for OtherManager {}

use crate::utils::ClaimRewardResult;
#[async_trait]
impl NeuronRewardsManager for OtherManager {
    // NOTE: this method is not fetching the current available rewards.
    // It uses internal canister state (last fetched neurons) to do it,
    // so before calling it it's obligatory to fetch neurons
    async fn get_available_rewards(&self) -> Nat {
        self.get_available_sns_rewards().await
    }

    async fn claim_rewards(&self) -> ClaimRewardResult {
        let sns_rewards_canister_id = read_state(|state| state.data.sns_rewards_canister_id);
        self.claim_sns_rewards(Account {
            owner: Some(sns_rewards_canister_id),
            subaccount: None,
        })
        .await
    }
}
