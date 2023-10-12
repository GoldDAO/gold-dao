//! As the GLD NFTs are using the ORIGYN NFT standard, the
//! royalty fees for transactions are protected for any transfer.
//! This includes also the swapping of NFTs for GLDT.
//! To incentivice users to swap their NFTs for GLDT, the
//! foundation is compensating the fees for the first 100 million
//! GLDT. This canister takes care of the fee compensation.

use candid::{ CandidType, Deserialize, Nat, Principal };
use canistergeek_ic_rust::logger::log_message;
use ic_cdk::{ api, storage };
use ic_cdk_macros::{ export_candid, init, query, update };

use serde::Serialize;
use std::cell::RefCell;

use gldt_libs::gld_nft::{ Service, HistoryResult };
use gldt_libs::types::NftWeight;

mod registry;
mod error;

use registry::Registry;
use error::{ CustomError, ErrorType };

pub type Index = Nat;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
struct Conf {
    /// The last index when the NFT canister was queried to check for new royalty payments
    last_query_index: Nat,
    /// The timer interval in seconds
    timer_interval_secs: u64,
    /// Whether or not the canister compensation is enabled.
    enabled: bool,
    /// The canister ID of the GLDT ledger canister.
    gldt_ledger_canister_id: Principal,
    /// Canister IDs of the Origyn NFT canisters that manages gold NFTs.
    /// Is a tuple of the canister ID, the weight of the NFT and the last index checked.
    gld_nft_canister_ids: Vec<(Principal, NftWeight, Index)>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            last_query_index: Nat::default(),
            timer_interval_secs: 60,
            enabled: false,
            gldt_ledger_canister_id: Principal::anonymous(),
            gld_nft_canister_ids: Vec::new(),
        }
    }
}

// The stable memory of the canister.
thread_local! {
    /* stable */
    static REGISTRY: RefCell<Registry> = RefCell::default();
    static CONF: RefCell<Conf> = RefCell::default();
    static MANAGERS: RefCell<Vec<Principal>> = RefCell::default();
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
        Ok(_) => log_message("INFO :: pre_upgrade :: stable memory saved".to_string()),
        Err(msg) =>
            api::trap(
                &format!("ERROR :: pre_upgrade :: failed to save stable memory. Message: {}", msg)
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
                &format!("Failed to restore from stable memory. Reverting upgrade. Message: {}", msg)
            );
        }
    }

    log_message("executed post_upgrade".to_string());
}

#[init]
fn init(conf: Conf) {
    CONF.with(|cell| {
        *cell.borrow_mut() = conf.clone();
    });

    if conf.enabled {
        // activate the timer
        let interval = std::time::Duration::from_secs(
            CONF.with(|cell| cell.borrow().timer_interval_secs)
        );
        ic_cdk::println!("Starting a periodic task with interval {interval:?}");
        ic_cdk_timers::set_timer_interval(interval, || {
            compensation_cronjob();
        });
    }
}

/// Returns the GLDT balance of the fee compensation canister.
#[query]
fn get_balance() -> Result<(), ()> {
    Ok(())
}

/// Turns the compensation on or off.
#[update]
pub fn set_compensation_enabled(enabled: bool) -> Result<(), CustomError> {
    validate_caller()?;
    CONF.with(|cell| {
        let mut conf = cell.borrow_mut();
        conf.enabled = enabled;
    });
    Ok(())
}

/// Gets the status of whether or not the compensation is active.
#[query]
fn get_compensation_enabled() -> Result<bool, ()> {
    Ok(CONF.with(|cell| cell.borrow().enabled))
}

/// Sets the timer interval of the automatic royalty payout check.
#[update]
fn set_timer_interval_secs(timer_interval_secs: u64) -> Result<(), CustomError> {
    validate_caller()?;
    CONF.with(|cell| {
        let mut conf = cell.borrow_mut();
        conf.timer_interval_secs = timer_interval_secs;
    });
    Ok(())
}

/// Gets the timer interval of the automatic royalty payout check.
#[query]
fn get_timer_interval_secs() -> Result<u64, ()> {
    Ok(CONF.with(|cell| cell.borrow().timer_interval_secs))
}

/// The fee compensation canister is checking the NFT canister for new royalty payments.
async fn compensation_cronjob() -> Result<(), CustomError> {
    // only run the script if it's enabled
    if !CONF.with(|cell| cell.borrow().enabled) {
        return Ok(());
    }

    let (last_query_index, gld_nft_canister_ids) = CONF.with(|cell| (
        cell.borrow().last_query_index.clone(),
        cell.borrow().gld_nft_canister_ids.clone(),
    ));
    // 1. get historical records
    let gld_nft_service = Service(gld_nft_canister_ids[0].0);
    let new_entries = match
        gld_nft_service.history_nft_origyn("".to_string(), Some(last_query_index), None).await
    {
        Ok((HistoryResult::ok(vec),)) => {
            // Ok(vec);
        }
        _ => {}
    };
    Ok(())
}

fn validate_caller() -> Result<(), CustomError> {
    MANAGERS.with(|m| {
        if !m.borrow().contains(&api::caller()) {
            return Err(CustomError::new_with_message(ErrorType::Unauthorized, "Invalid caller"));
        }
        Ok(())
    })
}

export_candid!();
