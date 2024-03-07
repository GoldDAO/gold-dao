use std::{borrow::Cow, fmt::Display};

use candid::{CandidType, Decode, Encode, Principal};
use serde::{ Deserialize, Serialize };
use ic_stable_structures::{storable::Bound, StableBTreeMap, Storable};
use tracing::warn;

use crate::memory::{ get_reward_pool_memory, VM };

#[derive(CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RewardPoolToken {
    ICP,
    OGY,
    // Add other possible tokens here...
}

impl RewardPoolToken {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            RewardPoolToken::ICP => b"ICP".to_vec(),
            RewardPoolToken::OGY => b"OGY".to_vec(),
            // Add other possible tokens here...
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        match bytes {
            b"ICP" => Ok(RewardPoolToken::ICP),
            b"OGY" => Ok(RewardPoolToken::OGY),
            // Add other possible tokens here...
            _ => Err("Unknown token".to_string()),
        }
    }
}

impl Storable for RewardPoolToken {

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded { max_size: 4, is_fixed_size: false };
}

impl Display for RewardPoolToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_str = match self {
            RewardPoolToken::ICP => "ICP",
            RewardPoolToken::OGY => "OGY",
            // Add other possible tokens here...
        };
        write!(f, "{}", token_str)
    }
}

/// The history of each neuron's maturity.
// NOTE: Stable structures don't need to be serialized, hence the #[serde(skip)].
#[derive(Serialize, Deserialize)]
pub struct RewardPool {
    #[serde(skip, default = "init_map")]
    pools: StableBTreeMap<RewardPoolToken, u64, VM>,
}

fn init_map() -> StableBTreeMap<RewardPoolToken, u64, VM> {
    let memory = get_reward_pool_memory();

    StableBTreeMap::init(memory)
}

impl Default for RewardPool {
    fn default() -> Self {
        Self { pools: init_map() }
    }
}

impl RewardPool { 

    pub fn get_pool_balance(&mut self, token : RewardPoolToken) -> u64 {
        let balance = self.pools.get(&token).unwrap_or(0);
        balance
    }

    pub fn increment_pool_balance(&mut self, amount: u64, token : RewardPoolToken) -> u64 {
        let balance = self.pools.get(&token).unwrap_or(0);
        balance.checked_add(amount).unwrap_or_else(|| {
            warn!("Unexpected overflow when incrementing balance of {token} reward pool.");
            0
        })
    }

    pub fn decrement_pool_balance(&mut self, amount: u64, token : RewardPoolToken) -> u64 {
        let balance = self.pools.get(&token).unwrap_or(0);
        if balance < amount {
            warn!("Derementing more than the balance of the {token} reward pool will result in a negative balance for this pool.")
        }
        balance.checked_sub(amount).unwrap_or_else(|| {
            warn!("Unexpected overflow when decrementing balance of {token} reward pool.");
            0
        })
    }

}
    

// fn history_range(
//     hist: &StableBTreeMap<(NeuronId, TimestampMillis), NeuronInfo, VM>,
//     neuron_id: NeuronId,
//     len: usize
// ) -> impl Iterator<Item = (TimestampMillis, NeuronInfo)> + '_ {
//     hist.range((neuron_id.clone(), 0)..(neuron_id, u64::MAX))
//         .take(len)
//         .map(|((_, ts), event)| (ts, event.clone()))
// }




// pub fn insert(&mut self, key: (NeuronId, TimestampMillis), val: NeuronInfo) {
//     self.history.insert(key, val);
// }

// pub fn _insert_multiple(&mut self, events: Vec<(NeuronId, TimestampMillis, NeuronInfo)>) {
//     for (neuron_id, ts, event) in events {
//         self.insert((neuron_id, ts), event);
//     }
// }

// pub fn get_maturity_history(
//     &self,
//     neuron_id: NeuronId,
//     len: usize
// ) -> Vec<(TimestampMillis, NeuronInfo)> {
//     history_range(&self.history, neuron_id, len).collect()
// }

// pub fn get(&self, size: usize) -> Vec<((NeuronId, TimestampMillis), NeuronInfo)> {
//     self.history.iter().take(size).collect()
// }
// }