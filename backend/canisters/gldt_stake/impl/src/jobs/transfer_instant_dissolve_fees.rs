use bity_ic_canister_time::{run_interval, WEEK_IN_MS};
use candid::Nat;
use gldt_stake_common::accounts::USER_STAKES_POOL;
use gldt_stake_common::{accounts::INSTANT_DISSOLVEMENT_FEE_ACCOUNT, ledgers::GLDT_TX_FEE};
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use std::time::Duration;
use tracing::{error, info};
use utils::env::Environment;

use crate::state::{mutate_state, read_state};

pub fn start_job() {
    run_interval(
        Duration::from_millis(WEEK_IN_MS),
        transfer_instant_dissolve_fees_job,
    );
}

pub fn transfer_instant_dissolve_fees_job() {
    ic_cdk::futures::spawn(transfer_instant_dissolve_fees_impl())
}

async fn transfer_instant_dissolve_fees_impl() {
    info!("TRANSFER INSTANT DISSOLVE FEES :: start");
    let fees_available = read_state(|s| s.data.stake_system.pending_fee_transfer_amount.clone());
    if fees_available <= GLDT_TX_FEE {
        info!("TRANSFER INSTANT DISSOLVE FEES :: Not enough GLDT early withdraw fees to transfer at this moment");
        return;
    }
    let amount_to_transfer = fees_available.clone() - Nat::from(GLDT_TX_FEE);
    match transfer_to_fee_account(amount_to_transfer.clone()).await {
        Ok(_) => {
            mutate_state(|s| s.data.stake_system.pending_fee_transfer_amount -= fees_available);
            info!(
                "TRANSFER INSTANT DISSOLVE FEES :: {amount_to_transfer:?} transferred to fee account"
            );
        }
        Err(e) => {
            error!("TRANSFER INSTANT DISSOLVE FEES :: failed to transfer to fee account {e:?}")
        }
    }
    info!("TRANSFER INSTANT DISSOLVE FEES :: finished");
}

async fn transfer_to_fee_account(amount_for_early_withdraw: Nat) -> Result<(), String> {
    let gldt_ledger = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());

    match icrc1_transfer(
        gldt_ledger,
        &TransferArg {
            from_subaccount: Some(USER_STAKES_POOL),
            to: Account {
                owner: this_canister_id,
                subaccount: Some(INSTANT_DISSOLVEMENT_FEE_ACCOUNT),
            },
            fee: None,
            created_at_time: None,
            memo: None,
            amount: amount_for_early_withdraw,
        },
    )
    .await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(format!("{e:?}")),
        Err(e) => Err(format!("{e:?}")),
    }
}
