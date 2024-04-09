use candid::Principal;
use ic_cdk::update;
use sns_governance_canister::types::NeuronId;
use utils::env::Environment;

use crate::{
    state::{ mutate_state, read_state },
    types::claim_neuron_response::UserClaimErrorResponse,
    utils::{ authenticate_by_hotkey, fetch_neuron_data_by_id },
};

use UserClaimErrorResponse::*;

#[update]
async fn remove_neuron_ownership(neuron_id: NeuronId) -> Result<NeuronId, UserClaimErrorResponse> {
    let caller = read_state(|s| s.env.caller());
    remove_neuron_impl(neuron_id, caller).await
}

pub async fn remove_neuron_impl(
    neuron_id: NeuronId,
    caller: Principal
) -> Result<NeuronId, UserClaimErrorResponse> {
    let neuron = fetch_neuron_data_by_id(&neuron_id).await?;
    // check the neuron contains the hotkey of the callers principal
    authenticate_by_hotkey(&neuron, &caller)?;
    let owner = read_state(|s| s.data.neuron_owners.get_owner_of_neuron_id(&neuron_id));
    match owner {
        Some(owner_principal) => {
            if owner_principal == caller {
                // neuron is owned by caller according to our state and has a valid hotkey
                mutate_state(|s| s.data.neuron_owners.remove(&neuron_id, caller));
                return Ok(neuron_id);
            } else {
                return Err(NeuronOwnerInvalid(Some(owner_principal)));
            }
        }
        None => { Err(NeuronNotClaimed) }
    }
}
