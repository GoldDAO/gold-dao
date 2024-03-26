use std::collections::BTreeMap;

use candid::Principal;
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;

/// The history of each neuron's maturity.
// NOTE: Stable structures don't need to be serialized, hence the #[serde(skip)].
#[derive(Serialize, Deserialize)]
pub struct NeuronOwners {
    owners: BTreeMap<NeuronId, Principal>,
}

impl Default for NeuronOwners {
    fn default() -> Self {
        Self { owners: BTreeMap::new() }
    }
}

impl NeuronOwners {
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
}
