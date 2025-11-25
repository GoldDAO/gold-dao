use crate::model::nft_manager::NftCollector;
use crate::state::{mutate_state, read_state};
use bity_ic_canister_time::{run_interval, timestamp_nanos};
use candid::{Nat, Principal};
use futures::future::join_all;
use gldt_swap_common::{gldt::GLDT_LEDGER_FEE_ACCOUNT, swap::MANAGE_GLDT_SUPPLY_INTERVAL};
use ic_stable_structures::Storable;
use icrc_ledger_canister_c2c_client::{icrc1_total_supply, icrc1_transfer};
use icrc_ledger_types::icrc1::{
    account::Account,
    transfer::{Memo, TransferArg},
};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{error, info};
use utils::env::Environment;

pub fn start_job() {
    run_interval(
        Duration::from_millis(MANAGE_GLDT_SUPPLY_INTERVAL),
        spawn_gldt_supply_balancer,
    );
}

pub fn spawn_gldt_supply_balancer() {
    let _span = tracing::info_span!("MANAGE_GLDT_SUPPLY").entered();
    info!("start");

    if read_state(|s| s.data.is_gldt_supply_balancer_running) {
        info!("Supply balancer already running, exiting");
        return;
    }

    ic_cdk::futures::spawn(balance_supply());

    info!("finish");
}

async fn balance_supply() {
    let _span = tracing::info_span!("BALANCE_SUPPLY").entered();
    info!("Balancing GLDT supply...");

    mutate_state(|s| {
        s.data.is_gldt_supply_balancer_running = true;
    });
    info!("Marked supply balancer as running");

    let swap_configs = read_state(|s| s.data.swap_configs.clone());
    let owned_nfts_batch = swap_configs
        .collect_nfts(ic_cdk::api::canister_self())
        .await
        .expect("Failed to fetch NFTs owned by canister");
    info!(
        "Collected owned NFTs batch: {} canisters",
        owned_nfts_batch.grouped.len()
    );

    let expected_amounts: HashMap<Principal, Nat> =
        match owned_nfts_batch.calculate_token_equivalent() {
            Ok(amounts) => {
                info!(
                    "Calculated expected token amounts for {} ledgers",
                    amounts.len()
                );
                amounts
            }
            Err(err) => {
                error!("Failed to calculate expected token amounts: {:?}", err);
                return;
            }
        };

    let real_supplies: Vec<(Principal, Nat)> = join_all(expected_amounts.keys().cloned().map(
        |ledger_principal| async move {
            match icrc1_total_supply(ledger_principal).await {
                Ok(supply) => {
                    info!(
                        "Fetched real supply for ledger {}: {}",
                        ledger_principal, supply
                    );
                    Some((ledger_principal, supply))
                }
                Err(err) => {
                    error!(
                        "Failed to fetch total supply for ledger {}: {:?}",
                        ledger_principal, err
                    );
                    None
                }
            }
        },
    ))
    .await
    .into_iter()
    .flatten()
    .collect();

    info!("Preparing mint operations for ledgers where real supply < expected");

    let mint_futures = real_supplies
        .into_iter()
        .filter_map(
            |(ledger, real_supply)| match expected_amounts.get(&ledger) {
                Some(expected) if *expected > real_supply => {
                    let diff = expected.clone() - real_supply.clone();
                    info!(
                        "Ledger {} needs minting: expected {}, real {}, diff {}",
                        ledger, expected, real_supply, diff
                    );
                    Some(mint_to_fee_account(ledger, diff))
                }
                Some(_) => {
                    info!("Ledger {} has sufficient supply, skipping mint", ledger);
                    None
                }
                None => {
                    error!(
                        "Skipping mint for ledger {}: expected amount not calculated",
                        ledger
                    );
                    None
                }
            },
        );

    join_all(mint_futures).await;

    mutate_state(|s| {
        s.data.is_gldt_supply_balancer_running = false;
    });
    info!("GLDT supply balancer finished, marked as not running");
}

async fn mint_to_fee_account(ledger_canister_id: Principal, amount: Nat) {
    let _span = tracing::info_span!("MINT_TO_FEE_ACCOUNT").entered();

    let this_canister_id = read_state(|s| s.env.canister_id());
    // FIXME: think on the memo, I would rather make random nonce
    let memo_str = format!("AUTO_BALANCE {:?}", ic_cdk::api::time());

    let args = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: this_canister_id,
            subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
        },
        amount: amount.clone(),
        fee: None,
        created_at_time: Some(timestamp_nanos()),
        memo: Some(Memo::from(memo_str.to_bytes().to_vec())),
    };

    info!(
        "Minting {} tokens to fee account on ledger {}",
        amount, ledger_canister_id
    );

    match icrc1_transfer(ledger_canister_id, &args).await {
        Ok(res) => match res {
            Ok(_) => {
                info!(
                    "Successfully minted {} tokens to fee account on ledger {}",
                    amount, ledger_canister_id
                );
            }
            Err(e) => {
                error!(
                    "Mint failed for ledger {}: {:?}, amount: {}",
                    ledger_canister_id, e, amount
                );
            }
        },
        Err(e) => {
            error!(
                "Mint call failed for ledger {}: {:?}, amount: {}",
                ledger_canister_id, e, amount
            );
        }
    }
}
