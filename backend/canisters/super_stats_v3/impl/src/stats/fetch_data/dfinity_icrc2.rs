use candid::Nat;
use crate::core::utils::{ canister_call, nat_to_u128, log, nat_to_u64 };
use super_stats_v3_api::{
    custom_types::{ ProcessedTX, TransactionType },
    fetch_data::dfinity_icrc2::{
        Account,
        ArchivedRange1,
        GetBlocksArgs1,
        GetTransactionsResponse,
        Transaction,
        TransactionRange,
        DEFAULT_SUBACCOUNT,
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
pub async fn t2_impl_set_target_canister(args: TargetArgs) -> Result<String, String> {
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

pub async fn t2_download_transactions() -> Result<Vec<ProcessedTX>, String> {
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
    let tip_call = get_tip_of_chain_t2(ledger_canister.as_str()).await;
    match tip_call {
        Ok(tip) => {
            match nat_to_u64(tip) {
                Ok(v_u64) => {
                    chain_tip = v_u64.clone();
                    RUNTIME_STATE.with(|s| {
                        s.borrow_mut().stats.ledger_tip_of_chain = v_u64;
                    });
                }
                Err(e) => {
                    return Err(format!("Could not process tip of chain to u64: {}", e));
                }
            }
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

async fn get_tip_of_chain_t2(ledger_id: &str) -> Result<Nat, String> {
    let req = GetBlocksArgs1 {
        start: Nat::from(0_u64),
        length: Nat::from(1_u64),
    };
    let result: Result<
        (GetTransactionsResponse,),
        (ic_cdk::api::call::RejectionCode, String)
    > = canister_call(ledger_id, "get_transactions", req).await;
    match result {
        Ok(value) => { Ok(value.0.log_length) }
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
    let tip_plus_one = tip.saturating_add(1_u64); // account for 0 index
    let blocks_needed = tip_plus_one.saturating_sub(next_block);
    let chunks_needed = (
        (blocks_needed as f64) / (MAX_TRANSACTION_BATCH_SIZE as f64)
    ).ceil() as u32;
    log("[][] ----- Starting ICRC-2 Download ----- [][]");
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
    let mut temp_tx_array: Vec<ProcessedTX> = Vec::new();
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
        let txns: Result<Vec<ProcessedTX>, String> = icrc2_download_chunk(
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

async fn icrc2_download_chunk(
    start: u64,
    length: u64,
    ledger_id: &str
) -> Result<Vec<ProcessedTX>, String> {
    let req = GetBlocksArgs1 {
        start: Nat::from(start),
        length: Nat::from(length),
    };
    let ledger_call: Result<
        (GetTransactionsResponse,),
        (ic_cdk::api::call::RejectionCode, String)
    > = canister_call(ledger_id, "get_transactions", req).await;
    match ledger_call {
        Ok(value) => {
            // check if archive call is needed
            match (value.0.transactions.is_empty(), value.0.archived_transactions.is_empty()) {
                (false, false) => {
                    // there are archive + ledger blocks needing downloaded
                    let mut all_txs: Vec<ProcessedTX> = Vec::new();
                    // loop over archives
                    for archived in value.0.archived_transactions {
                        let arve_txs = get_transactions_from_archive_t2(&archived).await; // ****
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
                    let last_block = all_txs.last();
                    let next_block: u64;
                    match last_block {
                        Some(v) => {
                            next_block = v.block.saturating_add(1);
                        }
                        None => {
                            let error = String::from("Could not get last archive block downloaded");
                            return Err(error);
                        }
                    }
                    // process ledger blocks

                    let ledger_txs: Result<Vec<ProcessedTX>, String> = process_ledger_block_t2(
                        value.0.transactions,
                        next_block
                    );
                    match ledger_txs {
                        Ok(mut v_txs) => {
                            // combine and return
                            all_txs.append(&mut v_txs);
                            return Ok(all_txs);
                        }
                        Err(e) => {
                            return Err(format!("Error processing ledger blocks: {}", e));
                        }
                    }
                }
                (false, true) => {
                    // Ledger blocks only - no archive blocks
                    let ledger_txs: Result<Vec<ProcessedTX>, String> = process_ledger_block_t2(
                        value.0.transactions,
                        start
                    );
                    match ledger_txs {
                        Ok(v_txs) => {
                            return Ok(v_txs);
                        }
                        Err(e) => {
                            return Err(format!("Error processing ledger blocks (2): {}", e));
                        }
                    }
                }
                (true, false) => {
                    // Archive blocks Only - no ledger blocks
                    let mut all_txs: Vec<ProcessedTX> = Vec::new();
                    // loop over archives
                    for archived in value.0.archived_transactions {
                        let arve_txs = get_transactions_from_archive_t2(&archived).await; // ****
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
                "Error fetching blocks from ledger (get_transactions) - {:?}. {}",
                e.0,
                e.1
            );
            return Err(error);
        }
    }
}

pub fn icrc_account_to_string(account: Account) -> String {
    let pr = account.owner.to_text();
    let sa;
    match account.subaccount {
        Some(v) => {
            sa = hex::encode(v);
        }
        None => {
            sa = DEFAULT_SUBACCOUNT.to_string();
        }
    }
    return format!("{}.{}", pr, sa);
}

async fn get_transactions_from_archive_t2(
    archived: &ArchivedRange1
) -> Result<Vec<ProcessedTX>, String> {
    let mut processed_transactions: Vec<ProcessedTX> = Vec::new();
    let req = GetBlocksArgs1 {
        start: Nat::from(archived.start.clone()),
        length: Nat::from(archived.length.clone()),
    };
    let mut master_block;
    match nat_to_u64(archived.start.clone()) {
        Ok(v_u64) => {
            master_block = v_u64;
        }
        Err(e) => {
            return Err(
                format!("Can't convert archive.start into u64 value (get_transactions_from_archive_t2) : {}", e)
            );
        }
    }
    let ledger_id = archived.callback.0.principal.to_text();
    let method = &archived.callback.0.method;
    let ledger_call: Result<
        (TransactionRange,),
        (ic_cdk::api::call::RejectionCode, String)
    > = canister_call(ledger_id.as_str(), method, req).await;
    match ledger_call {
        Ok(value) => {
            for tx in value.0.transactions {
                // MINT TX
                if let Some(mint) = tx.mint {
                    let to_ac = icrc_account_to_string(mint.to);
                    let val;
                    match nat_to_u128(mint.amount) {
                        Ok(v_u128) => {
                            val = v_u128;
                        }
                        Err(e) => {
                            return Err(
                                format!("Can't process archive tx value to u128 (1) : {}", e)
                            );
                        }
                    }
                    processed_transactions.push(ProcessedTX {
                        block: master_block.clone(),
                        hash: String::from("no-hash"),
                        tx_type: TransactionType::Mint.to_string(),
                        from_account: String::from("Token Ledger"),
                        to_account: to_ac,
                        tx_value: val,
                        tx_fee: None,
                        tx_time: tx.timestamp,
                        spender: None,
                    });
                    master_block = master_block.saturating_add(1_u64);
                }

                // BURN TX
                if let Some(burn) = tx.burn {
                    let fm_ac = icrc_account_to_string(burn.from);
                    let spend = if let Some(v) = burn.spender {
                        Some(icrc_account_to_string(v))
                    } else {
                        None
                    };
                    let val;
                    match nat_to_u128(burn.amount) {
                        Ok(v_u128) => {
                            val = v_u128;
                        }
                        Err(e) => {
                            return Err(
                                format!("Can't process archive tx value to u128 (2) : {}", e)
                            );
                        }
                    }
                    processed_transactions.push(ProcessedTX {
                        block: master_block.clone(),
                        hash: String::from("no-hash"),
                        tx_type: TransactionType::Burn.to_string(),
                        from_account: fm_ac,
                        to_account: String::from("Token Ledger"),
                        tx_value: val,
                        tx_fee: None,
                        tx_time: tx.timestamp,
                        spender: spend,
                    });
                    master_block = master_block.saturating_add(1_u64);
                }

                // TRANSFER TX
                if let Some(transfer) = tx.transfer {
                    let to_ac = icrc_account_to_string(transfer.to);
                    let fm_ac = icrc_account_to_string(transfer.from);
                    let spend = if let Some(v) = transfer.spender {
                        Some(icrc_account_to_string(v))
                    } else {
                        None
                    };
                    let fee = if let Some(f) = transfer.fee {
                        match nat_to_u128(f) {
                            Ok(f_u128) => { Some(f_u128) }
                            Err(e) => {
                                return Err(
                                    format!("Can't process transaction fee to u128 : {}", e)
                                );
                            }
                        }
                    } else {
                        None
                    };
                    let val;
                    match nat_to_u128(transfer.amount) {
                        Ok(v_u128) => {
                            val = v_u128;
                        }
                        Err(e) => {
                            return Err(
                                format!("Can't process archive tx value to u128 (3) : {}", e)
                            );
                        }
                    }
                    processed_transactions.push(ProcessedTX {
                        block: master_block.clone(),
                        hash: String::from("no-hash"),
                        tx_type: TransactionType::Transfer.to_string(),
                        from_account: fm_ac,
                        to_account: to_ac,
                        tx_value: val,
                        tx_fee: fee,
                        tx_time: tx.timestamp,
                        spender: spend,
                    });
                    master_block = master_block.saturating_add(1_u64);
                }

                // APPROVE TX
                if let Some(approve) = tx.approve {
                    let fm_ac = icrc_account_to_string(approve.from);
                    let spend = icrc_account_to_string(approve.spender);
                    let fee = if let Some(f) = approve.fee {
                        match nat_to_u128(f) {
                            Ok(f_u128) => { Some(f_u128) }
                            Err(e) => {
                                return Err(
                                    format!("Can't process transaction fee to u128 : {}", e)
                                );
                            }
                        }
                    } else {
                        None
                    };
                    let val;
                    match nat_to_u128(approve.amount) {
                        Ok(v_u128) => {
                            val = v_u128;
                        }
                        Err(_e) => {
                            val = u128::MAX;
                            // fix eth issue. accurate Val isn't mission critical for approve txs.
                            //return Err(format!("Can't process archive tx value to u128 (4) : {}", e))
                        }
                    }

                    processed_transactions.push(ProcessedTX {
                        block: master_block.clone(),
                        hash: String::from("no-hash"),
                        tx_type: TransactionType::Approve.to_string(),
                        from_account: fm_ac,
                        to_account: spend.clone(),
                        tx_value: val,
                        tx_fee: fee,
                        tx_time: tx.timestamp,
                        spender: Some(spend),
                    });
                    master_block = master_block.saturating_add(1_u64);
                }
            } // for loop
        }
        Err(e) => {
            let error = format!("Error fetching archive blocks (2) - {:?}. {}", e.0, e.1);
            return Err(error);
        }
    }
    return Ok(processed_transactions);
}

fn process_ledger_block_t2(
    txs: Vec<Transaction>,
    next_block_number: u64
) -> Result<Vec<ProcessedTX>, String> {
    let mut master_block = next_block_number;
    let mut processed_transactions: Vec<ProcessedTX> = Vec::new();

    for tx in txs {
        // MINT TX
        if let Some(mint) = tx.mint {
            let to_ac = icrc_account_to_string(mint.to);
            let val;
            match nat_to_u128(mint.amount) {
                Ok(v_u128) => {
                    val = v_u128;
                }
                Err(e) => {
                    return Err(format!("Can't process archive tx value to u128 (1) : {}", e));
                }
            }
            processed_transactions.push(ProcessedTX {
                block: master_block.clone(),
                hash: String::from("no-hash"),
                tx_type: TransactionType::Mint.to_string(),
                from_account: String::from("Token Ledger"),
                to_account: to_ac,
                tx_value: val,
                tx_fee: None,
                tx_time: tx.timestamp,
                spender: None,
            });
            master_block = master_block.saturating_add(1_u64);
        }

        // BURN TX
        if let Some(burn) = tx.burn {
            let fm_ac = icrc_account_to_string(burn.from);
            let spend = if let Some(v) = burn.spender {
                Some(icrc_account_to_string(v))
            } else {
                None
            };
            let val;
            match nat_to_u128(burn.amount) {
                Ok(v_u128) => {
                    val = v_u128;
                }
                Err(e) => {
                    return Err(format!("Can't process archive tx value to u128 (2) : {}", e));
                }
            }
            processed_transactions.push(ProcessedTX {
                block: master_block.clone(),
                hash: String::from("no-hash"),
                tx_type: TransactionType::Burn.to_string(),
                from_account: fm_ac,
                to_account: String::from("Token Ledger"),
                tx_value: val,
                tx_fee: None,
                tx_time: tx.timestamp,
                spender: spend,
            });
            master_block = master_block.saturating_add(1_u64);
        }

        // TRANSFER TX
        if let Some(transfer) = tx.transfer {
            let to_ac = icrc_account_to_string(transfer.to);
            let fm_ac = icrc_account_to_string(transfer.from);
            let spend = if let Some(v) = transfer.spender {
                Some(icrc_account_to_string(v))
            } else {
                None
            };
            let fee = if let Some(f) = transfer.fee {
                match nat_to_u128(f) {
                    Ok(f_u128) => { Some(f_u128) }
                    Err(e) => {
                        return Err(format!("Can't process transaction fee to u128 : {}", e));
                    }
                }
            } else {
                None
            };
            let val;
            match nat_to_u128(transfer.amount) {
                Ok(v_u128) => {
                    val = v_u128;
                }
                Err(e) => {
                    return Err(format!("Can't process archive tx value to u128 (3) : {}", e));
                }
            }
            processed_transactions.push(ProcessedTX {
                block: master_block.clone(),
                hash: String::from("no-hash"),
                tx_type: TransactionType::Transfer.to_string(),
                from_account: fm_ac,
                to_account: to_ac,
                tx_value: val,
                tx_fee: fee,
                tx_time: tx.timestamp,
                spender: spend,
            });
            master_block = master_block.saturating_add(1_u64);
        }

        // APPROVE TX
        if let Some(approve) = tx.approve {
            let fm_ac = icrc_account_to_string(approve.from);
            let spend = icrc_account_to_string(approve.spender);
            let fee = if let Some(f) = approve.fee {
                match nat_to_u128(f) {
                    Ok(f_u128) => { Some(f_u128) }
                    Err(e) => {
                        return Err(format!("Can't process transaction fee to u128 : {}", e));
                    }
                }
            } else {
                None
            };
            let val;
            match nat_to_u128(approve.amount) {
                Ok(v_u128) => {
                    val = v_u128;
                }
                Err(e) => {
                    return Err(format!("Can't process archive tx value to u128 (4) : {}", e));
                }
            }

            processed_transactions.push(ProcessedTX {
                block: master_block.clone(),
                hash: String::from("no-hash"),
                tx_type: TransactionType::Approve.to_string(),
                from_account: fm_ac,
                to_account: spend.clone(),
                tx_value: val,
                tx_fee: fee,
                tx_time: tx.timestamp,
                spender: Some(spend),
            });
            master_block = master_block.saturating_add(1_u64);
        }
    } // for loop
    return Ok(processed_transactions);
}
