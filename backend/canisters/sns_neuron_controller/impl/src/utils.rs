use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::{
    account::{Account, Subaccount},
    transfer::TransferArg,
};
use sns_governance_canister::types::{Neuron, NeuronId};
use tracing::debug;
use tracing::error;

use sns_governance_canister::types::get_neuron_response::Result::{
    Error as NeuronErrorResponse, Neuron as NeuronResponse,
};

pub async fn transfer_token(
    from_sub_account: Subaccount,
    to_account: Account,
    ledger_id: Principal,
    amount: Nat,
) -> Result<(), String> {
    match icrc_ledger_canister_c2c_client::icrc1_transfer(
        ledger_id,
        &(TransferArg {
            from_subaccount: Some(from_sub_account),
            to: to_account,
            fee: None,
            created_at_time: None,
            amount: amount,
            memo: None,
        }),
    )
    .await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(error)) => Err(format!("Transfer error: {error:?}")),
        Err(error) => Err(format!("Network error: {error:?}")),
    }
}

async fn get_ledger_balance_of(account: Account, ledger_canister_id: Principal) -> Nat {
    match icrc_ledger_canister_c2c_client::icrc1_balance_of(ledger_canister_id, &account).await {
        Ok(response) => response,
        Err(err) => {
            let message = format!("{err:?}");
            let principal_as_text = account.owner.to_text();
            error!(
                ?message,
                "There was an error while getting balance of {principal_as_text}."
            );
            Nat::from(0u64)
        }
    }
}

pub enum FetchNeuronDataByIdResponse {
    NeuronDoesNotExist,
    Ok(Neuron),
    InternalError(String),
}

pub async fn fetch_neuron_data_by_id(
    neuron_id: &NeuronId,
    canister_id: Principal,
) -> FetchNeuronDataByIdResponse {
    let args = sns_governance_canister::get_neuron::Args {
        neuron_id: Some(neuron_id.clone()),
    };
    match sns_governance_canister_c2c_client::get_neuron(canister_id, &args).await {
        Ok(neuron_data) => match neuron_data.result {
            Some(neuron) => match neuron {
                NeuronResponse(n) => FetchNeuronDataByIdResponse::Ok(n),
                NeuronErrorResponse(_) => FetchNeuronDataByIdResponse::NeuronDoesNotExist,
            },
            None => FetchNeuronDataByIdResponse::NeuronDoesNotExist,
        },
        Err(e) => {
            debug!(
                "Error fetching neuron with id : {:?}, error code : {:?}, message : {:?}",
                neuron_id, e.0, e.1
            );
            FetchNeuronDataByIdResponse::InternalError(e.1)
        }
    }
}

pub async fn fetch_neuron_reward_balance(
    ledger_canister_id: Principal,
    ogy_sns_rewards_canister_id: Principal,
    neuron_id: &NeuronId,
) -> Nat {
    match icrc_ledger_canister_c2c_client::icrc1_balance_of(
        ledger_canister_id,
        &(Account {
            owner: ogy_sns_rewards_canister_id,
            subaccount: Some(neuron_id.into()),
        }),
    )
    .await
    {
        Ok(t) => t,
        Err(e) => {
            error!(
                "Failed to fetch token balance of ledger canister id {} with ERROR : {:?}",
                ledger_canister_id, e
            );
            Nat::from(0u64)
        }
    }
}
