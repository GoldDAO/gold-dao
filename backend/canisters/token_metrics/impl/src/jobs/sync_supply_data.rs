use futures::future::join_all;
use icrc_ledger_types::icrc1::account::Account;
use tracing::error;
use types::Milliseconds;

use crate::{ consts::TEAM_PRINCIPALS, state::{ mutate_state, read_state } };

const SYNC_SUPPLY_DATA_INTERVAL: Milliseconds = 3_600 * 1_000;

pub fn run() {
    ic_cdk::spawn(sync_supply_data())
}

pub async fn sync_supply_data() {
    let ledger_canister_id = read_state(|state| state.data.sns_ledger_canister);

    let total_foundation_balance = get_total_ledger_balance_of_accounts(
        TEAM_PRINCIPALS.to_vec()
    ).await;

    match icrc_ledger_canister_c2c_client::icrc1_total_supply(ledger_canister_id).await {
        Ok(total_supply) => {
            let total_locked = read_state(|state| state.data.all_gov_stats.total_locked.clone());
            let circulating_supply = total_supply.clone() - total_locked - total_foundation_balance;

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

async fn get_total_ledger_balance_of_accounts(accounts: Vec<Account>) -> u64 {
    let getter_futures: Vec<_> = accounts
        .iter()
        .map(|account| {
            let getter_future = get_ledger_balance_of(*account);
            getter_future
        })
        .collect();
    let results = join_all(getter_futures).await;
    results.iter().sum()
}

async fn get_ledger_balance_of(account: Account) -> u64 {
    let ledger_canister_id = read_state(|state| state.data.sns_ledger_canister);
    match icrc_ledger_canister_c2c_client::icrc1_balance_of(ledger_canister_id, &account).await {
        Ok(response) => response.0.try_into().unwrap(),
        Err(err) => {
            let message = format!("{err:?}");
            let principal_as_text = account.owner.to_text();
            error!(
                ?message,
                "There was an error while getting balance of {principal_as_text}."
            );
            0
        }
    }
}
