use candid::{ Nat, Principal };
use icrc_ledger_types::icrc1::{ account::{ Account, Subaccount }, transfer::TransferArg };
use sns_governance_canister::types::{ Neuron, NeuronId };
use tracing::debug;

use crate::state::read_state;

use sns_governance_canister::types::get_neuron_response::Result::{
    Neuron as NeuronResponse,
    Error as NeuronErrorResponse,
};

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
pub enum FetchNeuronDataByIdResponse {
    NeuronDoesNotExist,
    Ok(Neuron),
    InternalError(String),
}

pub async fn fetch_neuron_data_by_id(neuron_id: &NeuronId) -> FetchNeuronDataByIdResponse {
    let canister_id = read_state(|state| state.data.sns_governance_canister);
    let args = sns_governance_canister::get_neuron::Args {
        neuron_id: Some(neuron_id.clone()),
    };
    match sns_governance_canister_c2c_client::get_neuron(canister_id, &args).await {
        Ok(neuron_data) => {
            match neuron_data.result {
                Some(neuron) => {
                    match neuron {
                        NeuronResponse(n) => FetchNeuronDataByIdResponse::Ok(n),
                        NeuronErrorResponse(_) => FetchNeuronDataByIdResponse::NeuronDoesNotExist,
                    }
                }
                None => FetchNeuronDataByIdResponse::NeuronDoesNotExist,
            }
        }
        Err(e) => {
            debug!(
                "Error fetching neuron with id : {:?}, error code : {:?}, message : {:?}",
                neuron_id,
                e.0,
                e.1
            );
            FetchNeuronDataByIdResponse::InternalError(e.1)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AuthenticateByHotkeyResponse {
    NeuronHotKeyAbsent,
    Ok(bool),
    NeuronHotKeyInvalid,
}

pub fn authenticate_by_hotkey(
    neuron_data: &Neuron,
    caller: &Principal
) -> AuthenticateByHotkeyResponse {
    // first is always the nns owner principal so if less than or equal to 1 then no hotkeys have been added.
    if neuron_data.permissions.len() <= 1 {
        return AuthenticateByHotkeyResponse::NeuronHotKeyAbsent;
    }

    // Check if any of the permission principals contain an entry that matches the caller principal
    let matching_caller_hotkey = neuron_data.permissions
        .iter()
        .skip(1)
        .filter(|permission| permission.principal.as_ref() == Some(caller))
        .count();

    if matching_caller_hotkey >= 1 {
        AuthenticateByHotkeyResponse::Ok(true)
    } else {
        AuthenticateByHotkeyResponse::NeuronHotKeyInvalid
    }
}

#[cfg(test)]
mod tests {
    use candid::Principal;
    use sns_governance_canister::types::{ Neuron, NeuronId, NeuronPermission };

    use crate::state::{ init_state, RuntimeState };
    use crate::utils::AuthenticateByHotkeyResponse;
    use super::authenticate_by_hotkey;

    fn init_runtime_state() {
        init_state(RuntimeState::default());
    }

    #[test]
    fn test_authenticate_by_hotkey_with_correct_data() {
        let neuron_id = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let caller = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let sns_neuron_owner_id = Principal::from_text("tr3th-kiaaa-aaaaq-aab6q-cai").unwrap();

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());

        neuron.permissions.push(NeuronPermission {
            principal: Some(sns_neuron_owner_id.clone()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });
        neuron.permissions.push(NeuronPermission {
            principal: Some(caller.clone()),
            permission_type: vec![3, 4],
        });

        let result = authenticate_by_hotkey(&neuron, &caller);

        assert_eq!(result, AuthenticateByHotkeyResponse::Ok(true));
    }

    #[test]
    fn test_authenticate_by_hotkey_with_no_hotkeys() {
        let neuron_id = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let caller = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let sns_neuron_owner_id = Principal::from_text("tr3th-kiaaa-aaaaq-aab6q-cai").unwrap();

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());

        neuron.permissions.push(NeuronPermission {
            principal: Some(sns_neuron_owner_id.clone()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });

        let result = authenticate_by_hotkey(&neuron, &caller);

        assert_eq!(result, AuthenticateByHotkeyResponse::NeuronHotKeyAbsent)
    }

    #[test]
    fn test_authenticate_by_hotkey_with_invalid_hotkey() {
        let neuron_id = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let caller = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let sns_neuron_owner_id = Principal::from_text("tr3th-kiaaa-aaaaq-aab6q-cai").unwrap();

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());

        neuron.permissions.push(NeuronPermission {
            principal: Some(sns_neuron_owner_id.clone()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });
        neuron.permissions.push(NeuronPermission {
            principal: Some(Principal::from_text("tyyy3-4aaaa-aaaaq-aab7a-cai").unwrap()),
            permission_type: vec![3, 4],
        });

        let result = authenticate_by_hotkey(&neuron, &caller);

        assert_eq!(result, AuthenticateByHotkeyResponse::NeuronHotKeyInvalid)
    }
}
