//! As the GLD NFTs are using the ORIGYN NFT standard, the
//! royalty fees for transactions are protected for any transfer.
//! This includes also the swapping of NFTs for GLDT.
//! To incentivice users to swap their NFTs for GLDT, the
//! foundation is compensating the fees for the first 100 million
//! GLDT. This canister takes care of the fee compensation.

use candid::{ CandidType, Deserialize, Nat, Principal };
use canistergeek_ic_rust::logger::log_message;
use gldt_libs::constants::GLDT_TX_FEE;
use gldt_libs::misc::{
    get_principal_from_gldnft_account,
    convert_gld_nft_account_to_icrc1_account,
};
use ic_cdk::{ api, storage };
use ic_cdk_macros::{ export_candid, init, query, update };

use icrc_ledger_types::icrc1::transfer::{ NumTokens, TransferArg };
use serde::Serialize;
use std::cell::RefCell;

use gldt_libs::gld_nft::{ Service as GldNft_service, HistoryResult, TransactionRecord_txn_type };
use gldt_libs::gldt_ledger::Service as ICRC1_service;
use gldt_libs::types::{ NftWeight, GldtTokenSpec, GldtNumTokens };

mod registry;
mod error;

use registry::{ Registry, FeeRegistryEntry };
use error::{ CustomError, ErrorType };

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
    /// The timer interval in seconds
    timer_interval_secs: u64,
    /// Whether or not the canister compensation is enabled.
    enabled: bool,
    /// The canister ID of the GLDT canister.
    gldt_canister_id: Principal,
    /// The canister ID of the GLDT ledger canister.
    gldt_ledger_canister_id: Principal,
    /// Canister IDs of the Origyn NFT canisters that manages gold NFTs.
    /// Is a tuple of the canister ID, the weight of the NFT and the last index checked.
    gld_nft_canister_ids: Vec<GldNftConf>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            timer_interval_secs: 60,
            enabled: false,
            gldt_ledger_canister_id: Principal::anonymous(),
            gld_nft_canister_ids: Vec::new(),
            gldt_canister_id: Principal::anonymous(),
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
    MANAGERS.with(|cell| {
        *cell.borrow_mut() = vec![api::caller()];
    });

    cronjob_master();
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

fn cronjob_master() {
    // only run the script if it is enabled
    if !CONF.with(|cell| cell.borrow().enabled) {
        return;
    }
    // activate the timer
    let interval = std::time::Duration::from_secs(
        CONF.with(|cell| cell.borrow().timer_interval_secs)
    );

    ic_cdk::println!("Starting a periodic task with interval {interval:?}");
    let run = || ic_cdk::spawn(compensation_cronjob());
    ic_cdk_timers::set_timer_interval(interval, run);
}

fn calculate_compensation(sale_price: NumTokens) -> NumTokens {
    // The user should in the end have the sale_price + GLDT_TX_FEE on his balance.
    // There are three royalties and one intermediate transaction that need to be considered.
    // The fees are in total 1% which are fully compensated.
    // Therefore, the equation is: (sale_price - GLDT_TX_FEE) / 100 + 3 * GLDT_TX_FEE
    (sale_price - GLDT_TX_FEE) / 100 + 3 * GLDT_TX_FEE
}

/// The fee compensation canister is checking the NFT canister for new royalty payments.
#[update]
async fn compensation_cronjob() {
    let (gld_nft_canister_ids, gldt_canister_id, gldt_ledger_canister_id) = CONF.with(|cell| {
        let conf = cell.borrow();
        (
            conf.gld_nft_canister_ids.clone(),
            conf.gldt_canister_id.clone(),
            conf.gldt_ledger_canister_id.clone(),
        )
    });

    // define the constants for the check
    let token_spec = GldtTokenSpec::new(gldt_ledger_canister_id).get();
    for canister in gld_nft_canister_ids.into_iter() {
        let GldNftConf { gld_nft_canister_id, weight, last_query_index } = canister;
        // expected sale price is the weight of the NFT * 100
        let expected_sale_price = GldtNumTokens::new_from_weight(weight).unwrap_or_default().get();
        // expected royalty fee is 0.5% of the sale price, also decucting the TX fee
        let expected_royalty_fee = (expected_sale_price.clone() - GLDT_TX_FEE) / 200;
        //
        let fee_compensation = calculate_compensation(expected_sale_price.clone());
        // 1. get historical records
        let gld_nft_service = GldNft_service(gld_nft_canister_id);
        match
            gld_nft_service.history_nft_origyn(
                "".to_string(),
                Some(last_query_index.clone()),
                None
            ).await
        {
            Ok((HistoryResult::ok(vec),)) => {
                let num_new_entries = vec.len();
                let new_entries = vec
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, val)| {
                        // extract royalty_paid transactions
                        match &val.txn_type {
                            TransactionRecord_txn_type::royalty_paid {
                                pubtag,
                                token,
                                buyer,
                                sale_id,
                                seller,
                                ..
                            } => {
                                // select only the one that has a tag of "com.origyn.royalty.network"
                                if pubtag != &"com.origyn.royalty.network".to_string() {
                                    return None;
                                }
                                // select only the ones where the token is GLDT
                                if token.clone() != token_spec {
                                    return None;
                                }
                                // select only the ones where the buyer is the GLDT canister
                                if
                                    let Some(principal) = get_principal_from_gldnft_account(
                                        buyer.clone()
                                    )
                                {
                                    if principal.to_text() != gldt_canister_id.to_text() {
                                        return None;
                                    }
                                } else {
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
                                let entry = FeeRegistryEntry::new(
                                    fee_compensation.clone(),
                                    gld_nft_canister_id.clone(),
                                    api::time() / 1_000_000_000,
                                    Nat::from(idx),
                                    None
                                );
                                Some((key, entry))
                            }
                            _ => None,
                        }
                    });
                // Since all entries that enter here are supposed to be legit, the ones that
                // don't pass the following checks are also added to the registry for troubleshooting.
                for (key, entry) in new_entries {
                    // create the entry to the registry and validate that the sale_id doesn't already exist
                    let entry_added = REGISTRY.with(
                        |cell| -> Result<(), String> {
                            let mut registry = cell.borrow_mut();
                            registry.init_entry(key.clone(), entry.clone())
                        }
                    );
                    match entry_added {
                        Err(msg) => {
                            log_message(
                                format!("WARNING :: compensation_cronjob :: failed to add entry to registry. Message: {}", msg)
                            );
                            continue;
                        }
                        Ok(_) => {}
                    }
                    // validate that the amount of the paid royalty is the expected value
                    if expected_royalty_fee != entry.get_amount() {
                        log_message(
                            format!(
                                "WARNING :: compensation_cronjob :: expected royalty fee of {} but got {} for sale_id {} of user {}. Adding to registry.",
                                expected_royalty_fee,
                                entry.get_amount(),
                                key.1,
                                key.0.owner.to_text()
                            )
                        );
                        REGISTRY.with(|cell| {
                            let mut registry = cell.borrow_mut();
                            registry.update_failed(
                                key,
                                CustomError::new_with_message(
                                    ErrorType::InvalidRoyaltyFee,
                                    format!(
                                        "Expected royalty fee of {} but got {}",
                                        expected_royalty_fee,
                                        entry.get_amount()
                                    )
                                )
                            )
                        });
                        continue;
                    }
                    // send the transfer request
                    let transfer_args = TransferArg {
                        memo: None,
                        amount: entry.get_amount(),
                        fee: Some(Nat::from(GLDT_TX_FEE)),
                        from_subaccount: None,
                        to: key.0,
                        created_at_time: None,
                    };
                    let gldt_ledger_service = ICRC1_service(gldt_ledger_canister_id);
                    match gldt_ledger_service.icrc1_transfer(transfer_args).await {
                        Ok((Ok(v),)) => {
                            // update the entry in the registry
                            REGISTRY.with(|cell| {
                                let mut registry = cell.borrow_mut();
                                registry.update_completed(key, v)
                            });
                        }
                        _ => {}
                    }
                }
                // update the last query index
                CONF.with(|cell| {
                    let mut conf = cell.borrow_mut();
                    conf.gld_nft_canister_ids = conf.gld_nft_canister_ids
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
            }
            _ => {}
        };
    }
}

fn validate_caller() -> Result<(), CustomError> {
    MANAGERS.with(|m| {
        if !m.borrow().contains(&api::caller()) {
            return Err(
                CustomError::new_with_message(ErrorType::Unauthorized, "Invalid caller".to_string())
            );
        }
        Ok(())
    })
}

export_candid!();
