use std::collections::VecDeque;

use super_stats_v3_api::{
    custom_types::{ IndexerType, ProcessedTX, TimeChunkStats, TimeStats, TotCntAvg },
    process_data::process_time_stats::StatsType,
    runtime::RUNTIME_STATE,
    stats::constants::{ DAY_AS_NANOS, HOUR_AS_NANOS },
};

use crate::{
    core::utils::log,
    stats::utils::{ nearest_day_start, nearest_past_hour, parse_icrc_account },
};

pub fn calculate_time_stats(
    process_from: u64,
    mode: StatsType,
    ledger_type: IndexerType,
    time_now: u64
) -> TimeStats {
    let array: VecDeque<ProcessedTX> = RUNTIME_STATE.with(|s| {
        s.borrow().data.latest_blocks.blocks.clone()
    });

    if array.len() == 0 {
        log("Blockstore is empty: Returning empty TimeStats!");
        return TimeStats::default();
    }

    let mut all_accounts: Vec<String> = Vec::new();
    let mut all_principals: Vec<String> = Vec::new();
    let mut mint_count: u128 = 0;
    let mut mint_value: u128 = 0;
    let mut burn_count: u128 = 0;
    let mut burn_value: u128 = 0;
    let mut transfer_count: u128 = 0;
    let mut transfer_value: u128 = 0;
    let mut approve_count: u128 = 0;
    let mut approve_value: u128 = 0;
    let mut total_value: u128 = 0;
    let mut total_txs: u128 = 0;
    let mut all_mints: Vec<ProcessedTX> = Vec::new();
    let mut all_burns: Vec<ProcessedTX> = Vec::new();
    let mut all_transfers: Vec<ProcessedTX> = Vec::new();

    for tx in &array {
        if tx.tx_time >= process_from {
            // Get All Account/ Principals
            match ledger_type {
                IndexerType::DfinityIcp => {
                    if tx.from_account != "Token Ledger" {
                        all_accounts.push(tx.from_account.clone());
                    }
                    if tx.to_account != "Token Ledger" {
                        all_accounts.push(tx.to_account.clone());
                    }
                }
                IndexerType::DfinityIcrc2 => {
                    if tx.from_account != "Token Ledger" {
                        let fm_parse = parse_icrc_account(&tx.from_account);
                        if let Some(v) = fm_parse {
                            all_principals.push(v.0);
                            all_accounts.push(v.1);
                        }
                    }
                    if tx.to_account != "Token Ledger" {
                        let to_parse = parse_icrc_account(&tx.to_account);
                        if let Some(v) = to_parse {
                            all_principals.push(v.0);
                            all_accounts.push(v.1);
                        }
                    }
                }
                IndexerType::DfinityIcrc3 => {} // TO-DO!
            } // match

            // COUNT MINT
            if tx.tx_type == "Mint" {
                mint_count = mint_count.saturating_add(1);
                mint_value = mint_value.saturating_add(tx.tx_value);
                all_mints.push(tx.clone());
            }

            // COUNT BURN
            if tx.tx_type == "Burn" {
                burn_count = burn_count.saturating_add(1);
                burn_value = burn_value.saturating_add(tx.tx_value);
                all_burns.push(tx.clone());
            }

            // COUNT TRANSFER
            if tx.tx_type == "Transfer" {
                transfer_count = transfer_count.saturating_add(1);
                transfer_value = transfer_value.saturating_add(tx.tx_value);
                all_transfers.push(tx.clone());
            }

            //COUNT APPROVE
            if tx.tx_type == "Approve" {
                approve_count = approve_count.saturating_add(1);
                approve_value = approve_value.saturating_add(tx.tx_value);
            }

            // COUNT TOTAL VALUE OF TXS (excluding approve txs!)
            if tx.tx_type != "Approve" {
                total_value = total_value.saturating_add(tx.tx_value);
            }

            // TX Count
            total_txs = total_txs.saturating_add(1);
        } // end if >= process_from
    } // end loop

    // VOLUMES PER TIME CHUNK
    let mut count_over_time: Vec<TimeChunkStats> = Vec::new();
    count_over_time = calculate_time_chunk_stats(
        time_now.clone(),
        process_from.clone(),
        mode.clone(),
        &array
    );

    // LARGEST BURN/ TX ETC
    let return_len = RUNTIME_STATE.with(|s| { s.borrow().data.max_return_length });
    let top_mints: Vec<ProcessedTX> = top_x_by_txvalue(all_mints, return_len);
    let top_burns: Vec<ProcessedTX> = top_x_by_txvalue(all_burns, return_len);
    let top_transfers: Vec<ProcessedTX> = top_x_by_txvalue(all_transfers, return_len);
    let unique_accounts: Vec<String> = get_unique_string_values(all_accounts);
    let unique_principals: Vec<String> = if all_principals.len() > 0 {
        get_unique_string_values(all_principals)
    } else {
        Vec::new()
    };
    let ua: &usize = &unique_accounts.len();
    let up: &usize = &unique_principals.len();

    let ret = TimeStats {
        total_transaction_count: total_txs,
        total_transaction_value: total_value,
        total_transaction_average: (total_value as f64) / (total_txs as f64),
        total_unique_accounts: ua.to_owned() as u64,
        total_unique_principals: up.to_owned() as u64,
        most_active_accounts: Vec::new(),
        most_active_principals: Vec::new(),
        burn_stats: TotCntAvg {
            total_value: burn_value,
            count: burn_count,
            average: if burn_count != 0 {
                (burn_value as f64) / (burn_count as f64)
            } else {
                0_f64
            },
        },
        mint_stats: TotCntAvg {
            total_value: mint_value,
            count: mint_count,
            average: if mint_count != 0 {
                (mint_value as f64) / (mint_count as f64)
            } else {
                0_f64
            },
        },
        transfer_stats: TotCntAvg {
            total_value: transfer_value,
            count: transfer_count,
            average: if transfer_count != 0 {
                (transfer_value as f64) / (transfer_count as f64)
            } else {
                0_f64
            },
        },
        approve_stats: TotCntAvg {
            total_value: approve_value,
            count: approve_count,
            average: if approve_count != 0 {
                (approve_value as f64) / (approve_count as f64)
            } else {
                0_f64
            },
        },
        count_over_time,
        top_mints,
        top_burns,
        top_transfers,
    };

    return ret;
}

fn calculate_time_chunk_stats(
    time_now: u64,
    process_from: u64,
    mode: StatsType,
    txs: &VecDeque<ProcessedTX>
) -> Vec<TimeChunkStats> {
    // return early if empty
    if txs.len() == 0 {
        let def = TimeChunkStats::default();
        let mut v = Vec::new();
        v.push(def);
        return v;
    }

    let mut count_over_time: Vec<TimeChunkStats> = Vec::new();
    let chunks_needed: u32;
    let nearest_past_x: u64;
    let x_in_nanos: u64;
    let mut start_chunk: u64 = 0_u64;
    let mut end_chunk: u64;
    let mut tx_count_chunk: u64;
    let mut mint_count_chunk: u64;
    let mut burn_count_chunk: u64;
    let mut transfer_count_chunk: u64;
    let mut approve_count_chunk: u64;

    match mode {
        StatsType::Hourly => {
            chunks_needed = (
                ((time_now - process_from) as f64) / (HOUR_AS_NANOS as f64)
            ).ceil() as u32;
            nearest_past_x = nearest_past_hour(time_now);
            x_in_nanos = HOUR_AS_NANOS;
        }
        StatsType::Daily => {
            chunks_needed = (
                ((time_now - process_from) as f64) / (DAY_AS_NANOS as f64)
            ).ceil() as u32;
            nearest_past_x = nearest_day_start(time_now);
            x_in_nanos = DAY_AS_NANOS;
        }
    }

    for i in 0..chunks_needed {
        // Set start/ end times
        if i == 0 {
            start_chunk = if time_now == nearest_past_x {
                nearest_past_x - x_in_nanos
            } else {
                nearest_past_x
            };
            end_chunk = time_now;
        } else {
            end_chunk = start_chunk;
            start_chunk = start_chunk - x_in_nanos;
        }

        // clear counts
        tx_count_chunk = 0;
        mint_count_chunk = 0;
        burn_count_chunk = 0;
        transfer_count_chunk = 0;
        approve_count_chunk = 0;

        for tx in txs {
            if tx.tx_time >= start_chunk && tx.tx_time < end_chunk {
                tx_count_chunk = tx_count_chunk.saturating_add(1);
                if tx.tx_type == "Mint" {
                    mint_count_chunk = mint_count_chunk.saturating_add(1);
                }
                if tx.tx_type == "Burn" {
                    burn_count_chunk = burn_count_chunk.saturating_add(1);
                }
                if tx.tx_type == "Transfer" {
                    transfer_count_chunk = transfer_count_chunk.saturating_add(1);
                }
                if tx.tx_type == "Approve" {
                    approve_count_chunk = approve_count_chunk.saturating_add(1);
                }
            }
            if tx.tx_time > end_chunk {
                break;
            }
        }

        let tcs: TimeChunkStats = TimeChunkStats {
            start_time: start_chunk,
            end_time: end_chunk,
            total_count: tx_count_chunk,
            mint_count: mint_count_chunk,
            transfer_count: transfer_count_chunk,
            burn_count: burn_count_chunk,
            approve_count: approve_count_chunk,
        };
        count_over_time.push(tcs);
    } // end loop

    return count_over_time;
}

pub fn top_x_by_txvalue(
    mut transactions: Vec<ProcessedTX>,
    result_length: usize
) -> Vec<ProcessedTX> {
    // decending
    transactions.sort_by(|a, b| b.tx_value.cmp(&a.tx_value));
    if transactions.len() > result_length {
        transactions.truncate(result_length);
    }
    return transactions;
}

pub fn top_x_txcount(
    mut transactions: Vec<(String, u64)>,
    result_length: usize
) -> Vec<(String, u64)> {
    // decending
    transactions.sort_by(|a, b| b.1.cmp(&a.1));
    if transactions.len() > result_length {
        transactions.truncate(result_length);
    }
    return transactions;
}

pub fn get_unique_string_values(vec: Vec<String>) -> Vec<String> {
    if vec.len() == 0 {
        return Vec::new();
    }

    let mut working_array: Vec<String> = vec.to_owned();
    let mut keepers: Vec<String> = Vec::new();
    working_array.sort();
    keepers.push(working_array[0].to_owned()); // 1st is always a keeeper
    for i in 1..working_array.len() {
        if working_array[i] != working_array[i - 1] {
            keepers.push(working_array[i].to_owned());
        }
    }
    return keepers;
}
