use crate::guards::caller_is_governance_principal;
use ic_cdk::update;
use sns_governance_canister::types::NeuronId;
pub use sns_rewards_api_canister::insert_mock_neuron_info::{
    Args as InsertMockNeuronArgs, Response as InsertMockNeuronResponse,
};

use crate::state::{mutate_state, read_state};
use types::NeuronInfo;

#[update(guard = "caller_is_governance_principal")]
async fn insert_mock_neuron(args: InsertMockNeuronArgs) -> InsertMockNeuronResponse {
    insert_mock_neuron_impl(args.neuron_id, args.neuron_info).await
}

pub async fn insert_mock_neuron_impl(
    neuron_id: NeuronId,
    neuron_info: NeuronInfo,
) -> InsertMockNeuronResponse {
    mutate_state(|state| {
        state
            .data
            .maturity_history
            .insert((neuron_id, 0), neuron_info);
    });
    Ok(())
}
