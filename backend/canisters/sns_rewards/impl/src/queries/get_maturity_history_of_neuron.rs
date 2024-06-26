use ic_cdk_macros::query;
pub use sns_rewards_api_canister::get_maturity_history_of_neuron::{
    Args as GetMaturityHistoryOfNeuronArgs,
    Response as GetMaturityHistoryOfNeuronResponse,
};

use crate::state::read_state;

#[query(hidden = true)]
fn get_maturity_history_of_neuron(
    args: GetMaturityHistoryOfNeuronArgs
) -> GetMaturityHistoryOfNeuronResponse {
    read_state(|state| {
        state.data.maturity_history.get_maturity_history(args.neuron_id, args.size.unwrap_or(100))
    })
}
