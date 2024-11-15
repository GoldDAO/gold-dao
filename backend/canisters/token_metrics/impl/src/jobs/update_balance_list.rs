use crate::memory::VM;
use crate::state::{mutate_state, read_state};
use candid::Principal;
use canister_time::run_now_then_interval;
use ic_stable_structures::Vec as SVec;
use icrc_ledger_types::icrc1::account::Account;
use std::collections::BTreeMap as NormalBTreeMap;
use std::{collections::HashMap, time::Duration};
use super_stats_v3_api::account_tree::Overview as LedgerOverview;
use super_stats_v3_api::{
    stats::queries::get_account_holders::GetHoldersArgs as GetAccountHoldersArgs,
    stats::queries::get_principal_holders::GetHoldersArgs as GetPrincipalHoldersArgs,
};
use token_metrics_api::token_data::{GovernanceStats, WalletEntry, WalletOverview};
use tracing::{debug, error, info};
use types::Milliseconds;
use utils::principal::string_to_account;

// The cronjob currently disabled, after sync_governance is done
// this is called as well

// every 5 hours
const UPDATE_LEDGER_BALANCE_LIST: Milliseconds = 5 * 3_600 * 1000;

pub fn start_job() {
    info!("Starting the update ledger balance list job...");
    run_now_then_interval(Duration::from_millis(UPDATE_LEDGER_BALANCE_LIST), run);
}

pub fn run() {
    ic_cdk::spawn(update_balance_list())
}
pub async fn commit_changes() {
    let _ = ic_cdk::call::<(), ()>(
        Principal::from_text("yfjcz-3iaaa-aaaap-accjq-cai").unwrap(),
        "commit",
        (),
    )
    .await;
}

pub async fn update_balance_list() {
    info!("getting all holders from super_stats_v3");
    let (principal_holders_map, account_holders_map) = get_all_holders().await;
    commit_changes().await;
    info!("success to get all holders");

    let mut temp_wallets_list: NormalBTreeMap<Account, WalletOverview> = NormalBTreeMap::new();
    let mut temp_merged_wallets_list: NormalBTreeMap<Account, WalletOverview> =
        NormalBTreeMap::new();

    // Iterate through accounts
    for (wallet, stats) in account_holders_map.into_iter() {
        let new_stats = WalletOverview {
            ledger: stats.clone(),
            governance: GovernanceStats::default(),
            total: stats.balance.clone() as u64,
        };
        match string_to_account(wallet.clone()) {
            Ok(account) => {
                // Insert the item in the list with all accounts
                temp_wallets_list.insert(account, new_stats.clone());
            }
            Err(err) => info!(err),
        }
    }
    commit_changes().await;
    info!("done itterating through account_holders_map");
    // Iterate through principals
    for (wallet, stats) in principal_holders_map.into_iter() {
        let new_stats = WalletOverview {
            ledger: stats.clone(),
            governance: GovernanceStats::default(),
            total: stats.balance as u64,
        };
        match string_to_account(wallet.clone()) {
            Ok(account) => {
                // Update the merged list with the principal
                let merged_account_into_principal = Account {
                    owner: account.owner,
                    subaccount: None,
                };
                check_and_update_list(
                    &mut temp_merged_wallets_list,
                    merged_account_into_principal,
                    new_stats.clone(),
                );
            }
            Err(err) => info!(err),
        }
    }
    commit_changes().await;
    info!("done itterating through principal_holders_map");
    commit_changes().await;

    // Going through all governance principals and apending their stats
    // to wallets_list and merged_wallets_list
    info!("reading the state, adding gov stats");
    commit_changes().await;
    mutate_state(|state| {
        for (principal, gov_stats) in state.data.principal_gov_stats.iter() {
            let total_staked = gov_stats.clone().total_staked.0.try_into().unwrap_or(0u64);
            let new_stats = WalletOverview {
                ledger: LedgerOverview::default(),
                governance: gov_stats,
                total: total_staked,
            };
            let account = Account::from(principal);
            check_and_update_list(&mut temp_merged_wallets_list, account, new_stats.clone());
            check_and_update_list(&mut temp_wallets_list, account, new_stats.clone());
        }
    });

    let treasury_account = read_state(|state| state.data.treasury_account.clone());
    info!("mutating state with the new data from gov stats");
    mutate_state(|state| {
        // Remove the first item from the merged array, which is the governance canister
        // including all stakes, also available in each principal.governance in the list
        // and then replace its value with the value of subbaccount 32x0
        let governance_0_account = Account {
            owner: state.data.sns_governance_canister,
            subaccount: None,
        };

        match string_to_account(treasury_account.to_string()) {
            Ok(treasury_account) => match temp_wallets_list.get(&treasury_account) {
                Some(v) => {
                    temp_merged_wallets_list.insert(governance_0_account, v.clone());
                }
                None => {
                    let default_overview = WalletOverview::default();
                    temp_merged_wallets_list.insert(governance_0_account, default_overview);
                }
            },
            Err(_) => {
                let default_overview = WalletOverview::default();
                temp_merged_wallets_list.insert(governance_0_account, default_overview);
            }
        }

        sort_map_descending(
            &mut state.data.merged_wallets_list,
            &temp_merged_wallets_list,
        );
        sort_map_descending(&mut state.data.wallets_list, &temp_wallets_list);

        state.data.active_users.active_principals_count =
            count_active_users(&temp_merged_wallets_list);
        state.data.active_users.active_accounts_count = count_active_users(&temp_wallets_list);
    });
    mutate_state(|state| state.data.update_foundation_accounts_data());
    info!("update_balance_list -> done, mutated the state")
}
async fn get_all_holders() -> (
    HashMap<String, LedgerOverview>,
    HashMap<String, LedgerOverview>,
) {
    info!("getting all holders..");
    let super_stats_canister_id = read_state(|state| state.data.super_stats_canister);

    let mut principal_holders_map: HashMap<String, LedgerOverview> = HashMap::new();
    let mut account_holders_map: HashMap<String, LedgerOverview> = HashMap::new();

    let mut p_args = GetPrincipalHoldersArgs {
        offset: 0,
        limit: 100,
    };
    let mut continue_scanning_principals = true;

    while continue_scanning_principals {
        continue_scanning_principals = false;
        info!(
            "get_all_holders -> continue principals with {}",
            p_args.offset
        );
        match super_stats_v3_c2c_client::get_principal_holders(super_stats_canister_id, &p_args)
            .await
        {
            Ok(principal_holders) => {
                for response in principal_holders.iter() {
                    principal_holders_map
                        .insert(response.holder.to_string(), response.data.clone());
                }
                let count = principal_holders.len();
                if count == (p_args.limit as usize) {
                    continue_scanning_principals = true;
                }
                p_args.offset += count as u64;
            }
            Err(err) => {
                let message = format!("{err:?}");
                info!(message, "update_balance_list -> get_principal_holders");
            }
        }
    }

    let mut a_args = GetAccountHoldersArgs {
        offset: 0,
        limit: 100,
    };
    let mut continue_scanning_accounts = true;

    while continue_scanning_accounts {
        continue_scanning_accounts = false;
        info!(
            "get_all_holders -> continue accounts with {}",
            a_args.offset
        );

        match super_stats_v3_c2c_client::get_account_holders(super_stats_canister_id, &a_args).await
        {
            Ok(account_holders) => {
                info!(
                    "get_all_holders -> size of accounts response: {}",
                    account_holders.len()
                );
                for response in account_holders.iter() {
                    account_holders_map.insert(response.holder.clone(), response.data.clone());
                }
                info!(
                    "get_all_holders -> size of map after insertion response: {}",
                    account_holders_map.len()
                );

                let count = account_holders.len();
                if count == (a_args.limit as usize) {
                    continue_scanning_accounts = true;
                }
                a_args.offset += count as u64;
            }
            Err(err) => {
                let message = format!("{err:?}");
                info!(message, "update_balance_list -> get_account_holders");
            }
        }
    }
    info!(
        "get_all_holders is done, returning the maps, accounts count: {}, principals count: {}",
        account_holders_map.len(),
        principal_holders_map.len()
    );
    (principal_holders_map, account_holders_map)
}

fn check_and_update_list(
    list: &mut NormalBTreeMap<Account, WalletOverview>,
    key: Account,
    new_value: WalletOverview,
) {
    match list.get(&key) {
        Some(list_value) => {
            // wallet already in the list
            let updated_value = WalletOverview {
                ledger: list_value.ledger + new_value.ledger,
                governance: list_value.governance.clone() + new_value.governance,
                total: list_value.total + new_value.total,
            };
            list.insert(key, updated_value);
        }
        None => {
            list.insert(key, new_value);
        }
    }
}

fn count_active_users(list: &NormalBTreeMap<Account, WalletOverview>) -> usize {
    list.values().filter(|wallet| wallet.total > 0).count()
}
fn sort_map_descending(
    state_vec: &mut SVec<WalletEntry, VM>,
    map: &NormalBTreeMap<Account, WalletOverview>,
) {
    let mut vec: Vec<WalletEntry> = map
        .iter()
        .map(|(k, v)| WalletEntry(k.clone(), v.clone()))
        .collect();

    vec.sort_by(|a, b| b.1.total.cmp(&a.1.total));

    for (index, value) in vec.iter().enumerate() {
        if state_vec.len() > (index as u64) {
            state_vec.set(index as u64, value);
        } else {
            match state_vec.push(value) {
                Ok(_) => {}
                Err(e) => info!(
                    "sort_map_descneding -> error when inserting to state_vec: {}",
                    e
                ),
            }
        }
    }
}
