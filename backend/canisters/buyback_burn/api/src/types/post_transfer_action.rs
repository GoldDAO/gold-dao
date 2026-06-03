use candid::CandidType;
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::manage_neuron::claim_or_refresh::By;
use sns_governance_canister::types::manage_neuron::{ClaimOrRefresh, Command};
use sns_governance_canister::types::ManageNeuron;
use tracing::info;
use types::CanisterId;

// An optional action to perform after tokens have been transferred to the destination account.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum PostTransferAction {
    SnsClaimOrRefresh {
        governance_canister_id: CanisterId,
        // The neuron's subaccount (= its ID) on the SNS governance canister.
        neuron_subaccount: Subaccount,
    },
}

impl PostTransferAction {
    pub async fn execute_post_transfer_action(&self) -> Result<(), String> {
        match self {
            PostTransferAction::SnsClaimOrRefresh {
                governance_canister_id,
                neuron_subaccount,
            } => {
                match sns_governance_canister_c2c_client::manage_neuron(
                    *governance_canister_id,
                    &ManageNeuron {
                        subaccount: neuron_subaccount.to_vec(),
                        command: Some(Command::ClaimOrRefresh(ClaimOrRefresh {
                            by: Some(By::NeuronId(sns_governance_canister::types::Empty {})),
                        })),
                    },
                )
                .await
                {
                    Ok(response) => {
                        info!("SNS ClaimOrRefresh response: {:?}", response);
                        Ok(())
                    }
                    Err(e) => Err(format!("SNS ClaimOrRefresh call error: {:?}", e)),
                }
            }
        }
    }
}
