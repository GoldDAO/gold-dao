use candid::Nat;
use canister_time::{run_interval, HOUR_IN_MS};
use gldt_swap_common::gldt::{GLDT_LEDGER_FEE_ACCOUNT, GLDT_SWAP_FEE_ACCOUNT, GLDT_TX_FEE};
use icrc_ledger_canister_c2c_client::{icrc1_balance_of, icrc1_transfer};
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use std::time::Duration;
use tracing::info;
use utils::env::Environment;

use crate::state::read_state;

pub fn start_job() {
    run_interval(
        Duration::from_millis(HOUR_IN_MS * 12),
        transfer_gldt_fees_job,
    );
}

pub fn transfer_gldt_fees_job() {
    info!("TRANSFER GLDT FEES FOR BUY BACK BURN :: start");
    ic_cdk::spawn(transfer_gldt_fees_job_impl());
    info!("TRANSFER GLDT FEES FOR BUY BACK BURN :: finished");
}

async fn transfer_gldt_fees_job_impl() {
    let buy_back_and_burn_canister_id = read_state(|s| s.data.buy_back_burn_canister);

    let buy_back_and_burn_canister_id = match buy_back_and_burn_canister_id {
        Some(account) => account,
        None => {
            info!("No buy back and burn canister ID set, exiting early");
            return ();
        }
    };

    let this_canister_id = read_state(|s| s.env.canister_id());
    let _ = check_and_transfer_for_account(
        Account {
            owner: this_canister_id,
            subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
        },
        buy_back_and_burn_canister_id.clone(),
    )
    .await;
    let _ = check_and_transfer_for_account(
        Account {
            owner: this_canister_id,
            subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
        },
        buy_back_and_burn_canister_id,
    )
    .await;
}

async fn check_and_transfer_for_account(
    account: Account,
    buy_back_canister_id: Account,
) -> Result<(), ()> {
    // get the balance
    let ledger_canister_id = read_state(|s| s.data.gldt_ledger_id);
    let gldt_balance = icrc1_balance_of(ledger_canister_id, account.clone())
        .await
        .map_err(|e| {
            info!("TRANSFER GLDT FEES FOR BUY BACK BURN :: check_and_transfer_for_account {e:?}");
            ()
        })?;

    // if more than 100 GLDT then we can send the balance to the buy back and burn canister otherwise return early
    if gldt_balance < Nat::from(10_000_000_000u64) {
        info!("TRANSFER GLDT FEES FOR BUY BACK BURN :: this account does not have over 10 GLDT yet and so will not transfer");
        return Err(());
    }

    let _ = transfer_to_buy_back_and_burn_canister(account, gldt_balance, buy_back_canister_id).await.map_err(|e| {
        info!("TRANSFER GLDT FEES FOR BUY BACK BURN :: transfer_to_buy_back_and_burn_canister failed with error - {e:?}");
        ()
    })?;

    Ok(())
}

async fn transfer_to_buy_back_and_burn_canister(
    from_account: Account,
    amount: Nat,
    to_account: Account,
) -> Result<(), String> {
    let gldt_ledger = read_state(|s| s.data.gldt_ledger_id);
    let amount = amount - Nat::from(GLDT_TX_FEE);
    match icrc1_transfer(
        gldt_ledger,
        &TransferArg {
            from_subaccount: from_account.subaccount,
            to: to_account,
            fee: None,
            created_at_time: None,
            memo: None,
            amount: amount,
        },
    )
    .await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(format!("{e:?}")),
        Err(e) => Err(format!("{e:?}")),
    }
}
