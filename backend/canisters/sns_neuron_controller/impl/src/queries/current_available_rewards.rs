use crate::guards::caller_is_governance_principal;
use crate::state::read_state;
use crate::types::neuron_manager::NeuronRewardsManager;
use candid::Nat;
use ic_cdk::query;

#[query(guard = "caller_is_governance_principal", hidden = true)]
async fn current_available_rewards() -> Nat {
    let ogy_neuron_manager = read_state(|s| s.data.neuron_managers.ogy.clone());
    ogy_neuron_manager.get_available_rewards().await
}
