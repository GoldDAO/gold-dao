use crate::state::mutate_state;
use sns_governance_canister::types::NeuronId;
pub use sns_rewards_api_canister::_insert_mock_neuron::{
    Args as InsertMockNeuronArgs, Response as InsertMockNeuronResponse,
};
use types::NeuronInfo;

#[cfg(feature = "inttest")]
use crate::guards::caller_is_governance_principal;
#[cfg(feature = "inttest")]
use ic_cdk::update;

#[update(guard = "caller_is_governance_principal")]
#[cfg(feature = "inttest")]
async fn _insert_mock_neuron(args: InsertMockNeuronArgs) -> InsertMockNeuronResponse {
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
