use candid::Principal;
use ic_cdk::update;
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::add_neuron_ownership::{ Args, Response };
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
async fn add_neuron_ownership(args: Args) -> Response {
    let caller = read_state(|s| s.env.caller());
    add_neuron_impl(args.neuron_id, caller).await
}

pub async fn add_neuron_impl(neuron_id: NeuronId, caller: Principal) -> Response {
    let neuron = fetch_neuron_data_by_id(&neuron_id).await;
    let neuron = match neuron {
        FetchNeuronDataByIdResponse::InternalError(e) => {
            return Response::InternalError(e);
        }
        FetchNeuronDataByIdResponse::NeuronDoesNotExist => {
            return Response::NeuronDoesNotExist;
        }
        FetchNeuronDataByIdResponse::Ok(n) => n,
    };

    // check the neuron contains the hotkey of the callers principal
    match authenticate_by_hotkey(&neuron, &caller) {
        AuthenticateByHotkeyResponse::NeuronHotKeyAbsent => {
            return Response::NeuronHotKeyAbsent;
        }
        AuthenticateByHotkeyResponse::NeuronHotKeyInvalid => {
            return Response::NeuronHotKeyInvalid;
        }
        AuthenticateByHotkeyResponse::Ok(_) => {}
    }
    let owner = read_state(|s| s.data.neuron_owners.get_owner_of_neuron_id(&neuron_id));
    match owner {
        Some(owner_principal) => {
            if owner_principal == caller {
                // neuron is owned by caller according to our state and has a valid hotkey - nothing to do
                return Response::Ok(neuron_id);
            } else {
                // hotkey is valid but neuron id is owned already so we return the principal that owns it
                return Response::NeuronOwnerInvalid(Some(owner_principal));
            }
        }
        None => {
            // we have no record in our state of the neuron_id being owned and they passed hotkey validation
            mutate_state(|s| s.data.neuron_owners.add(&neuron_id, caller));
            Response::Ok(neuron_id)
        }
    }
}
