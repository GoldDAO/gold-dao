use bity_ic_canister_time::{run_interval, HOUR_IN_MS};
use candid::{Nat, Principal};
use gldt_swap_common::gldt::{GLDT_LEDGER_FEE_ACCOUNT, GLDT_SWAP_FEE_ACCOUNT, GLDT_TX_FEE};
use icrc_ledger_canister_c2c_client::{icrc1_balance_of, icrc1_transfer};
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use std::collections::HashSet;
use std::time::Duration;
use tracing::{error, info};
use utils::env::Environment;

use crate::state::read_state;

pub fn start_job() {
    run_interval(
        Duration::from_millis(HOUR_IN_MS * 12),
        spawn_transfer_fees_job,
    );
}

pub fn spawn_transfer_fees_job() {
    let _span = tracing::info_span!("TRANSFER_FEES_FOR_BUYBACK_BURN").entered();
    info!("start");
    ic_cdk::futures::spawn(transfer_fees_job_impl());
    info!("finish");
}

async fn transfer_fees_job_impl() {
    if read_state(|s| s.get_is_migrating()) {
        return;
    }

    let buy_back_and_burn_canister = match read_state(|s| s.data.buyback_burn_canister) {
        Some(canister) => canister,
        None => {
            info!("No buy back and burn canister ID set, exiting early");
            return;
        }
    };

    let this_canister_id = read_state(|s| s.env.canister_id());

    // Collect all unique ledger ids from swap configs
    let ledgers: HashSet<_> = read_state(|s| s.data.swap_configs.unique_ledger_ids());

    let fee_accounts = vec![
        (
            "SWAP_FEE_ACCOUNT",
            Account {
                owner: this_canister_id,
                subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
            },
        ),
        (
            "LEDGER_FEE_ACCOUNT",
            Account {
                owner: this_canister_id,
                subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
            },
        ),
    ];

    // Iterate over all fee accounts and ledgers
    for (name, account) in fee_accounts {
        let mut futures = Vec::new();

        for ledger_canister in &ledgers {
            let from_account = account;
            let to_account = buy_back_and_burn_canister;
            let ledger_id = *ledger_canister;
            futures.push(async move {
                match icrc1_balance_of(ledger_id, from_account).await {
                    Ok(balance) if balance > 10_000_000_000_u64 => {
                        match transfer_to_buyback_and_burn(
                            ledger_id,
                            from_account,
                            balance.clone(),
                            to_account,
                        )
                        .await
                        {
                            Ok(_) => info!(
                                "{} :: transferred {} tokens from ledger {} successfully",
                                name, balance, ledger_id
                            ),
                            Err(e) => error!(
                                "{} :: failed to transfer {} tokens from ledger {}: {:?}",
                                name, balance, ledger_id, e
                            ),
                        }
                    }
                    Ok(balance) => {
                        info!(
                            "{} :: ledger {} balance {} is below threshold, skipping",
                            name, ledger_id, balance
                        );
                    }
                    Err(e) => {
                        error!(
                            "{} :: failed to get balance from ledger {}: {:?}",
                            name, ledger_id, e
                        );
                    }
                }
            });
        }

        // Execute all transfers in parallel for this account
        futures::future::join_all(futures).await;
    }
}

async fn transfer_to_buyback_and_burn(
    ledger_id: Principal,
    from_account: Account,
    mut amount: Nat,
    to_account: Account,
) -> Result<(), String> {
    // Subtract transaction fee if applicable
    amount = if amount > GLDT_TX_FEE {
        amount - Nat::from(GLDT_TX_FEE)
    } else {
        Nat::from(0u64)
    };

    match icrc1_transfer(
        ledger_id,
        &TransferArg {
            from_subaccount: from_account.subaccount,
            to: to_account,
            fee: None,
            created_at_time: None,
            memo: None,
            amount,
        },
    )
    .await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(format!("{e:?}")),
        Err(e) => Err(format!("{e:?}")),
    }
}
