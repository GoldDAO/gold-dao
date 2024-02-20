use crate::guards::caller_is_governance_principal;
use crate::state::{ read_state, RuntimeState };
use candid::{ CandidType, Principal };
use canister_tracing_macros::trace;
use ic_cdk::update;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use ledger_utils::compute_neuron_staking_subaccount_bytes;
use nns_governance_canister::types::manage_neuron::claim_or_refresh::{ By, MemoAndController };
use nns_governance_canister::types::manage_neuron::{ ClaimOrRefresh, Command };
use nns_governance_canister::types::{ manage_neuron_response, ManageNeuron };
use serde::{ Deserialize, Serialize };
use tracing::{ error, info };
use types::CanisterId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum StakeNnsNeuronResponse {
    Success(u64),
    InternalError(String),
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn stake_nns_neuron() -> StakeNnsNeuronResponse {
    let nonce: u64;
    if let Ok((random_bytes,)) = ic_cdk::api::management_canister::main::raw_rand().await {
        nonce = u64::from_be_bytes(random_bytes.try_into().unwrap());
    } else {
        return StakeNnsNeuronResponse::InternalError("Error initialising nonce.".to_string());
    }

    let PrepareResult { nns_governance_canister_id, icp_ledger_canister_id, principal } =
        read_state(prepare);

    let subaccount = compute_neuron_staking_subaccount_bytes(principal, nonce);

    match
        icrc_ledger_canister_c2c_client::icrc1_transfer(
            icp_ledger_canister_id,
            &(TransferArg {
                from_subaccount: None,
                to: Account {
                    owner: nns_governance_canister_id,
                    subaccount: Some(subaccount),
                },
                fee: Some((10_000u32).into()),
                created_at_time: None,
                memo: Some(nonce.into()),
                amount: (100_000_000u32).into(), // 1 ICP
            })
        ).await
    {
        Ok(Ok(_)) => {}
        Ok(Err(error)) => {
            error!(?error, "Transfer error");
            return StakeNnsNeuronResponse::InternalError(format!("{error:?}"));
        }
        Err(error) => {
            return StakeNnsNeuronResponse::InternalError(format!("{error:?}"));
        }
    }

    match
        nns_governance_canister_c2c_client::manage_neuron(
            nns_governance_canister_id,
            &(ManageNeuron {
                id: None,
                neuron_id_or_subaccount: None,
                command: Some(
                    Command::ClaimOrRefresh(ClaimOrRefresh {
                        by: Some(
                            By::MemoAndController(MemoAndController {
                                controller: Some(principal),
                                memo: nonce,
                            })
                        ),
                    })
                ),
            })
        ).await
    {
        Ok(response) =>
            match response.command {
                Some(manage_neuron_response::Command::ClaimOrRefresh(c)) => {
                    let neuron_id = c.refreshed_neuron_id.unwrap().id;
                    info!(neuron_id, "Staked new NNS neuron");
                    StakeNnsNeuronResponse::Success(neuron_id)
                }
                response => {
                    error!(?response, "Governance error");
                    StakeNnsNeuronResponse::InternalError(format!("{response:?}"))
                }
            }
        Err(error) => StakeNnsNeuronResponse::InternalError(format!("{error:?}")),
    }
}

struct PrepareResult {
    nns_governance_canister_id: CanisterId,
    icp_ledger_canister_id: CanisterId,
    principal: Principal,
}

fn prepare(state: &RuntimeState) -> PrepareResult {
    PrepareResult {
        nns_governance_canister_id: state.data.nns_governance_canister_id,
        icp_ledger_canister_id: state.data.icp_ledger_canister_id,
        principal: state.data.get_principal(),
    }
}
