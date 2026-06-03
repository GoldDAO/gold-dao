use crate::state::read_state;
use crate::utils::get_token_balance;
use bity_ic_canister_time::NANOS_PER_MILLISECOND;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use nns_governance_canister::types::manage_neuron::claim_or_refresh::By;
use nns_governance_canister::types::manage_neuron::{ClaimOrRefresh, Command};
use nns_governance_canister::types::ManageNeuron;
use nns_governance_canister::types::NeuronId;
use tracing::{error, info};
use types::TokenSymbol;
use utils::consts::NNS_GOVERNANCE_CANISTER_ID;
use utils::env::Environment;
use utils::numeric::Rate;

pub(crate) async fn stake_icp() {
    let config = read_state(|s| s.data.stake_icp_config.clone());
    let Some(config) = config else { return };

    let icp_token_info = TokenSymbol::ICP.get_prod_token_info();

    let available = match get_token_balance(icp_token_info.ledger_id, None).await {
        Ok(a) => a,
        Err(e) => {
            error!("stake_icp: failed to get ICP balance: {}", e);
            return;
        }
    };

    let rate = Rate::from_scaled(config.rate_per_interval);
    let amount = rate.apply_to(&available);
    let fee: candid::Nat = icp_token_info.fee.into();

    let min_amount = config.min_amount;
    if amount <= fee.clone() + min_amount.clone() {
        info!(
            "stake_icp: ICP amount ({:?}) too low to cover fee ({:?}) and minimum amount ({:?}), skipping.",
            amount, fee, min_amount
        );
        return;
    }

    let now = read_state(|s| s.env.now());

    match icrc_ledger_canister_c2c_client::icrc1_transfer(
        icp_token_info.ledger_id,
        &TransferArg {
            from_subaccount: None,
            to: config.destination_account,
            fee: Some(fee.clone()),
            created_at_time: Some(now * NANOS_PER_MILLISECOND),
            memo: None,
            amount: amount - fee,
        },
    )
    .await
    {
        Ok(Ok(_)) => {
            info!(
                "stake_icp: transferred ICP to neuron destination account, calling ClaimOrRefresh."
            );
            claim_or_refresh_nns_neuron(config.neuron_id).await;
        }
        Ok(Err(e)) => error!("stake_icp: transfer error: {:?}", e),
        Err(e) => error!("stake_icp: transfer call error: {:?}", e),
    }
}

async fn claim_or_refresh_nns_neuron(neuron_id: u64) {
    match nns_governance_canister_c2c_client::manage_neuron(
        NNS_GOVERNANCE_CANISTER_ID,
        &ManageNeuron {
            id: None,
            neuron_id_or_subaccount: Some(
                nns_governance_canister::types::manage_neuron::NeuronIdOrSubaccount::NeuronId(
                    NeuronId::from(neuron_id),
                ),
            ),
            command: Some(Command::ClaimOrRefresh(ClaimOrRefresh {
                by: Some(By::NeuronIdOrSubaccount(
                    nns_governance_canister::types::Empty {},
                )),
            })),
        },
    )
    .await
    {
        Ok(_) => info!(
            "stake_icp: NNS neuron {} refreshed successfully.",
            neuron_id
        ),
        Err(e) => error!(
            "stake_icp: ClaimOrRefresh failed for neuron {}: {:?}",
            neuron_id, e
        ),
    }
}
