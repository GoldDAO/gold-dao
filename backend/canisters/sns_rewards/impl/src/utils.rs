use candid::{ Nat, Principal };
use icrc_ledger_types::icrc1::{ account::{ Account, Subaccount }, transfer::TransferArg };
use sns_governance_canister::types::{ Neuron, NeuronId };
use tracing::debug;

use crate::{ state::read_state, types::claim_neuron_response::UserClaimErrorResponse };

use sns_governance_canister::types::get_neuron_response::Result::{
    Neuron as NeuronResponse,
    Error as NeuronErrorResponse,
};

use UserClaimErrorResponse::*;

pub async fn transfer_token(
    from_sub_account: Subaccount,
    to_account: Account,
    ledger_id: Principal,
    amount: Nat
) -> Result<(), String> {
    match
        icrc_ledger_canister_c2c_client::icrc1_transfer(
            ledger_id,
            &(TransferArg {
                from_subaccount: Some(from_sub_account),
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

pub async fn fetch_neuron_data_by_id(
    neuron_id: &NeuronId
) -> Result<Neuron, UserClaimErrorResponse> {
    let canister_id = read_state(|state| state.data.sns_governance_canister);
    let args = sns_governance_canister::get_neuron::Args {
        neuron_id: Some(neuron_id.clone()),
    };
    match sns_governance_canister_c2c_client::get_neuron(canister_id, &args).await {
        Ok(neuron_data) => {
            match neuron_data.result {
                Some(neuron) => {
                    match neuron {
                        NeuronResponse(n) => Ok(n),
                        NeuronErrorResponse(_) => Err(NeuronDoesNotExist),
                    }
                }
                None => Err(NeuronDoesNotExist),
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

pub fn authenticate_by_hotkey(
    neuron_data: &Neuron,
    caller: &Principal
) -> Result<bool, UserClaimErrorResponse> {
    // first is always the nns owner principal so if less than or equal to 1 then no hotkeys have been added.
    if neuron_data.permissions.len() <= 1 {
        return Err(NeuronHotKeyAbsent);
    }

    // Check if any of the permission principals contain an entry that matches the caller principal
    let matching_caller_hotkey = neuron_data.permissions
        .iter()
        .skip(1)
        .filter(|permission| permission.principal.as_ref() == Some(caller))
        .count();

    if matching_caller_hotkey >= 1 {
        Ok(true)
    } else {
        Err(NeuronHotKeyInvalid)
    }
}
