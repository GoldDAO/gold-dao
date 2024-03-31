use std::f32::consts::E;

use candid::{ CandidType, Principal };
use ic_cdk::{ caller, update };
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::{ Neuron, NeuronId };
use tracing::debug;

use crate::state::{ mutate_state, read_state };

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum NeuronOwnershipError {
    NeuronAlreadyAdded(Option<Principal>), // Neuron has already been added by a different user Principal
    NoOwner, // No hotkey found for this neuron.
    InvalidPermissions(Principal), // Neuron exists but user doesn't match the hotkey of the neuron
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum GenericError {
    InternalError(String),
    DoesNotExist,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum RewardClaimError {
    ZeroBalance,
    FailedTransfer(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum UserClaimErrorResponse {
    NeuronOwnershipError(NeuronOwnershipError),
    GenericError(GenericError),
    RewardClaimError(RewardClaimError),
}

#[update]
async fn add_neuron(neuron_id: NeuronId) -> Result<NeuronId, UserClaimErrorResponse> {
    add_neuron_impl(neuron_id, caller()).await
}

#[update]
async fn remove_neuron(neuron_id: NeuronId) -> Result<NeuronId, UserClaimErrorResponse> {
    remove_neuron_impl(neuron_id, caller()).await
}

#[update]
async fn claim_reward(neuron_id: NeuronId, token: String) -> Result<bool, UserClaimErrorResponse> {
    claim_reward_impl(neuron_id, token, caller()).await
}

pub async fn add_neuron_impl(
    neuron_id: NeuronId,
    caller: Principal
) -> Result<NeuronId, UserClaimErrorResponse> {
    let neuron = get_neuron_by_id(&neuron_id).await?;
    verify_no_ownership(&neuron)?;
    update_principal_neurons(&neuron_id, &caller);
    Ok(neuron_id)
}

pub async fn remove_neuron_impl(
    neuron_id: NeuronId,
    caller: Principal
) -> Result<NeuronId, UserClaimErrorResponse> {
    let neuron = get_neuron_by_id(&neuron_id).await?;
    verify_owned_by_caller(&neuron, &caller)?;
    // remove them TODO - need to ask Dustin if im modiying the correct state neuron_principals
    Ok(neuron_id)
}

pub async fn claim_reward_impl(
    neuron_id: NeuronId,
    token: String,
    caller: Principal
) -> Result<bool, UserClaimErrorResponse> {
    let neuron = get_neuron_by_id(&neuron_id).await?;
    verify_owned_by_caller(&neuron, &caller)?;
    // get the neuron by id
    transfer_rewards(&neuron_id, caller, token).await?;
    Ok(true)
}

pub async fn get_neuron_by_id(neuron_id: &NeuronId) -> Result<Neuron, UserClaimErrorResponse> {
    let canister_id = read_state(|state| state.data.sns_governance_canister);
    let args = sns_governance_canister::list_neurons::Args {
        limit: 1,
        start_page_at: Some(neuron_id.clone()),
        of_principal: None,
    };
    let res = sns_governance_canister_c2c_client::list_neurons(canister_id, &args).await;
    match res {
        Ok(neuron_data) => {
            if let Some(single_neuron) = neuron_data.neurons.get(0) {
                Ok(single_neuron.clone())
            } else {
                Err(UserClaimErrorResponse::GenericError(GenericError::DoesNotExist))
            }
        }
        Err(e) => {
            debug!(
                "Error fetching neuron with id : {:?}, error code : {:?}, message : {:?}",
                neuron_id,
                e.0,
                e.1
            );
            Err(UserClaimErrorResponse::GenericError(GenericError::InternalError(e.1)))
        }
    }
}

pub fn verify_no_ownership(neuron_data: &Neuron) -> Result<bool, UserClaimErrorResponse> {
    // if a neuron has at least 2 hotkeys ( principals ) then it's already owned
    match neuron_data.permissions.get(1) {
        Some(owner) => {
            Err(
                UserClaimErrorResponse::NeuronOwnershipError(
                    NeuronOwnershipError::NeuronAlreadyAdded(owner.principal)
                )
            )
        }
        None => { Ok(true) }
    }
}

pub fn verify_owned_by_caller(
    neuron_data: &Neuron,
    caller: &Principal
) -> Result<bool, UserClaimErrorResponse> {
    match neuron_data.permissions.get(1) {
        Some(owner) => {
            if let Some(principal) = owner.principal {
                if &principal == caller {
                    Ok(true)
                } else {
                    Err(
                        UserClaimErrorResponse::NeuronOwnershipError(
                            NeuronOwnershipError::InvalidPermissions(principal)
                        )
                    )
                }
            } else {
                Err(UserClaimErrorResponse::NeuronOwnershipError(NeuronOwnershipError::NoOwner))
            }
        }
        None => { Err(UserClaimErrorResponse::NeuronOwnershipError(NeuronOwnershipError::NoOwner)) }
    }
}

pub fn update_principal_neurons(neuron_id: &NeuronId, user_id: &Principal) {
    mutate_state(|s| {
        let neurons = &mut s.data.principal_neurons;
        neurons
            .entry(user_id.clone())
            .and_modify(|neurons| {
                if !neurons.contains(&neuron_id) {
                    neurons.push(neuron_id.clone());
                }
            })
            .or_insert_with(|| { vec![neuron_id.clone()] });
    })
}

pub async fn transfer_rewards(
    neuron_id: &NeuronId,
    user_id: Principal,
    token: String
) -> Result<bool, UserClaimErrorResponse> {
    todo!();
}
