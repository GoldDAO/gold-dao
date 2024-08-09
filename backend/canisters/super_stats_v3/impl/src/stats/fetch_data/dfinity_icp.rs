use candid::{ CandidType, Nat };
use num_traits::ToPrimitive;
use serde::{ Serialize, Deserialize };
use crate::core::utils::{ canister_call, log, nat_to_u128 };
use super_stats_v3_api::{
    custom_types::{ ProcessedTX, TransactionType },
    fetch_data::dfinity_icp::{
        ArchiveGetBlocksArgs,
        ArchiveGetBlocksResult,
        ArchiveOperation,
        ArchivedBlocksRange,
        CandidBlock,
        CandidOperation,
        GetBlocksArgs,
        QueryBlocksResponse,
    },
    runtime::RUNTIME_STATE,
    stats::{constants::{
        DAY_AS_NANOS,
        HOUR_AS_NANOS,
        MAX_TOTAL_DOWNLOAD,
        MAX_TRANSACTION_BATCH_SIZE,
    }, updates::init_target_ledger::TargetArgs},
};

//Set target canister, tx store, fee and decimals to runtime memory
pub async fn t1_impl_set_target_canister(args: TargetArgs) -> Result<String, String> {
    let check = RUNTIME_STATE.with(|s| { s.borrow().data.target_ledger_locked });
    let mut had_error = false;
    let mut errors: Vec<String> = Vec::new();
    if check == true {
        ic_cdk::trap(
            "Target canister can't be changed after being set. Re-install canister to change."
        );
    } else {
        // get/ set tx fee
        let result: Result<(Nat,), (ic_cdk::api::call::RejectionCode, String)> = canister_call(
            args.target_ledger.as_str(),
            "icrc1_fee",
            ()
        ).await;
        match result {
            Ok(v) => {
                match nat_to_u128(v.0) {
                    Ok(v_128) => {
                        RUNTIME_STATE.with(|s| { s.borrow_mut().data.set_ledger_fee(v_128) });
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                let call_error = format!("Call 1 - {:?}. {}", e.0, e.1);
                errors.push(call_error);
                had_error = true;
            }
        }
        // get/ set decimals
        let dec_result: Result<(u8,), (ic_cdk::api::call::RejectionCode, String)> = canister_call(
            args.target_ledger.as_str(),
            "icrc1_decimals",
            ()
        ).await;
        match dec_result {
            Ok(v) => {
                RUNTIME_STATE.with(|s| { s.borrow_mut().data.set_ledger_decimals(v.0) });
            }
            Err(e) => {
                let call_error = format!("Call 2 - {:?}. {}", e.0, e.1);
                errors.push(call_error);
                had_error = true;
            }
        }
    }

    if had_error == true {
        let all_errors = format!("{:?}", errors);
        return Err(all_errors);
    } else {
        // set ledger
        RUNTIME_STATE.with(|s| { s.borrow_mut().data.set_target_ledger(args.target_ledger) });
        // set hourly
        RUNTIME_STATE.with(|s| {
            s.borrow_mut().data.latest_blocks.hours_nano =
                (args.hourly_size as u64) * HOUR_AS_NANOS;
        });
        // set daily
        RUNTIME_STATE.with(|s| {
            s.borrow_mut().data.latest_blocks.days_nano = (args.daily_size as u64) * DAY_AS_NANOS;
        });
        // lock
        RUNTIME_STATE.with(|s| {
            s.borrow_mut().data.target_ledger_locked = true;
        });

        let res = String::from("Target canister, fee and decimals set");
        log(res.clone());
        return Ok(res);
    }
}

pub async fn t1_download_transactions() -> Result<Vec<ProcessedTX>, String> {
    // check init done
    RUNTIME_STATE.with(|s| {
        let check = s.borrow().data.target_ledger_locked;
        if check == false {
            ic_cdk::trap("Target Ledger is not yet set!")
        }
    });
    // target ledger
    let ledger_canister = RUNTIME_STATE.with(|s| { s.borrow().data.get_target_ledger() });
    // get tip of chain
    let chain_tip: u64;
    let tip_call = get_tip_of_chain(ledger_canister.as_str()).await;
    match tip_call {
        Ok(tip) => {
            chain_tip = tip.clone();
            RUNTIME_STATE.with(|s| {
                s.borrow_mut().stats.ledger_tip_of_chain = tip;
            });
        }
        Err(e) => {
            let error = format!("Error fetching tip of ledger chain {}", e);
            return Err(error);
        }
    }
    // fetch transaction/ block data
    let next_block_needed = RUNTIME_STATE.with(|s| { s.borrow().stats.get_next_block() });
    if chain_tip > next_block_needed {
        RUNTIME_STATE.with(|s| {
            s.borrow_mut().stats.set_is_upto_date(false);
        });
        let new_txs_res = download_manager(
            chain_tip,
            next_block_needed,
            ledger_canister.as_str()
        ).await;
        match new_txs_res {
            Ok(new_txs) => {
                log(format!("Download manager returned {}", new_txs.len()));
                return Ok(new_txs);
            }
            Err(e) => {
                log(format!("Download manager returned Error {}", e));
                return Err(e);
            }
        }
    } else {
        // nothing to download return empty
        log("Nothing to download");
        let return_empty: Vec<ProcessedTX> = Vec::new();
        return Ok(return_empty);
    }
}

async fn get_tip_of_chain(ledger_id: &str) -> Result<u64, String> {
    let req = GetBlocksArgs {
        start: 0,
        length: 1,
    };
    let result: Result<
        (QueryBlocksResponse,),
        (ic_cdk::api::call::RejectionCode, String)
    > = canister_call(ledger_id, "query_blocks", req).await;
    match result {
        Ok(value) => { Ok(value.0.chain_length) }
        Err(e) => {
            let error = format!("Tip of Chain Error - {:?}. {}", e.0, e.1);
            Err(error)
        }
    }
}

async fn download_manager(
    tip: u64,
    next_block: u64,
    ledger: &str
) -> Result<Vec<ProcessedTX>, String> {
    let mut temp_tx_array: Vec<ProcessedTX> = Vec::new();
    let last_block_in_chain = if tip != 0 {
        tip.saturating_sub(1_u64)
    } else {
        return Ok(temp_tx_array);
    };
    let last_block_processed = if next_block != 0 { next_block.saturating_sub(1) } else { 0 };
    let blocks_needed = last_block_in_chain.saturating_sub(last_block_processed);
    let chunks_needed = (
        (blocks_needed as f64) / (MAX_TRANSACTION_BATCH_SIZE as f64)
    ).ceil() as u32;
    log("[][] ----- Starting ICP Download ----- [][]");
    log(
        format!(
            "Blocks Needed: {}, Chunks Needed: {}, Tip: {}, Next-Block: {}",
            blocks_needed,
            chunks_needed,
            tip,
            next_block
        )
    );

    // Download in chunks
    let mut start: u64;
    let mut length: u64;
    let mut remaining: u64;
    let mut completed_this_run: u64 = 0;

    let max_loops = (
        (MAX_TOTAL_DOWNLOAD as f64) / (MAX_TRANSACTION_BATCH_SIZE as f64)
    ).ceil() as u32;
    let chunks: u32 = if chunks_needed > max_loops { max_loops } else { chunks_needed };
    // Loop x number of times
    for i in 0..chunks {
        start = if i == 0 { next_block } else { next_block + completed_this_run };
        remaining = tip - start;
        length = if remaining > (MAX_TRANSACTION_BATCH_SIZE as u64) {
            MAX_TRANSACTION_BATCH_SIZE as u64
        } else {
            remaining
        };
        // Get next chunk of transactions
        let txns: Result<Vec<ProcessedTX>, String> = icp_download_chunk(
            start,
            length,
            ledger
        ).await;
        // add to temp vec
        let txns_len;
        match txns {
            Ok(value) => {
                txns_len = value.len() as u64;
                for tx in value {
                    temp_tx_array.push(tx);
                }
            }
            Err(e) => {
                let error = format!("Error downloading blocks - {}", e);
                return Err(error);
            }
        }
        completed_this_run += txns_len;
    }
    return Ok(temp_tx_array);
}

async fn icp_download_chunk(
    start: u64,
    length: u64,
    ledger_id: &str
) -> Result<Vec<ProcessedTX>, String> {
    let req = GetBlocksArgs {
        start,
        length,
    };
    let ledger_call: Result<
        (QueryBlocksResponse,),
        (ic_cdk::api::call::RejectionCode, String)
    > = canister_call(ledger_id, "query_blocks", req).await;
    match ledger_call {
        Ok(value) => {
            // check if archive call is needed
            match (value.0.blocks.is_empty(), value.0.archived_blocks.is_empty()) {
                (false, false) => {
                    // there are archive + ledger blocks needing downloaded
                    let mut all_txs: Vec<ProcessedTX> = Vec::new();
                    // loop over archives
                    for archived in value.0.archived_blocks {
                        let arve_txs = get_transactions_from_archive(&archived).await;
                        match arve_txs {
                            Ok(mut value) => {
                                all_txs.append(&mut value);
                            }
                            Err(e) => {
                                let error = format!("Error fetching blocks from archive - {}", e);
                                return Err(error);
                            }
                        }
                    }
                    // get last archive block index
                    let last_block = get_highest_block_index(&all_txs);
                    let mut next_block_needed: u64 = last_block.saturating_add(1);

                    // loop over ledger blocks
                    let mut ledger_txs: Vec<ProcessedTX> = Vec::new();
                    for (_i, block) in value.0.blocks.iter().enumerate() {
                        let prced = process_ledger_block(block, next_block_needed);
                        match prced {
                            Some(value) => {
                                ledger_txs.push(value);
                                next_block_needed = next_block_needed.saturating_add(1);
                            }
                            None => {} // do nothing
                        }
                    }
                    // combine and return
                    all_txs.append(&mut ledger_txs);
                    return Ok(all_txs);
                }
                (false, true) => {
                    // Ledger blocks only - no archive blocks
                    let mut ledger_txs: Vec<ProcessedTX> = Vec::new();
                    let mut next_block = start;
                    for (_i, block) in value.0.blocks.iter().enumerate() {
                        let prced = process_ledger_block(block, next_block);
                        match prced {
                            Some(value) => {
                                ledger_txs.push(value);
                                next_block = next_block.saturating_add(1);
                            }
                            None => {} // do nothing
                        }
                    }
                    return Ok(ledger_txs);
                }
                (true, false) => {
                    // Archive blocks Only - no ledger blocks
                    let mut all_txs: Vec<ProcessedTX> = Vec::new();
                    // loop over archives
                    for archived in value.0.archived_blocks {
                        let arve_txs = get_transactions_from_archive(&archived).await;
                        match arve_txs {
                            Ok(mut value) => {
                                all_txs.append(&mut value);
                            }
                            Err(e) => {
                                let error =
                                    format!("Error fetching blocks from archive (2) - {}", e);
                                return Err(error);
                            }
                        }
                    }
                    return Ok(all_txs);
                }
                (true, true) => {
                    let empty_txs: Vec<ProcessedTX> = Vec::new();
                    return Ok(empty_txs);
                } // no blocks to fetch
            }
        }
        Err(e) => {
            let error = format!(
                "Error fetching blocks from ledger (query_blocks) - {:?}. {}",
                e.0,
                e.1
            );
            return Err(error);
        }
    }
}

async fn get_transactions_from_archive(
    archived: &ArchivedBlocksRange
) -> Result<Vec<ProcessedTX>, String> {
    let mut processed_transactions: Vec<ProcessedTX> = Vec::new();
    let req = ArchiveGetBlocksArgs {
        start: archived.start.clone(),
        length: archived.length.clone(),
    };
    let mut master_block = archived.start;
    let ledger_id = archived.callback.0.principal.to_text();
    let method = &archived.callback.0.method;
    let ledger_call: Result<
        (ArchiveGetBlocksResult,),
        (ic_cdk::api::call::RejectionCode, String)
    > = canister_call(ledger_id.as_str(), method, req).await;
    match ledger_call {
        Ok(value) => {
            match value.0 {
                ArchiveGetBlocksResult::Ok(data) => {
                    for bl in data.blocks {
                        match bl.transaction.operation {
                            Some(block_data) => {
                                match block_data {
                                    ArchiveOperation::Mint { to, amount } => {
                                        let to_ac = hex::encode(to);
                                        let val = amount.e8s as u128;
                                        processed_transactions.push(ProcessedTX {
                                            block: master_block.clone(),
                                            hash: String::from("no-hash"),
                                            tx_type: TransactionType::Mint.to_string(),
                                            from_account: String::from("Token Ledger"),
                                            to_account: to_ac,
                                            tx_value: val,
                                            tx_fee: None,
                                            tx_time: bl.timestamp.timestamp_nanos,
                                            spender: None,
                                        });
                                        master_block = master_block.saturating_add(1_u64);
                                    }
                                    ArchiveOperation::Burn { from, amount, spender } => {
                                        let fm_ac = hex::encode(from);
                                        let spend = if let Some(v) = spender {
                                            Some(hex::encode(v))
                                        } else {
                                            None
                                        };
                                        let val = amount.e8s as u128;
                                        processed_transactions.push(ProcessedTX {
                                            block: master_block.clone(),
                                            hash: String::from("no-hash"),
                                            tx_type: TransactionType::Burn.to_string(),
                                            from_account: fm_ac,
                                            to_account: String::from("Token Ledger"),
                                            tx_value: val,
                                            tx_fee: None,
                                            tx_time: bl.timestamp.timestamp_nanos,
                                            spender: spend,
                                        });
                                        master_block = master_block.saturating_add(1_u64);
                                    }
                                    ArchiveOperation::Transfer {
                                        to,
                                        fee,
                                        from,
                                        amount,
                                        spender,
                                    } => {
                                        let to_ac = hex::encode(to);
                                        let fm_ac = hex::encode(from);
                                        let spend = if let Some(v) = spender {
                                            Some(hex::encode(v))
                                        } else {
                                            None
                                        };
                                        let fee = if fee.e8s != 0 {
                                            Some(fee.e8s as u128)
                                        } else {
                                            None
                                        };
                                        let val = amount.e8s as u128;
                                        processed_transactions.push(ProcessedTX {
                                            block: master_block.clone(),
                                            hash: String::from("no-hash"),
                                            tx_type: TransactionType::Transfer.to_string(),
                                            from_account: fm_ac,
                                            to_account: to_ac,
                                            tx_value: val,
                                            tx_fee: fee,
                                            tx_time: bl.timestamp.timestamp_nanos,
                                            spender: spend,
                                        });
                                        master_block = master_block.saturating_add(1_u64);
                                    }
                                    ArchiveOperation::Approve {
                                        fee,
                                        from,
                                        allowance_e8s,
                                        allowance,
                                        expected_allowance: _,
                                        expires_at: _,
                                        spender,
                                    } => {
                                        let allce = allowance_e8s.0
                                            .to_i128()
                                            .unwrap_or_else(|| 0_i128);
                                        let to_ac = hex::encode(spender.clone());
                                        let fm_ac = hex::encode(from);
                                        let spend = Some(hex::encode(spender));
                                        let fee = if fee.e8s != 0 {
                                            Some(fee.e8s as u128)
                                        } else {
                                            None
                                        };
                                        let val = if (allce as u64) >= allowance.e8s {
                                            allce as u128
                                        } else {
                                            allowance.e8s as u128
                                        };
                                        processed_transactions.push(ProcessedTX {
                                            block: master_block.clone(),
                                            hash: String::from("no-hash"),
                                            tx_type: TransactionType::Approve.to_string(),
                                            from_account: fm_ac,
                                            to_account: to_ac,
                                            tx_value: val,
                                            tx_fee: fee,
                                            tx_time: bl.timestamp.timestamp_nanos,
                                            spender: spend,
                                        });
                                        master_block = master_block.saturating_add(1_u64);
                                    }
                                }
                            }
                            None => {} // do nothing
                        }
                    }
                }
                ArchiveGetBlocksResult::Err(e) => {
                    let error = format!("Error fetching archive blocks (1) - {:?}", e);
                    return Err(error);
                }
            }
        }
        Err(e) => {
            let error = format!("Error fetching archive blocks (2) - {:?}. {}", e.0, e.1);
            return Err(error);
        }
    }
    return Ok(processed_transactions);
}

fn process_ledger_block(block: &CandidBlock, next_block_number: u64) -> Option<ProcessedTX> {
    match block.transaction.operation.clone() {
        Some(block_operation) => {
            match block_operation {
                CandidOperation::Mint { to, amount } => {
                    let to_ac = hex::encode(to);
                    let val = amount.e8s as u128;
                    return Some(ProcessedTX {
                        block: next_block_number,
                        hash: String::from("no-hash"),
                        tx_type: TransactionType::Mint.to_string(),
                        from_account: String::from("Token Ledger"),
                        to_account: to_ac,
                        tx_value: val,
                        tx_fee: None,
                        tx_time: block.timestamp.timestamp_nanos,
                        spender: None,
                    });
                }
                CandidOperation::Burn { from, amount, spender } => {
                    let fm_ac = hex::encode(from);
                    let spend = if let Some(v) = spender { Some(hex::encode(v)) } else { None };
                    let val = amount.e8s as u128;
                    return Some(ProcessedTX {
                        block: next_block_number,
                        hash: String::from("no-hash"),
                        tx_type: TransactionType::Burn.to_string(),
                        from_account: fm_ac,
                        to_account: String::from("Token Ledger"),
                        tx_value: val,
                        tx_fee: None,
                        tx_time: block.timestamp.timestamp_nanos,
                        spender: spend,
                    });
                }
                CandidOperation::Transfer { to, fee, from, amount, spender } => {
                    let to_ac = hex::encode(to);
                    let fm_ac = hex::encode(from);
                    let spend = if let Some(v) = spender { Some(hex::encode(v)) } else { None };
                    let fee = if fee.e8s != 0 { Some(fee.e8s as u128) } else { None };
                    let val = amount.e8s as u128;
                    return Some(ProcessedTX {
                        block: next_block_number,
                        hash: String::from("no-hash"),
                        tx_type: TransactionType::Transfer.to_string(),
                        from_account: fm_ac,
                        to_account: to_ac,
                        tx_value: val,
                        tx_fee: fee,
                        tx_time: block.timestamp.timestamp_nanos,
                        spender: spend,
                    });
                }
                CandidOperation::Approve {
                    fee,
                    from,
                    allowance_e8s,
                    allowance,
                    expected_allowance: _,
                    expires_at: _,
                    spender,
                } => {
                    let allce = allowance_e8s.0.to_i128().unwrap_or_else(|| 0_i128);
                    let to_ac = hex::encode(spender.clone());
                    let fm_ac = hex::encode(from);
                    let spend = Some(hex::encode(spender));
                    let fee = if fee.e8s != 0 { Some(fee.e8s as u128) } else { None };
                    let val = if (allce as u64) >= allowance.e8s {
                        allce as u128
                    } else {
                        allowance.e8s as u128
                    };
                    return Some(ProcessedTX {
                        block: next_block_number,
                        hash: String::from("no-hash"),
                        tx_type: TransactionType::Approve.to_string(),
                        from_account: fm_ac,
                        to_account: to_ac,
                        tx_value: val,
                        tx_fee: fee,
                        tx_time: block.timestamp.timestamp_nanos,
                        spender: spend,
                    });
                }
            }
        }
        None => {
            return None;
        } // no nothing
    }
}

fn get_highest_block_index(input: &Vec<ProcessedTX>) -> u64 {
    let mut highest = 0;
    for tx in input {
        if tx.block > highest {
            highest = tx.block;
        }
    }
    return highest;
}
