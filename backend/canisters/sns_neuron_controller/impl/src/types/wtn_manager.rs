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
pub struct WtnManager {
    pub wtn_sns_governance_canister_id: CanisterId,
    pub wtn_sns_ledger_canister_id: CanisterId,
    pub neurons: Neurons,
    pub icp_ledger: Principal,
    pub icp_rewards_threshold: Nat,
    pub wtn_rewards_threshold: Nat,
}

impl Default for WtnManager {
    fn default() -> Self {
        Self {
            wtn_sns_governance_canister_id: Principal::from_text("jfnic-kaaaa-aaaaq-aadla-cai")
                .unwrap(),
            wtn_sns_ledger_canister_id: Principal::from_text("jcmow-hyaaa-aaaaq-aadlq-cai")
                .unwrap(),
            neurons: Neurons::default(),
            icp_ledger: Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
            icp_rewards_threshold: Nat::from(10_000_000_000_u64), // 100 ICP
            wtn_rewards_threshold: Nat::from(1_000_000_000_000_u64), // 10'000 WTN
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
impl NeuronManager for WtnManager {}

use crate::utils::ClaimRewardResult;
#[async_trait]
impl NeuronRewardsManager for WtnManager {
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

use sns_neuron_controller_api_canister::init::WtnManagerConfig;
impl From<WtnManagerConfig> for WtnManager {
    fn from(config: WtnManagerConfig) -> Self {
        WtnManager {
            wtn_sns_governance_canister_id: config.wtn_sns_governance_canister_id,
            wtn_sns_ledger_canister_id: config.wtn_sns_ledger_canister_id,
            neurons: Neurons::default(), // Initializes an empty neuron map
            icp_ledger: config.icp_ledger,
            icp_rewards_threshold: config.icp_rewards_threshold,
            wtn_rewards_threshold: config.wtn_rewards_threshold,
        }
    }
}
