use ::types::{CanisterId, SnsNeuronId};
use bity_ic_canister_client::generate_candid_c2c_call;
use ic_cdk::api::call::{CallResult, RejectionCode};
use sns_governance_canister::types::manage_neuron::configure::Operation;
use sns_governance_canister::types::manage_neuron::{Command, Configure};
use sns_governance_canister::types::{manage_neuron_response, ManageNeuron};
use sns_governance_canister::*;

// Queries
generate_candid_c2c_call!(get_metadata);
generate_candid_c2c_call!(get_nervous_system_parameters);
generate_candid_c2c_call!(list_neurons);
generate_candid_c2c_call!(list_proposals);
generate_candid_c2c_call!(get_neuron);

// Updates
generate_candid_c2c_call!(manage_neuron);

pub async fn configure_neuron(
    governance_canister_id: CanisterId,
    neuron_id: SnsNeuronId,
    operation: Operation,
) -> ic_cdk::call::CallResult<()> {
    let args = ManageNeuron {
        subaccount: neuron_id.to_vec(),
        command: Some(Command::Configure(Configure {
            operation: Some(operation),
        })),
    };

    let response = manage_neuron(governance_canister_id, &args)
        .await
        .map_err(|e| {
            ::ic_cdk::call::Error::CallRejected(::ic_cdk::call::CallRejected::with_rejection(
                0,
                format!("{e:?}"),
            ))
        })?;

    match response.command.unwrap() {
        manage_neuron_response::Command::Configure(_) => Ok(()),
        manage_neuron_response::Command::Error(e) => Err(ic_cdk::call::Error::CallRejected(
            ic_cdk::call::CallRejected::with_rejection(0, e.error_message),
        )),
        _ => unreachable!(),
    }
}
