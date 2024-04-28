use ic_cdk_macros::query;
pub use sns_rewards_api_canister::get_neuron_by_id::{
    Args as GetNeuronByIdArgs,
    Response as GetNeuronByIdResponse,
};
use crate::state::read_state;

#[query(hidden = true)]
fn get_neuron_by_id(id: GetNeuronByIdArgs) -> GetNeuronByIdResponse {
    read_state(|state| { state.data.neuron_maturity.get(&id).cloned() })
}
