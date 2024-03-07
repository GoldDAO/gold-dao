use std::{borrow::Cow, fmt::Display};

use candid::{CandidType, Decode, Encode, Principal};
use ic_ledger_types::AccountIdentifier;
use ic_stable_structures::{storable::Bound, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;
use tracing::warn;

use crate::memory::{get_user_reward_accounts, VM};

/// The history of each neuron's maturity.
// NOTE: Stable structures don't need to be serialized, hence the #[serde(skip)].
#[derive(Serialize, Deserialize)]
pub struct UserReward {
    #[serde(skip, default = "init_map")]
    reward_accounts: StableBTreeMap<NeuronId, AccountIdentifier, VM>,
}

fn init_map() -> StableBTreeMap<RewardPoolToken, AccountIdentifier, VM> {
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
    pub fn create_sub_account(&mut self, neuron_id: NeuronId) -> u64 {
        let sub_account = AccountIdentifier::new(&this_canister_id, neuron_id);
        self.reward_accounts
            .entry(neuron_id)
            .or_insert(neuron_id, sub_account)
    }

    pub fn get_account_by_neuron_id(&mut self, neuron_id: NeuronId) -> Option<AccountIdentifier> {
        self.reward_accounts.get(neuron_id)
    }
}
