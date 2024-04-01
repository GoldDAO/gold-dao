use std::f32::consts::E;

use candid::{ CandidType, Principal };
use ic_cdk::{ caller, update };
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::{ Neuron, NeuronId, NeuronPermission };
use tracing::debug;

use crate::state::{ mutate_state, read_state };

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum NeuronOwnershipError {
    NoHotkeysExist, // No hotkeys found for neuron
    InvalidOwnership(Principal), // Neuron has a hotkey owned by a different caller
    InvalidPrincipal, // Principal is invalid
    InvalidHotkeyPermissions(String), // Permissions of the hoykey are invalid
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
    verify_ownership(&neuron, &caller)?;
    add_owner_to_principal_neurons_map(&neuron_id, &caller);
    Ok(neuron_id)
}

pub async fn remove_neuron_impl(
    neuron_id: NeuronId,
    caller: Principal
) -> Result<NeuronId, UserClaimErrorResponse> {
    let neuron = get_neuron_by_id(&neuron_id).await?;
    verify_ownership(&neuron, &caller)?;
    remove_owner_from_principal_neurons_map(&neuron_id, &caller);
    Ok(neuron_id)
}

pub async fn claim_reward_impl(
    neuron_id: NeuronId,
    token: String,
    caller: Principal
) -> Result<bool, UserClaimErrorResponse> {
    let neuron = get_neuron_by_id(&neuron_id).await?;
    verify_ownership(&neuron, &caller)?;
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

pub fn verify_ownership(
    neuron_data: &Neuron,
    caller: &Principal
) -> Result<bool, UserClaimErrorResponse> {
    // skip the first because that is always the owner of the neuron
    let permissions: Vec<NeuronPermission> = neuron_data.permissions
        .clone()
        .into_iter()
        .skip(1)
        .collect();
    match permissions.get(1) {
        Some(permission) => {
            if
                !&permission.permission_type.contains(&3) &&
                !&permission.permission_type.contains(&4)
            {
                return Err(
                    UserClaimErrorResponse::NeuronOwnershipError(
                        NeuronOwnershipError::InvalidHotkeyPermissions(
                            "Should have permissions 3 and 4".to_string()
                        )
                    )
                );
            }
            if let Some(neuron_principal) = permission.principal {
                if &neuron_principal == caller {
                    return Ok(true);
                } else {
                    return Err(
                        UserClaimErrorResponse::NeuronOwnershipError(
                            NeuronOwnershipError::InvalidOwnership(neuron_principal)
                        )
                    );
                }
            } else {
                return Err(
                    UserClaimErrorResponse::NeuronOwnershipError(
                        NeuronOwnershipError::InvalidPrincipal
                    )
                );
            }
        }
        None => {
            return Err(
                UserClaimErrorResponse::NeuronOwnershipError(NeuronOwnershipError::NoHotkeysExist)
            );
        }
    }
}

pub fn add_owner_to_principal_neurons_map(neuron_id: &NeuronId, user_id: &Principal) {
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

pub fn remove_owner_from_principal_neurons_map(neuron_id: &NeuronId, user_id: &Principal) {
    mutate_state(|s| {
        let neurons = &mut s.data.principal_neurons;
        neurons
            .entry(user_id.clone())
            .and_modify(|neurons| {
                neurons.retain_mut(|n_id| n_id != neuron_id);
            })
            .or_insert_with(|| { vec![] });
    })
}

pub async fn transfer_rewards(
    neuron_id: &NeuronId,
    user_id: Principal,
    token: String
) -> Result<bool, UserClaimErrorResponse> {
    todo!();
    // verify ownership
    // verify token is correct with a parse
    // get balance of sub account
    // transfer all from sub account to user_id
}
