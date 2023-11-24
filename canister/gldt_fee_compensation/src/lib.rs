/*!
# GLDT and GLDT Swapp dApp canisters

As the GLD NFTs are using the ORIGYN NFT standard, the
royalty fees for transactions are protected for any transfer.
This includes also the swapping of NFTs for GLDT.
To incentivice users to swap their NFTs for GLDT, the
foundation is compensating the fees for the first 100 million
GLDT. This canister takes care of the fee compensation.

## Copyright
© 2023  [Bochsler Assets & Securities (BAS) SA], [Switzerland]

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

[Bochsler Assets & Securities (BAS) SA]: https://bas.tech
[Switzerland]: https://www.zefix.ch/fr/search/entity/list/firm/1579921
*/

#![allow(clippy::must_use_candidate, clippy::too_many_lines, clippy::too_many_arguments)]

use candid::{ CandidType, Deserialize, Nat, Principal };
use canistergeek_ic_rust::logger::log_message;
use gldt_libs::constants::GLDT_TX_FEE;
use gldt_libs::misc::{
    convert_gld_nft_account_to_icrc1_account,
    get_principal_from_gldnft_account,
};
use ic_cdk::{ api, storage };
use ic_cdk_macros::{ export_candid, init, query, update };

use ic_cdk_timers::TimerId;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{ NumTokens, TransferArg };
use serde::Serialize;
use serde_json::{ json, Value };
use std::cell::RefCell;
use std::time::Duration;

use gldt_libs::gld_nft::{ HistoryResult, Service as GldNft_service, TransactionRecord_txn_type };
use gldt_libs::gldt_ledger::Service as ICRC1_service;
use gldt_libs::types::{ GldtNumTokens, GldtTokenSpec, NftWeight };
use gldt_libs::error::{ Custom as CustomError, Type as ErrorType };

mod registry;

use registry::{ FeeRegistryEntry, Registry, Status as RegistryStatus };

pub type Index = Nat;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
struct GldNftConf {
    /// The canister ID of the GLD NFT canister.
    gld_nft_canister_id: Principal,
    /// The weight of the NFT.
    weight: NftWeight,
    /// The last index that was checked.
    last_query_index: Index,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
struct Conf {
    /// The timer interval of the cronjob fallback in seconds
    fallback_timer_interval_secs: u64,
    /// The delay of the execution of the compensation in seconds
    execution_delay_secs: u64,
    /// Whether or not the canister compensation is enabled.
    enabled: bool,
    /// The canister ID of the GLDT canister.
    gldt_canister_id: Principal,
    /// The canister ID of the GLDT ledger canister.
    gldt_ledger_canister_id: Principal,
    /// Canister IDs of the Origyn NFT canisters that manages gold NFTs.
    /// Is a tuple of the canister ID, the weight of the NFT and the last index checked.
    gld_nft_canister_conf: Vec<GldNftConf>,
    /// The compensation factor to calculate the amount of fees compensated.
    /// Default is 10, which means that 1% are compensated.
    /// Is allowed to range between including 1 and 10.
    compensation_factor: u64,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            fallback_timer_interval_secs: 3600,
            execution_delay_secs: 20,
            enabled: false,
            gldt_ledger_canister_id: Principal::anonymous(),
            gld_nft_canister_conf: Vec::new(),
            gldt_canister_id: Principal::anonymous(),
            compensation_factor: 10,
        }
    }
}

// The stable memory of the canister.
thread_local! {
    /* stable */
    static REGISTRY: RefCell<Registry> = RefCell::default();
    static CONF: RefCell<Conf> = RefCell::default();
    static MANAGERS: RefCell<Vec<Principal>> = RefCell::default();
    static FALLBACK_TIMER_ID: RefCell<TimerId> = RefCell::default();
    static LAST_NOTIFY_CALL_TIMESTAMP: RefCell<u64> = RefCell::default();
}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    log_message("executing pre_upgrade".to_string());

    // canister geek data
    let monitor_stable_data = canistergeek_ic_rust::monitor::pre_upgrade_stable_data();
    let logger_stable_data = canistergeek_ic_rust::logger::pre_upgrade_stable_data();

    let registry = REGISTRY.with(|cell| cell.borrow().clone());
    let conf = CONF.with(|cell| cell.borrow().clone());
    let managers = MANAGERS.with(|cell| cell.borrow().clone());

    match storage::stable_save((registry, conf, managers, monitor_stable_data, logger_stable_data)) {
        Ok(()) => log_message("INFO :: pre_upgrade :: stable memory saved".to_string()),
        Err(msg) =>
            api::trap(
                &format!("ERROR :: pre_upgrade :: failed to save stable memory. Message: {msg}")
            ),
    }
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    let stable_data: Result<
        (
            Registry,
            Conf,
            Vec<Principal>,
            canistergeek_ic_rust::monitor::PostUpgradeStableData,
            canistergeek_ic_rust::logger::PostUpgradeStableData,
        ),
        String
    > = storage::stable_restore();
    match stable_data {
        Ok((registry, conf, managers, monitor_stable_data, logger_stable_data)) => {
            REGISTRY.with(|cell| {
                *cell.borrow_mut() = registry;
            });
            CONF.with(|cell| {
                *cell.borrow_mut() = conf;
            });
            MANAGERS.with(|cell| {
                *cell.borrow_mut() = managers;
            });
            canistergeek_ic_rust::monitor::post_upgrade_stable_data(monitor_stable_data);
            canistergeek_ic_rust::logger::post_upgrade_stable_data(logger_stable_data);
        }
        Err(msg) => {
            // Traps in pre_upgrade or post_upgrade will cause the upgrade to be reverted
            // and the state to be restored.
            api::trap(
                &format!("Failed to restore from stable memory. Reverting upgrade. Message: {msg}")
            );
        }
    }

    log_message("executed post_upgrade".to_string());
}

#[init]
fn init(conf: Option<Conf>) {
    if let Some(conf) = conf {
        CONF.with(|cell| {
            *cell.borrow_mut() = conf.clone();
        });
    }

    #[cfg(not(test))]
    MANAGERS.with(|cell| {
        *cell.borrow_mut() = vec![api::caller()];
    });
}

#[query]
fn get_gld_nft_conf() -> Vec<GldNftConf> {
    CONF.with(|cell| cell.borrow().gld_nft_canister_conf.clone())
}

#[update]
fn set_gld_nft_conf(gld_nft_conf: Vec<GldNftConf>) -> Result<(), CustomError> {
    validate_caller()?;
    CONF.with(|cell| {
        let mut conf = cell.borrow_mut();
        conf.gld_nft_canister_conf = gld_nft_conf;
    });
    Ok(())
}

#[query]
fn get_compensation_factor() -> u64 {
    CONF.with(|cell| cell.borrow().compensation_factor)
}

#[update]
fn set_compensation_factor(new_compensation_factor: u64) -> Result<(), CustomError> {
    validate_caller()?;

    if new_compensation_factor > 10 || new_compensation_factor < 1 {
        return Err(
            CustomError::new_with_message(
                ErrorType::Other,
                "Compensation factor value has to be between (including) 1 and 10 (mean 0.1% and 1%).".to_string()
            )
        );
    }

    CONF.with(|cell| {
        cell.borrow_mut().compensation_factor = new_compensation_factor;
    });
    Ok(())
}

/// Returns the GLDT balance of the fee compensation canister.
#[update]
async fn get_balance() -> Result<Nat, ()> {
    let gldt_ledger_canister_id = CONF.with(|cell| cell.borrow().gldt_ledger_canister_id);
    let service_ledger = ICRC1_service(gldt_ledger_canister_id);
    if
        let Ok((balance,)) = service_ledger.icrc1_balance_of(Account {
            owner: api::id(),
            subaccount: None,
        }).await
    {
        return Ok(balance);
    }
    Err(())
}

/// Turns the compensation on or off.
#[update]
pub fn set_compensation_enabled(enabled: bool) -> Result<(), CustomError> {
    validate_caller()?;
    log_message(format!("Setting compensation enabled to {enabled}"));
    CONF.with(|cell| {
        let mut conf = cell.borrow_mut();
        conf.enabled = enabled;
    });

    if enabled {
        // starts the job
        cronjob_master()
    } else {
        // deletes an existing job if running
        let timer_id = FALLBACK_TIMER_ID.with(|cell| *cell.borrow());
        log_message(format!("Stopping timer with id {timer_id:?}"));
        ic_cdk_timers::clear_timer(timer_id);
        Ok(())
    }
}

/// Gets the status of whether or not the compensation is active.
#[query]
fn get_compensation_enabled() -> bool {
    CONF.with(|cell| cell.borrow().enabled)
}

/// Sets the fallback timer interval of the automatic royalty payout check.
#[update]
fn set_fallback_timer_interval_secs(fallback_timer_interval_secs: u64) -> Result<(), CustomError> {
    validate_caller()?;
    CONF.with(|cell| {
        let mut conf = cell.borrow_mut();
        conf.fallback_timer_interval_secs = fallback_timer_interval_secs;
    });
    Ok(())
}

/// Gets the fallback timer interval of the automatic royalty payout check.
#[query]
fn get_fallback_timer_interval_secs() -> u64 {
    CONF.with(|cell| cell.borrow().fallback_timer_interval_secs)
}

/// Sets the execution delay for the notify execution of the automatic royalty payout check.
#[update]
fn set_execution_delay_secs(execution_delay_secs: u64) -> Result<(), CustomError> {
    validate_caller()?;
    CONF.with(|cell| {
        let mut conf = cell.borrow_mut();
        conf.execution_delay_secs = execution_delay_secs;
    });
    Ok(())
}

/// Gets the execution delay for the notify execution of the automatic royalty payout check.
#[query]
fn get_execution_delay_secs() -> u64 {
    CONF.with(|cell| cell.borrow().execution_delay_secs)
}

#[query]
fn fetch_metadata() -> String {
    let registry_data = REGISTRY.with(|cell| cell.borrow().clone());
    let conf_data = CONF.with(|cell| cell.borrow().clone());
    let managers_data = MANAGERS.with(|cell| cell.borrow().clone());

    json!({
        "registry": registry_data,
        "configuration": conf_data,
        "managers": managers_data,
    }).to_string()
}

#[update]
fn import_data(json_data: String) -> Result<String, CustomError> {
    validate_caller()?;

    let previous_metadata = fetch_metadata();

    let data: Value = serde_json
        ::from_str(&json_data)
        .map_err(|e|
            CustomError::new_with_message(
                ErrorType::Other,
                format!("Error parsing JSON: {:?}", e).to_string()
            )
        )?;

    let registry_data = data["registry"].clone();
    let conf_data = data["configuration"].clone();
    let managers_data = data["managers"].clone();

    REGISTRY.with(
        |cell| -> Result<(), CustomError> {
            *cell.borrow_mut() = serde_json
                ::from_value(registry_data)
                .map_err(|e|
                    CustomError::new_with_message(
                        ErrorType::Other,
                        format!("Error parsing registry data: {:?}", e).to_string()
                    )
                )?;
            Ok(())
        }
    )?;

    CONF.with(
        |cell| -> Result<(), CustomError> {
            *cell.borrow_mut() = serde_json
                ::from_value(conf_data)
                .map_err(|e|
                    CustomError::new_with_message(
                        ErrorType::Other,
                        format!("Error parsing configuration data: {:?}", e).to_string()
                    )
                )?;
            Ok(())
        }
    )?;

    MANAGERS.with(
        |cell| -> Result<(), CustomError> {
            *cell.borrow_mut() = serde_json
                ::from_value(managers_data)
                .map_err(|e|
                    CustomError::new_with_message(
                        ErrorType::Other,
                        format!("Error parsing managers data: {:?}", e).to_string()
                    )
                )?;
            Ok(())
        }
    )?;

    Ok(previous_metadata)
}

/// The master job that triggers the compensation execution.
fn cronjob_master() -> Result<(), CustomError> {
    // only run the script if it is enabled
    if !CONF.with(|cell| cell.borrow().enabled) {
        return Err(
            CustomError::new_with_message(
                ErrorType::CompensationDisabled,
                "Compensation is not enabled".to_string()
            )
        );
    }
    // activate the timer
    let interval = std::time::Duration::from_secs(
        CONF.with(|cell| cell.borrow().fallback_timer_interval_secs)
    );

    log_message(format!("Starting a periodic task with interval {interval:?}"));
    let run = || ic_cdk::spawn(run_compensation_job());

    let current_time = api::time();

    match current_time.checked_add(interval.as_secs()) {
        Some(_) => {
            let timer_id: TimerId = ic_cdk_timers::set_timer_interval(interval, run);

            FALLBACK_TIMER_ID.with(|cell| {
                *cell.borrow_mut() = timer_id;
            });
            Ok(())
        }
        None =>
            Err(
                CustomError::new_with_message(
                    ErrorType::Other,
                    "Fatal error: interval + current_time > u64 MAX.".to_string()
                )
            ),
    }
}

fn calculate_compensation(sale_price: NumTokens) -> NumTokens {
    let compensation_factor = CONF.with(|cell| cell.borrow().compensation_factor);

    // The user should in the end have the sale_price + GLDT_TX_FEE on his balance.
    // There are three royalties and one intermediate transaction that need to be considered.
    // The fees are in total 1% which are fully compensated.
    // Therefore, the equation is: (sale_price - GLDT_TX_FEE) / 100 + 3 * GLDT_TX_FEE
    ((sale_price - GLDT_TX_FEE) * compensation_factor) / 1000 + 3 * GLDT_TX_FEE
}

/// The notify method which is called from the GLDT core canister to trigger the compensation.
#[update]
fn notify_compensation_job() -> Result<(), CustomError> {
    log_message(format!("notify_compensation_job() called by {}", api::caller().to_text()));
    // only the GLDT core canister is allowed to call this method
    if api::caller() != CONF.with(|cell| cell.borrow().gldt_canister_id) {
        return Err(
            CustomError::new_with_message(ErrorType::Unauthorized, "Invalid caller".to_string())
        );
    }
    // only run the script if it is enabled
    if !CONF.with(|cell| cell.borrow().enabled) {
        return Err(CustomError::new(ErrorType::CompensationDisabled));
    }
    // check if the last call was more than the specified delay ago
    let last_call_timestamp = LAST_NOTIFY_CALL_TIMESTAMP.with(|cell| *cell.borrow()); // in nano seconds
    let now = api::time(); // in nano seconds
    let threshold = CONF.with(|cell| cell.borrow().execution_delay_secs) * 1_000_000_000;
    if last_call_timestamp + threshold > now {
        // cancel further execution if a delayed execution is already pending
        return Ok(());
    }
    // update the last call timestamp
    LAST_NOTIFY_CALL_TIMESTAMP.with(|cell| {
        *cell.borrow_mut() = api::time();
    });
    // activate the timer
    let delay = Duration::from_secs(CONF.with(|cell| cell.borrow().execution_delay_secs));
    log_message(format!("Starting a delayed task with delay {delay:?}"));
    let run = || ic_cdk::spawn(run_compensation_job());
    ic_cdk_timers::set_timer(delay, run);
    // return Ok() as the job will finish asynchronously
    Ok(())
}

/// The fee compensation canister is checking the NFT canister for new royalty payments.
async fn run_compensation_job() {
    log_message("Running run_compensation_job()".to_string());
    let mut counter = 0;
    let (gld_nft_canister_conf, gldt_canister_id, gldt_ledger_canister_id) = CONF.with(|cell| {
        let conf = cell.borrow();
        (conf.gld_nft_canister_conf.clone(), conf.gldt_canister_id, conf.gldt_ledger_canister_id)
    });

    // define the constants for the check
    let token_spec = GldtTokenSpec::new(gldt_ledger_canister_id).get();
    for canister in gld_nft_canister_conf {
        let GldNftConf { gld_nft_canister_id, weight, last_query_index } = canister.clone();
        // expected sale price is the weight of the NFT * 100
        let expected_sale_price: Nat;

        if let Ok(val) = GldtNumTokens::new_from_weight(weight) {
            expected_sale_price = val.get();
        } else {
            return;
        }

        // expected royalty fee is 0.5% of the sale price, plus deducting the TX fee
        let expected_royalty_fee = (expected_sale_price.clone() - GLDT_TX_FEE) / 200;
        // calculate the compensated amount based on the sale price
        let fee_compensation = calculate_compensation(expected_sale_price.clone());
        let gld_nft_service = GldNft_service(gld_nft_canister_id);
        if
            let Ok((HistoryResult::ok(res),)) = gld_nft_service.history_nft_origyn(
                String::new(),
                Some(last_query_index.clone()),
                None
            ).await
        {
            let num_new_entries = res.len();
            let new_entries = res
                .iter()
                .enumerate()
                .filter_map(|(idx, val)| {
                    // extract royalty_paid transactions
                    match &val.txn_type {
                        TransactionRecord_txn_type::royalty_paid {
                            tag,
                            token,
                            buyer,
                            sale_id,
                            seller,
                            amount,
                            ..
                        } => {
                            // select only the one that has a tag of "com.origyn.royalty.network"
                            if tag != &"com.origyn.royalty.network".to_string() {
                                log_message(format!("Tag is not correct : {tag} received."));
                                return None;
                            }
                            // select only the ones where the token is GLDT
                            if token.clone() != token_spec {
                                log_message(
                                    format!("Token is not correct : {:?} received.", token.clone())
                                );
                                return None;
                            }
                            // select only the ones where the buyer is the GLDT canister
                            if let Some(principal) = get_principal_from_gldnft_account(buyer) {
                                if principal.to_text() != gldt_canister_id.to_text() {
                                    log_message(
                                        format!(
                                            "Principal id is not correct : received {}.",
                                            principal.to_text()
                                        )
                                    );
                                    return None;
                                }
                            } else {
                                log_message(format!("Couldnt get principal from gldtnft account."));
                                return None;
                            }
                            // select only the ones where the expected royalty fee
                            // matches the amount of the royalty fee in the entry
                            if expected_royalty_fee != *amount {
                                log_message(format!("Expected royalty fee has bad value."));
                                return None;
                            }
                            // select only the ones where the sale_id is defined
                            let sale_id = sale_id.clone()?;
                            // pick out the seller in icrc1 format
                            let seller_icrc1 = convert_gld_nft_account_to_icrc1_account(
                                seller.clone()
                            )?;
                            // for all other cases, return a new entry and its key
                            let key = (seller_icrc1, sale_id);
                            let entry = FeeRegistryEntry {
                                amount: fee_compensation.clone(),
                                gld_nft_canister_id,
                                timestamp: api::time() / 1_000_000_000,
                                history_index: Nat::from(idx),
                                status: RegistryStatus::Ongoing,
                                previous_entry: None,
                                block_height: None,
                            };
                            Some((key, entry))
                        }
                        _ => None,
                    }
                });
            // Since all entries that enter here are supposed to be legit, the ones that
            // don't pass the following checks are also added to the registry for troubleshooting.

            // Create an array of all transfer requests and send them in parellel
            let mut handles = Vec::new();
            for (key, entry) in new_entries {
                handles.push(transfer_compensation(key, entry));
            }
            futures::future::join_all(handles).await;

            // update the last query index
            CONF.with(|cell| {
                let mut conf = cell.borrow_mut();
                conf.gld_nft_canister_conf = conf.gld_nft_canister_conf
                    .iter()
                    .map(|canister| {
                        if canister.gld_nft_canister_id == gld_nft_canister_id {
                            GldNftConf {
                                last_query_index: last_query_index.clone() +
                                Nat::from(num_new_entries),
                                ..canister.clone()
                            }
                        } else {
                            canister.clone()
                        }
                    })
                    .collect();
            });
            counter += num_new_entries;
        };
    }
    log_message(format!("Scanned {counter} new entries for compensation."));
}

async fn transfer_compensation(key: (Account, String), entry: FeeRegistryEntry) {
    // create the entry to the registry and validate that the sale_id doesn't already exist
    let entry_added = REGISTRY.with(
        |cell| -> Result<(), String> {
            let mut registry = cell.borrow_mut();
            registry.init_entry(&key, &entry)
        }
    );
    if let Err(msg) = entry_added {
        log_message(
            format!(
                "WARNING :: compensation_job :: failed to add entry to registry. Message: {msg}"
            )
        );
        return;
    }

    // send the transfer request
    let transfer_args = TransferArg {
        memo: None,
        amount: entry.amount,
        fee: Some(Nat::from(GLDT_TX_FEE)),
        from_subaccount: None,
        to: key.0,
        created_at_time: None,
    };
    let gldt_ledger_canister_id = CONF.with(|cell| cell.borrow().gldt_ledger_canister_id);
    let gldt_ledger_service = ICRC1_service(gldt_ledger_canister_id);
    match gldt_ledger_service.icrc1_transfer(transfer_args.clone()).await {
        Ok((Ok(v),)) => {
            // This is the happy path. All went well when we end up here.
            log_message(
                format!("Successfully transferred GLDT ({}). Message: {v:?}", transfer_args.amount)
            );
            // update the entry in the registry
            REGISTRY.with(|cell| {
                let mut registry = cell.borrow_mut();
                registry.update_completed(&key, v);
            });
        }
        Ok((Err(err),)) => {
            // update the entry in the registry with failed
            log_message(
                format!(
                    "Failed to transfer GLDT ({}). Transfer Error : {err:?}",
                    transfer_args.amount
                )
            );
            REGISTRY.with(|cell| {
                let mut registry = cell.borrow_mut();
                registry.update_failed(
                    &key,
                    CustomError::new_with_message(
                        ErrorType::TransferError,
                        format!("Failed to transfer GLDT. Message: {err:?}")
                    )
                );
            });
        }
        Err(msg) => {
            log_message(
                format!(
                    "Failed to transfer GLDT ({}). Rejection code : {msg:?}",
                    transfer_args.amount
                )
            );
            // update the entry in the registry with failed
            REGISTRY.with(|cell| {
                let mut registry = cell.borrow_mut();
                registry.update_failed(
                    &key,
                    CustomError::new_with_message(
                        ErrorType::TransferError,
                        format!("Failed to transfer GLDT. Message: {msg:?}")
                    )
                );
            });
        }
    }
}

fn validate_caller() -> Result<(), CustomError> {
    #[cfg(test)]
    return Ok(());

    #[cfg(not(test))]
    MANAGERS.with(|m| {
        if !m.borrow().contains(&api::caller()) {
            return Err(
                CustomError::new_with_message(ErrorType::Unauthorized, "Invalid caller".to_string())
            );
        }
        Ok(())
    })
}

// for monitoring during development
#[query(name = "getCanistergeekInformation")]
fn get_canistergeek_information(
    request: canistergeek_ic_rust::api_type::GetInformationRequest
) -> canistergeek_ic_rust::api_type::GetInformationResponse<'static> {
    canistergeek_ic_rust::get_information(request)
}

#[update(name = "updateCanistergeekInformation")]
fn update_canistergeek_information(
    request: canistergeek_ic_rust::api_type::UpdateInformationRequest
) {
    canistergeek_ic_rust::update_information(request);
}

/// This makes this Candid service self-describing, so that for example Candid UI, but also other
/// tools, can seamlessly integrate with it. The concrete interface (method name etc.) is
/// provisional, but works.
#[query(name = "__get_candid_interface_tmp_hack")]
fn get_candid_interface_tmp_hack() -> String {
    include_str!("gldt_fee_compensation.did").to_string()
}
export_candid!();

#[cfg(test)]
mod test;
