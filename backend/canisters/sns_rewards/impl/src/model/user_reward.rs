use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;
use types::StoredSubaccount;

use crate::memory::{get_user_reward_accounts, VM};

/// The history of each neuron's maturity.
// NOTE: Stable structures don't need to be serialized, hence the #[serde(skip)].
#[derive(Serialize, Deserialize)]
pub struct UserReward {
    #[serde(skip, default = "init_map")]
    reward_accounts: StableBTreeMap<NeuronId, StoredSubaccount, VM>,
}

fn init_map() -> StableBTreeMap<NeuronId, StoredSubaccount, VM> {
    let memory = get_user_reward_accounts();

    StableBTreeMap::init(memory)
}

impl Default for UserReward {
    fn default() -> Self {
        Self {
            reward_accounts: init_map(),
        }
    }
}

impl UserReward {
    pub fn create_sub_account(&mut self, neuron_id: &NeuronId) -> u64 {
        let neuron_id_as_bytes = neuron_id.clone()
            .into_array()
            .expect("Error conerting NeuronId into u8");

        let sub_account_exists = self.reward_accounts.contains_key(&neuron_id);

        if !sub_account_exists {
            let sub_account = StoredSubaccount(neuron_id_as_bytes);
            self.reward_accounts.insert(neuron_id.clone(), sub_account);
            1
        } else {
            1
        }
    }

    pub fn get_account_id_by_neuron_id(
        &mut self,
        neuron_id: NeuronId,
    ) -> Option<StoredSubaccount> {
        self.reward_accounts.get(&neuron_id)
    }
}
