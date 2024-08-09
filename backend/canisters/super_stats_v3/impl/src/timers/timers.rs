use std::time::Duration;
use super_stats_v3_api::{
    custom_types::{ IndexerType, ProcessedTX },
    process_data::process_time_stats::StatsType,
    runtime::RUNTIME_STATE,
};

use crate::{
    core::{ utils::{ canister_call, critical_err, log }, working_stats::count_error },
    stats::{
        fetch_data::{
            dfinity_icp::t1_download_transactions,
            dfinity_icrc2::t2_download_transactions,
        },
        process_data::{
            process_index::{ process_smtx_to_index, process_smtx_to_principal_index },
            process_time_stats::calculate_time_stats,
            small_tx::{ processedtx_to_principal_only_smalltx, processedtx_to_smalltx },
        },
    },
};

use super::state::TIMER_STATE;

pub fn start_processing_time_impl(secs: u64) {
    let is_test_mode = RUNTIME_STATE.with(|s| { s.borrow().data.test_mode });
    if is_test_mode {
        ic_cdk::spawn(schedule_data_processing());
    }
    else {
        let secs = Duration::from_secs(secs);
        let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
            ic_cdk::spawn(schedule_data_processing())
        );
        TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
    }
    
}

// Fetch Main quotes
pub async fn schedule_data_processing() {
    // check is busy - early exit
    let is_busy = RUNTIME_STATE.with(|s| { s.borrow().stats.is_busy });
    if is_busy == true {
        return;
    } else {
        // set busy
        RUNTIME_STATE.with(|s| { s.borrow_mut().stats.set_busy() });
    }

    // FETCH LATEST TRANSACTIONS
    let index_type = RUNTIME_STATE.with(|s| { s.borrow().data.get_index_type() });
    let latest_transactions: Result<Vec<ProcessedTX>, String>;
    match index_type {
        // ICP LEDGER
        IndexerType::DfinityIcp => {
            // Download txs from ICP Type Ledger
            latest_transactions = t1_download_transactions().await;
        }
        IndexerType::DfinityIcrc2 => {
            // Download txs from Dfinity ICRC 2 type ledger
            latest_transactions = t2_download_transactions().await;
        }
        IndexerType::DfinityIcrc3 => todo!(),
    }

    // PROCESS LATEST TRANSACTIONS
    match latest_transactions {
        Ok(txs) => {
            // catch 0 len (nothing to download) events. Return early
            if txs.len() == 0 {
                //
                let time = ic_cdk::api::time();
                RUNTIME_STATE.with(|s| {
                    s.borrow_mut().stats.last_update_time = time;
                });

                // set to being not busy
                RUNTIME_STATE.with(|s| { s.borrow_mut().stats.set_not_busy() });
                return;
            }

            // process ptx to stx
            let latest_as_smalltx = processedtx_to_smalltx(&txs);
            // process account index
            let index_res = process_smtx_to_index(latest_as_smalltx);

            // IS ICRC Type Account? (Index Principals)
            if index_type != IndexerType::DfinityIcp {
                // process ptx to stx (principal only)
                let latest_as_smalltx_principal = processedtx_to_principal_only_smalltx(&txs);
                // process principal index
                let index_res_pr = process_smtx_to_principal_index(latest_as_smalltx_principal);
                match index_res_pr {
                    Ok(_v) => {} // do nothing
                    Err(e) => {
                        let error = format!("Error processing stx to index (Principal): {}", e);
                        count_error();
                        critical_err(error).await;
                    }
                }
            } //if

            // outcome of account index
            let mut up2date = false;
            match index_res {
                Ok(processed_tip) => {
                    // store ptx txs in blockstore
                    RUNTIME_STATE.with(|s| { s.borrow_mut().data.latest_blocks.push_tx_vec(txs) });

                    // clear temp vecs
                    RUNTIME_STATE.with(|s| {
                        s.borrow_mut().data.temp_small_tx = Vec::new();
                    });

                    // tip of ledger chain
                    let tip = RUNTIME_STATE.with(|s| { s.borrow().stats.ledger_tip_of_chain });

                    // is up to date?
                    let ptm1 = processed_tip.saturating_add(1); // +1 to account for 0 blockd
                    if &ptm1 == &tip {
                        RUNTIME_STATE.with(|s| {
                            s.borrow_mut().stats.set_is_upto_date(true);
                        });
                        up2date = true;
                    }

                    // next block
                    let next = processed_tip.saturating_add(1);
                    // update last processed metric
                    RUNTIME_STATE.with(|s| { s.borrow_mut().stats.set_next_block(next.clone()) });

                    // last update time
                    let time = ic_cdk::api::time();
                    RUNTIME_STATE.with(|s| {
                        s.borrow_mut().stats.last_update_time = time;
                    });

                    if up2date == true {
                        // self call (process stats)
                        let self_id = RUNTIME_STATE.with(|s| { s.borrow().data.get_self_id() });
                        let sc1: Result<
                            ((),),
                            (ic_cdk::api::call::RejectionCode, String)
                        > = canister_call(self_id.as_str(), "self_call", ()).await;

                        let sc2: Result<
                            ((),),
                            (ic_cdk::api::call::RejectionCode, String)
                        > = canister_call(self_id.as_str(), "self_call2", ()).await;

                        // update latest blocks/ tx_store/ stats/ clear temp vecs
                        match (sc1, sc2) {
                            // call returned ok
                            (Ok(_v), Ok(_v2)) => {
                                log("Timer calls complete. Waiting...");
                            }
                            (Err(e), Err(e2)) => {
                                // call failed
                                let error = format!(
                                    "(self call and self call 2) failed : {:?}, {}. Error 2:: {:?}, {}",
                                    e.0,
                                    e.1,
                                    e2.0,
                                    e2.1
                                );
                                count_error();
                                critical_err(error).await;
                            }
                            (Ok(_v), Err(e2)) => {
                                // call failed
                                let error = format!("(self call 2) failed : {:?}, {}", e2.0, e2.1);
                                count_error();
                                critical_err(error).await;
                            }
                            (Err(e), Ok(_v)) => {
                                // call failed
                                let error = format!("(self call) failed : {:?}, {}", e.0, e.1);
                                count_error();
                                critical_err(error).await;
                            }
                        } // end match self call
                    } // if
                }
                Err(e) => {
                    let error = format!("Error processing stx to index: {}", e);
                    count_error();
                    critical_err(error).await;
                }
            }
        }
        Err(e) => {
            let error = format!("Error downloading transactions from ledger: {}", e);
            count_error();
            critical_err(error).await;
        }
    }

    // set to being not busy
    RUNTIME_STATE.with(|s| { s.borrow_mut().stats.set_not_busy() });
}

// PROCESS DAILY
pub async fn process_self_call() {
    let time_now = ic_cdk::api::time();
    let process_from = RUNTIME_STATE.with(|s| {
        let day_nanos = s.borrow().data.latest_blocks.days_nano;
        return time_now - day_nanos;
    });
    let index_type = RUNTIME_STATE.with(|s| { s.borrow().data.get_index_type() });
    let stats = calculate_time_stats(process_from, StatsType::Daily, index_type, time_now);
    RUNTIME_STATE.with(|s| {
        s.borrow_mut().data.daily_stats = stats;
    });
}

// PROCESS HOUR
pub async fn process_self_call2() {
    let time_now = ic_cdk::api::time();
    let process_from = RUNTIME_STATE.with(|s| {
        let day_nanos = s.borrow().data.latest_blocks.hours_nano;
        return time_now - day_nanos;
    });
    let index_type = RUNTIME_STATE.with(|s| { s.borrow().data.get_index_type() });
    let stats = calculate_time_stats(process_from, StatsType::Hourly, index_type, time_now);
    RUNTIME_STATE.with(|s| {
        s.borrow_mut().data.hourly_stats = stats;
    });
}
