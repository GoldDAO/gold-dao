// use crate::types::neuron_manager::NeuronRewardsManager;
// use candid::CandidType;
// use ic_cdk::query;
// use crate::state::{read_state, NeuronList};

// #[derive(CandidType)]
// pub struct ListNeuronsResponse {
//     neurons: NeuronList,
// }

// #[query]
// fn list_ogy_neurons() -> ListNeuronsResponse {
//     read_state(|s| ListNeuronsResponse {
//         neurons: s.data.neuron_managers.ogy.get_available_rewards().await,
//     })
// }
