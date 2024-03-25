use candid::{ CandidType, Principal };
use ic_cdk::update;
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;

use crate::state::mutate_state;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum AddNeuronResponseError {
    NeuronAlreadyAdded(Principal), // Neuron has already been added by a different user Principal
    InvalidPermissions(Principal), // Neuron exists but user doesn't match the hotkey of the neuron
    InternalError(String),
}

#[update]
async fn add_neuron(neuron_id: NeuronId) -> Result<NeuronId, AddNeuronResponseError> {
    // mutate_state(|state| )
    todo!();
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum RemoveNeuronResponseError {
    InvalidPermissions, // Neuron exists but user doesn't match the hotkey of the neuron
    DoesNotExist, // Couldn't find neuron with this ID
    InternalError(String),
}

#[update]
async fn remove_neuron(neuron_id: NeuronId) -> Result<NeuronId, RemoveNeuronResponseError> {
    todo!();
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum ClaimRewardResponseError {
    InvalidPermissions, // Neuron exists but user doesn't match the hotkey of the neuron
    DoesNotExist, // Couldn't find neuron with this ID
    InternalError(String),
    NoRewards, // reject claiming a reward of 0
    TransactionFail(String), // Transaction from reward account to user account failed
}

#[update]
async fn claim_reward(neuron_id: NeuronId) -> Result<bool, ClaimRewardResponseError> {
    todo!();
}
