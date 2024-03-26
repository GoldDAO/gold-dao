use candid::{ CandidType, Principal };
use ic_cdk::update;
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;

use crate::state::mutate_state;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum AddNeuronResponseError {
    NeuronAlreadyAdded(Principal), // Neuron has already been added by a different user Principal
    MissingHotkey, // No hotkey found for this neuron.
    InvalidHotkeyPermissions, // hotkey exists but it doesn't contain the correct permissions.
    InvalidPermissions(Principal), // Neuron exists but user doesn't match the hotkey of the neuron
    InternalError(String),
}

#[update]
async fn add_neuron(neuron_id: NeuronId) -> Result<NeuronId, AddNeuronResponseError> {
    // - get GLD NeuronData
    // it possible that the neuron is fresh and because our sync happens every 24 hours for maturity, we wont be able to get that data.
    // we may have to manually query the neuron to get the most up to date data.
    // check neuron has only 2 principals
    // check one of them is the call's id and that the one found has correct permissions 3,4.
    // if all checks pass
    // add to a map of Map<NeuronId, Principal>
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
