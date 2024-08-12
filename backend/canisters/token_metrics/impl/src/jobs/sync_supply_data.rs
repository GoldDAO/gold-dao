use candid::Nat;
use canister_time::run_now_then_interval;
use futures::future::join_all;
use icrc_ledger_types::icrc1::account::Account;
use token_metrics_api::TEAM_PRINCIPALS;
use utils::principal::string_to_account;
use std::time::Duration;
use tracing::{ debug, error };
use types::Milliseconds;
use crate::state::{ mutate_state, read_state };

const SYNC_SUPPLY_DATA_INTERVAL: Milliseconds = 3_600 * 1_000;

pub fn _start_job_if_not_started() {
    debug!("Starting the sync supply data job...");
    run_now_then_interval(Duration::from_millis(SYNC_SUPPLY_DATA_INTERVAL), run)
}

pub fn run() {
    ic_cdk::spawn(sync_supply_data())
}

pub async fn sync_supply_data() {
    let ledger_canister_id = read_state(|state| state.data.sns_ledger_canister);

    match icrc_ledger_canister_c2c_client::icrc1_total_supply(ledger_canister_id).await {
        Ok(total_supply) => {
            let foundation_account_strings = read_state(|state|
                state.data.foundation_accounts.clone()
            );

            let mut foundation_accounts = Vec::new();
            for account_str in foundation_account_strings {
                match string_to_account(account_str) {
                    Ok(account) => {
                        foundation_accounts.push(account);
                    }
                    Err(err) => error!(err),
                }
            }

            foundation_accounts.extend_from_slice(&TEAM_PRINCIPALS);
            let total_foundation_balance =
                get_total_ledger_balance_of_accounts(foundation_accounts).await;
            let circulating_supply = total_supply.clone() - total_foundation_balance;

            mutate_state(|state| {
                state.data.supply_data.total_supply = total_supply.clone();
                state.data.supply_data.circulating_supply = circulating_supply;
            });
        }
        Err(err) => {
            let message = format!("{err:?}");
            error!(?message, "Error while getting the total supply data");
        }
    }
}

async fn get_total_ledger_balance_of_accounts(accounts: Vec<Account>) -> Nat {
    let getter_futures: Vec<_> = accounts
        .iter()
        .map(|account| {
            let getter_future = get_ledger_balance_of(*account);
            getter_future
        })
        .collect();
    let results = join_all(getter_futures).await;
    results.iter().fold(Nat::from(0u64), |acc, x| acc + x.clone())
}

async fn get_ledger_balance_of(account: Account) -> Nat {
    let ledger_canister_id = read_state(|state| state.data.sns_ledger_canister);
    match icrc_ledger_canister_c2c_client::icrc1_balance_of(ledger_canister_id, &account).await {
        Ok(response) => response,
        Err(err) => {
            let message = format!("{err:?}");
            let principal_as_text = account.owner.to_text();
            error!(
                ?message,
                "There was an error while getting balance of {principal_as_text}."
            );
            Nat::from(0u64)
        }
    }
}
