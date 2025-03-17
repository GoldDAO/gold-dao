use candid::Nat;
use canister_time::{run_interval, HOUR_IN_MS};
use gldt_stake_common::{accounts::EARLY_UNSTAKE_FEE_ACCOUNT, ledgers::GLDT_TX_FEE};
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use std::time::Duration;
use tracing::{error, info};
use utils::env::Environment;

use crate::state::{mutate_state, read_state};

pub fn start_job() {
    run_interval(
        Duration::from_millis(HOUR_IN_MS),
        transfer_early_unstake_fees_job,
    );
}

pub fn transfer_early_unstake_fees_job() {
    ic_cdk::spawn(transfer_early_unstake_fees_impl())
}

async fn transfer_early_unstake_fees_impl() {
    info!("TRANSFER EARLY UNSTAKE FEES :: start");
    let fees_available = read_state(|s| s.data.stake_system.pending_fee_transfer_amount.clone());
    if fees_available <= Nat::from(GLDT_TX_FEE) {
        info!("TRANSFER EARLY UNSTAKE FEES :: Not enough GLDT early unstake fees to transfer at this moment");
        return;
    }
    let amount_to_transfer = fees_available.clone() - Nat::from(GLDT_TX_FEE);
    match transfer_to_fee_account(amount_to_transfer.clone()).await {
        Ok(_) => {
            mutate_state(|s| s.data.stake_system.pending_fee_transfer_amount -= fees_available);
            info!(
                "TRANSFER EARLY UNSTAKE FEES :: {amount_to_transfer:?} transferred to fee account"
            );
        }
        Err(e) => {
            error!("TRANSFER EARLY UNSTAKE FEES :: failed to transfer to fee account {e:?}")
        }
    }
    info!("TRANSFER EARLY UNSTAKE FEES :: finished");
}

async fn transfer_to_fee_account(amount_for_early_unstake: Nat) -> Result<(), String> {
    let gldt_ledger = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());

    match icrc1_transfer(
        gldt_ledger,
        &TransferArg {
            from_subaccount: None,
            to: Account {
                owner: this_canister_id,
                subaccount: Some(EARLY_UNSTAKE_FEE_ACCOUNT),
            },
            fee: None,
            created_at_time: None,
            memo: None,
            amount: amount_for_early_unstake,
        },
    )
    .await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(format!("{e:?}")),
        Err(e) => Err(format!("{e:?}")),
    }
}
