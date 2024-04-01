use candid::{ CandidType, Nat, Principal };
use ic_cdk::{ caller, query, update };
use icrc_ledger_types::icrc1::{ account::Account, transfer::TransferArg };
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::{ Neuron, NeuronId, NeuronPermission };
use tracing::{ debug, error };

use crate::state::{ mutate_state, read_state };

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum UserClaimErrorResponse {
    NoHotkeysExist, // No hotkeys found for neuron
    InvalidOwnership(Principal), // Neuron has a hotkey owned by a different caller
    NotClaimed, // Nobody has claimed this neuron yet.
    InternalError(String),
    DoesNotExist,
    ZeroBalance,
    FailedTransfer(String),
}

use UserClaimErrorResponse::*;

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

#[query]
async fn get_neurons_by_owner() -> Option<Vec<NeuronId>> {
    get_neurons_by_owner_impl(caller())
}

pub async fn add_neuron_impl(
    neuron_id: NeuronId,
    caller: Principal
) -> Result<NeuronId, UserClaimErrorResponse> {
    let neuron = fetch_neuron_by_id(&neuron_id).await?;
    // check the neuron contains the hotkey of the callers principal
    neuron_contains_hotkey_of_caller(&neuron, &caller)?;
    // check if the neuron has already been claimed
    let owner = read_state(|s| s.data.neuron_owners.get_owner_of_neuron_id(&neuron_id));
    match owner {
        Some(principal) => {
            // if the principal is the same as the caller - already added ( by you )
            if principal == caller {
                return Ok(neuron_id);
            } else {
                // if the principal is not the same as the caller - already added ( by someone else )
                return Err(InvalidOwnership(principal));
            }
        }
        None => {
            mutate_state(|s| s.data.neuron_owners.add(&neuron_id, caller));
            Ok(neuron_id)
        }
    }
}

pub async fn remove_neuron_impl(
    neuron_id: NeuronId,
    caller: Principal
) -> Result<NeuronId, UserClaimErrorResponse> {
    let neuron = fetch_neuron_by_id(&neuron_id).await?;
    // check the neuron contains the hotkey of the callers principal
    neuron_contains_hotkey_of_caller(&neuron, &caller)?;
    // check if the neuron has already been claimed
    let owner = read_state(|s| s.data.neuron_owners.get_owner_of_neuron_id(&neuron_id));
    match owner {
        Some(principal) => {
            // if the principal is the same as the caller - already added ( by you )
            if principal == caller {
                // remove the neuron
                mutate_state(|s| s.data.neuron_owners.remove(&neuron_id, caller));
                return Ok(neuron_id);
            } else {
                // if the principal is not the same as the caller - already added ( by them )
                return Err(InvalidOwnership(principal));
            }
        }
        None => { Err(NotClaimed) }
    }
}

pub async fn claim_reward_impl(
    neuron_id: NeuronId,
    token: String,
    caller: Principal
) -> Result<bool, UserClaimErrorResponse> {
    let neuron = fetch_neuron_by_id(&neuron_id).await?;
    // check the neuron contains the hotkey of the callers principal
    neuron_contains_hotkey_of_caller(&neuron, &caller)?;
    // check if the neuron has already been claimed
    let owner = read_state(|s| s.data.neuron_owners.get_owner_of_neuron_id(&neuron_id));
    match owner {
        Some(principal) => {
            // if the principal is the same as the caller - already added ( by you )
            if principal == caller {
                return transfer_rewards(&neuron_id, caller, token).await;
            } else {
                // if the principal is not the same as the caller - already added ( by them )
                return Err(InvalidOwnership(principal));
            }
        }
        None => { Err(NotClaimed) }
    }
}

pub fn get_neurons_by_owner_impl(caller: Principal) -> Option<Vec<NeuronId>> {
    read_state(|s| s.data.neuron_owners.get_neuron_ids_by_owner(caller))
}

pub async fn fetch_neuron_by_id(neuron_id: &NeuronId) -> Result<Neuron, UserClaimErrorResponse> {
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
                Err(DoesNotExist)
            }
        }
        Err(e) => {
            debug!(
                "Error fetching neuron with id : {:?}, error code : {:?}, message : {:?}",
                neuron_id,
                e.0,
                e.1
            );
            Err(InternalError(e.1))
        }
    }
}

pub fn neuron_contains_hotkey_of_caller(
    neuron_data: &Neuron,
    caller: &Principal
) -> Result<bool, UserClaimErrorResponse> {
    // skip the first because that is always the owner of the neuron
    let valid: Vec<NeuronPermission> = neuron_data.permissions
        .clone()
        .into_iter()
        .skip(1)
        .filter(|permission| {
            match permission.principal {
                Some(principal) => {
                    if &principal == caller { true } else { false }
                }
                None => { false }
            }
        })
        .collect();

    if valid.len() == 1 {
        return Ok(true);
    } else {
        return Err(NoHotkeysExist);
    }
}

pub async fn transfer_rewards(
    neuron_id: &NeuronId,
    user_id: Principal,
    token: String
) -> Result<bool, UserClaimErrorResponse> {
    todo!();
    // verify token is correct with a parse
    // get balance of sub account
    // let balance = fetch_neuron_rewards_balance()
    // transfer all from sub account to user_id
}

async fn fetch_neuron_rewards_balance(
    ledger_canister_id: Principal,
    neuron_id: &NeuronId
) -> Result<Nat, UserClaimErrorResponse> {
    match
        icrc_ledger_canister_c2c_client::icrc1_balance_of(
            ledger_canister_id,
            &(Account {
                owner: ic_cdk::api::id(),
                subaccount: Some(neuron_id.into()),
            })
        ).await
    {
        Ok(t) => { Ok(t) }
        Err(e) => {
            error!("Fail - to neuron rewards: {:?}", e.1);
            Err(InternalError(e.1))
        }
    }
}

async fn transfer_token(
    from_sub_account: NeuronId,
    to_account: Account,
    ledger_id: Principal,
    amount: Nat
) -> Result<(), String> {
    match
        icrc_ledger_canister_c2c_client::icrc1_transfer(
            ledger_id,
            &(TransferArg {
                from_subaccount: Some(from_sub_account.into()),
                to: to_account,
                fee: None,
                created_at_time: None,
                amount: amount,
                memo: None,
            })
        ).await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(error)) => Err(format!("Transfer error: {error:?}")),
        Err(error) => Err(format!("Network error: {error:?}")),
    }
}
