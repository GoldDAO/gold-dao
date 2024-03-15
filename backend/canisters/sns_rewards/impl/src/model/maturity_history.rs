use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;
use types::{ NeuronInfo, TimestampMillis };
use ic_stable_structures::StableBTreeMap;

use crate::memory::{ get_maturity_history_memory, VM };

/// The history of each neuron's maturity.
// NOTE: Stable structures don't need to be serialized, hence the #[serde(skip)].
#[derive(Serialize, Deserialize)]
pub struct MaturityHistory {
    #[serde(skip, default = "init_map")]
    history: StableBTreeMap<(NeuronId, TimestampMillis), NeuronInfo, VM>,
}

fn init_map() -> StableBTreeMap<(NeuronId, TimestampMillis), NeuronInfo, VM> {
    let memory = get_maturity_history_memory();

    StableBTreeMap::init(memory)
}

impl Default for MaturityHistory {
    fn default() -> Self {
        Self { history: init_map() }
    }
}

impl MaturityHistory {
    pub fn insert(&mut self, key: (NeuronId, TimestampMillis), val: NeuronInfo) {
        self.history.insert(key, val);
    }

    pub fn _insert_multiple(&mut self, events: Vec<(NeuronId, TimestampMillis, NeuronInfo)>) {
        for (neuron_id, ts, event) in events {
            self.insert((neuron_id, ts), event);
        }
    }

    pub fn get_maturity_history(
        &self,
        neuron_id: NeuronId,
        len: usize
    ) -> Vec<(TimestampMillis, NeuronInfo)> {
        history_range(&self.history, neuron_id, len).collect()
    }

    pub fn get(&self, size: usize) -> Vec<((NeuronId, TimestampMillis), NeuronInfo)> {
        self.history.iter().take(size).collect()
    }
}

fn history_range(
    hist: &StableBTreeMap<(NeuronId, TimestampMillis), NeuronInfo, VM>,
    neuron_id: NeuronId,
    len: usize
) -> impl Iterator<Item = (TimestampMillis, NeuronInfo)> + '_ {
    hist.range((neuron_id.clone(), 0)..(neuron_id, u64::MAX))
        .take(len)
        .map(|((_, ts), event)| (ts, event.clone()))
}
