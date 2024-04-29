use candid::Principal;
use ic_cdk::update;
use sns_governance_canister::types::NeuronId;
pub use sns_rewards_api_canister::remove_neuron_ownership::{
    Args as RemoveNeuronOwnershipArgs,
    Response as RemoveNeuronOwnershipResponse,
};
use utils::env::Environment;

use crate::{
    state::{ mutate_state, read_state },
    utils::{
        authenticate_by_hotkey,
        fetch_neuron_data_by_id,
        AuthenticateByHotkeyResponse,
        FetchNeuronDataByIdResponse,
    },
};

#[update]
async fn remove_neuron_ownership(
    neuron_id: RemoveNeuronOwnershipArgs
) -> RemoveNeuronOwnershipResponse {
    let caller = read_state(|s| s.env.caller());
    remove_neuron_impl(neuron_id, caller).await
}

pub async fn remove_neuron_impl(
    neuron_id: NeuronId,
    caller: Principal
) -> RemoveNeuronOwnershipResponse {
    let neuron = fetch_neuron_data_by_id(&neuron_id).await;
    let neuron = match neuron {
        FetchNeuronDataByIdResponse::InternalError(e) => {
            return RemoveNeuronOwnershipResponse::InternalError(e);
        }
        FetchNeuronDataByIdResponse::NeuronDoesNotExist => {
            return RemoveNeuronOwnershipResponse::NeuronDoesNotExist;
        }
        FetchNeuronDataByIdResponse::Ok(n) => n,
    };
    // check the neuron contains the hotkey of the callers principal
    match authenticate_by_hotkey(&neuron, &caller) {
        AuthenticateByHotkeyResponse::NeuronHotKeyAbsent => {
            return RemoveNeuronOwnershipResponse::NeuronHotKeyAbsent;
        }
        AuthenticateByHotkeyResponse::NeuronHotKeyInvalid => {
            return RemoveNeuronOwnershipResponse::NeuronHotKeyInvalid;
        }
        AuthenticateByHotkeyResponse::Ok(_) => {}
    }
    let owner = read_state(|s| s.data.neuron_owners.get_owner_of_neuron_id(&neuron_id));
    match owner {
        Some(owner_principal) => {
            if owner_principal == caller {
                // neuron is owned by caller according to our state and has a valid hotkey
                mutate_state(|s| s.data.neuron_owners.remove(&neuron_id, caller));
                return RemoveNeuronOwnershipResponse::Ok(neuron_id);
            } else {
                return RemoveNeuronOwnershipResponse::NeuronOwnerInvalid(Some(owner_principal));
            }
        }
        None => { RemoveNeuronOwnershipResponse::NeuronNotClaimed }
    }
}
