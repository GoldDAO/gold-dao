use crate::{
    guards::caller_is_governance_principal, model::neuron_system::sync_neurons, state::read_state,
};
use candid::Nat;
use canister_tracing_macros::trace;
use gldt_stake_api_canister::create_neuron::CreateNeuronError;
pub use gldt_stake_api_canister::create_neuron::{
    Args as StakeSnsNeuronArgs, Response as StakeSnsNeuronResponse,
};

use gldt_stake_common::{accounts::NEURON_CREATION_POOL, ledgers::GLD_GOV_TX_FEE};
use ic_cdk::{caller, query, update};
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use ledger_utils::compute_neuron_staking_subaccount_bytes;
use sns_governance_canister::types::{
    manage_neuron::{
        claim_or_refresh::{By, MemoAndController},
        ClaimOrRefresh, Command,
    },
    manage_neuron_response, ManageNeuron,
};
use tracing::error;
use utils::rand::generate_rand_nonce;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn stake_sns_neuron_validate(args: StakeSnsNeuronArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn create_neuron(args: StakeSnsNeuronArgs) -> StakeSnsNeuronResponse {
    create_neuron_impl(args.amount).await
}

async fn create_neuron_impl(amount: u64) -> Result<Vec<u8>, CreateNeuronError> {
    let nonce = generate_rand_nonce()
        .await
        .map_err(|e| CreateNeuronError::InternalError(e))?;
    let this_canister_id = caller();

    let (sns_governance_canister, gld_ledger_id) = read_state(|s| {
        (
            s.data.goldao_sns_governance_canister_id,
            s.data.goldao_ledger_id,
        )
    });

    let subaccount = compute_neuron_staking_subaccount_bytes(this_canister_id, nonce);

    match icrc_ledger_canister_c2c_client::icrc1_transfer(
        gld_ledger_id,
        &(TransferArg {
            from_subaccount: Some(NEURON_CREATION_POOL),
            to: Account {
                owner: sns_governance_canister,
                subaccount: Some(subaccount),
            },
            fee: Some(Nat::from(GLD_GOV_TX_FEE)),
            created_at_time: None,
            memo: Some(nonce.into()),
            amount: amount.into(),
        }),
    )
    .await
    {
        Ok(Ok(_)) => {}
        Ok(Err(error)) => {
            error!("Transfer error: {:?}", error);
            return Err(CreateNeuronError::TransferError(format!("{error:?}")));
        }
        Err(error) => {
            error!("Transfer error: {:?}", error);
            return Err(CreateNeuronError::TransferError(format!("{error:?}")));
        }
    }

    match sns_governance_canister_c2c_client::manage_neuron(
        sns_governance_canister,
        &(ManageNeuron {
            subaccount: vec![],
            command: Some(Command::ClaimOrRefresh(ClaimOrRefresh {
                by: Some(By::MemoAndController(MemoAndController {
                    controller: Some(this_canister_id),
                    memo: nonce,
                })),
            })),
        }),
    )
    .await
    {
        Ok(response) => match response.command {
            Some(manage_neuron_response::Command::ClaimOrRefresh(c)) => {
                match c.refreshed_neuron_id {
                    Some(neuron_id) => {
                        ic_cdk::spawn(async {
                            let _ = sync_neurons().await;
                        });
                        Ok(neuron_id.id)
                    }
                    None => Err(CreateNeuronError::InternalError(format!(
                        "create_neuron error - newly created neuron had no ID"
                    ))),
                }
            }
            response => Err(CreateNeuronError::InternalError(format!(
                "Error while executing manage_neuron: {:?}",
                response
            ))),
        },
        Err(error) => Err(CreateNeuronError::InternalError(format!(
            "network error - {error:?}"
        ))),
    }

    // sync the neurons
}
