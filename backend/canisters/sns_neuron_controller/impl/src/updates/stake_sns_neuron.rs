use crate::{
    guards::caller_is_governance_principal,
    state::{read_state, RuntimeState},
};
use candid::Principal;
use canister_tracing_macros::trace;
use ic_cdk::{query, update};
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use ledger_utils::compute_neuron_staking_subaccount_bytes;
use sns_governance_canister::types::{
    manage_neuron::{
        claim_or_refresh::{By, MemoAndController},
        ClaimOrRefresh, Command,
    },
    manage_neuron_response, ManageNeuron,
};
pub use sns_neuron_controller_api_canister::stake_sns_neuron::Args as StakeSnsNeuronArgs;
pub use sns_neuron_controller_api_canister::stake_sns_neuron::Response as StakeSnsNeuronResponse;
use tracing::error;
use types::CanisterId;
use utils::{env::Environment, rand::generate_rand_nonce};

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn stake_sns_neuron_validate(args: StakeSnsNeuronArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn stake_ogy_neuron(args: StakeSnsNeuronArgs) -> StakeSnsNeuronResponse {
    match stake_ogy_neuron_impl(args.amount).await {
        Ok(neuron_id) => StakeSnsNeuronResponse::Success(neuron_id),
        Err(error) => {
            error!(error);
            StakeSnsNeuronResponse::InternalError(error)
        }
    }
}

async fn stake_ogy_neuron_impl(amount: u64) -> Result<Vec<u8>, String> {
    let nonce = generate_rand_nonce().await?;

    let PrepareResult {
        ogy_governance_canister_id,
        ogy_ledger_canister_id,
        principal,
    } = read_state(prepare)?;

    let subaccount = compute_neuron_staking_subaccount_bytes(principal, nonce);

    match icrc_ledger_canister_c2c_client::icrc1_transfer(
        ogy_ledger_canister_id,
        &(TransferArg {
            from_subaccount: None,
            to: Account {
                owner: ogy_governance_canister_id,
                subaccount: Some(subaccount),
            },
            fee: Some(
                icrc_ledger_canister_c2c_client::icrc1_fee(ogy_ledger_canister_id)
                    .await
                    .unwrap(),
            ),
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
            return Err(format!("Transfer error: {error:?}"));
        }
        Err(error) => {
            error!("Transfer error: {:?}", error);
            return Err(format!("Network error: {error:?}"));
        }
    }

    match sns_governance_canister_c2c_client::manage_neuron(
        ogy_governance_canister_id,
        &(ManageNeuron {
            // TODO: fix
            subaccount: vec![],
            command: Some(Command::ClaimOrRefresh(ClaimOrRefresh {
                by: Some(By::MemoAndController(MemoAndController {
                    controller: Some(principal),
                    memo: nonce,
                })),
            })),
        }),
    )
    .await
    {
        Ok(response) => match response.command {
            Some(manage_neuron_response::Command::ClaimOrRefresh(c)) => {
                let neuron_id = c.refreshed_neuron_id.unwrap().id;
                Ok(neuron_id)
            }
            response => {
                error!("Error while executing manage_neuron: {:?}", response);
                Err("manage_neuron error".to_string())
            }
        },
        Err(error) => Err(format!("Network error: {error:?}")),
    }
}

struct PrepareResult {
    ogy_governance_canister_id: CanisterId,
    ogy_ledger_canister_id: CanisterId,
    principal: Principal,
}

fn prepare(state: &RuntimeState) -> Result<PrepareResult, String> {
    Ok(PrepareResult {
        ogy_governance_canister_id: state
            .data
            .neuron_managers
            .ogy
            .ogy_sns_governance_canister_id,
        ogy_ledger_canister_id: state.data.neuron_managers.ogy.ogy_sns_ledger_canister_id,
        principal: state.env.canister_id(),
    })
}
