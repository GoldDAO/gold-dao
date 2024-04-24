use std::collections::BTreeMap;

use candid::Principal;
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;

#[derive(Serialize, Deserialize)]
pub struct NeuronOwnership {
    owner_to_neurons: BTreeMap<Principal, Vec<NeuronId>>,
    neuron_to_owner: BTreeMap<NeuronId, Principal>,
}

impl Default for NeuronOwnership {
    fn default() -> Self {
        Self { owner_to_neurons: BTreeMap::new(), neuron_to_owner: BTreeMap::new() }
    }
}

impl NeuronOwnership {
    pub fn add(&mut self, neuron_id: &NeuronId, caller: Principal) {
        self.owner_to_neurons
            .entry(caller)
            .and_modify(|neurons| {
                if !neurons.contains(neuron_id) {
                    neurons.push(neuron_id.clone())
                }
            })
            .or_insert_with(|| { vec![neuron_id.clone()] });

        self.neuron_to_owner.insert(neuron_id.clone(), caller);
    }

    pub fn remove(&mut self, neuron_id: &NeuronId, caller: Principal) {
        // add to owner_to_neurons
        self.owner_to_neurons
            .entry(caller)
            .and_modify(|neurons| {
                if neurons.contains(neuron_id) {
                    neurons.retain(|n_id| n_id != neuron_id)
                }
            })
            .or_insert_with(|| { vec![] });

        self.neuron_to_owner.remove(&neuron_id);
    }

    pub fn get_neuron_ids_by_owner(&self, caller: Principal) -> Option<Vec<NeuronId>> {
        let neuron_ids = self.owner_to_neurons.get(&caller);
        match neuron_ids {
            Some(n_ids) => Some(n_ids.clone()),
            None => None,
        }
    }

    pub fn get_owner_of_neuron_id(&self, neuron_id: &NeuronId) -> Option<Principal> {
        let owner = self.neuron_to_owner.get(neuron_id);
        match owner {
            Some(o) => Some(o.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use candid::Principal;
    use sns_governance_canister::types::NeuronId;

    use crate::state::{ init_state, mutate_state, read_state, RuntimeState };

    fn init_runtime_state() {
        init_state(RuntimeState::default());
    }

    #[test]
    fn test_neuron_owners() {
        init_runtime_state();
        let neuron_id = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let caller = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

        mutate_state(|s| s.data.neuron_owners.add(&neuron_id, caller));
        let neurons = read_state(|s|
            s.data.neuron_owners.get_neuron_ids_by_owner(caller.clone()).unwrap()
        );
        assert_eq!(neurons.len(), 1);
        let owner = read_state(|s|
            s.data.neuron_owners.get_owner_of_neuron_id(&neuron_id).unwrap()
        );
        assert_eq!(owner, caller);

        mutate_state(|s| s.data.neuron_owners.remove(&neuron_id, caller));
        let neurons = read_state(|s|
            s.data.neuron_owners.get_neuron_ids_by_owner(caller.clone()).unwrap()
        );
        assert_eq!(neurons.len(), 0);
    }
}
