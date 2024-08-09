use super_stats_v3_api::{
    core::constants::D1_AS_NANOS, custom_types::{ IndexerType, ProcessedTX, SmallTX }, runtime::RUNTIME_STATE, stable_memory::STABLE_STATE
};

use crate::stats::{
    active_accounts::{
        get_count_of_unique_accounts,
        init_activity_stats,
        push_activity_snapshot,
        push_padding_snapshot,
    },
    directory::add_to_directory,
    utils::parse_icrc_account,
};

pub fn processedtx_to_smalltx(input_vec: &Vec<ProcessedTX>) -> Vec<SmallTX> {
    // Vars for calculating simple activity stats (active_accounts.rs)
    let mut activity_start_time = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().activity_stats.chunk_start_time.clone()
    });
    let mut activity_end_time = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().activity_stats.chunk_end_time.clone()
    });
    let mut accounts_directory_refs: Vec<Option<u64>> = Vec::new();
    let mut principals_directory_refs: Vec<Option<u64>> = Vec::new();

    // Process smallTX
    let mut stx: Vec<SmallTX> = Vec::new();
    for tx in input_vec {
        // init activity stats on first block
        if tx.block == 0 {
            activity_start_time = tx.tx_time.clone();
            activity_end_time = init_activity_stats(tx.tx_time.clone());
        }

        // get refs for from/ to accounts
        let fm: Option<u64>;
        let fm_ac = tx.from_account.as_str();
        if fm_ac != "Token Ledger" {
            fm = add_to_directory(&tx.from_account);
        } else {
            fm = None;
        }

        let to: Option<u64>;
        let to_ac = tx.to_account.as_str();
        if to_ac != "Token Ledger" {
            to = add_to_directory(&tx.to_account);            
        } else {
            to = None;
        }

        let fee: Option<u128>;
        if let Some(fee_value) = tx.tx_fee.clone() {
            fee = Some(fee_value);
        } else {
            fee = None;
        }

        let tx_type: u8;
        match tx.tx_type.as_str() {
            "Transfer" => {
                tx_type = 0;
            }
            "Mint" => {
                tx_type = 1;
            }
            "Burn" => {
                tx_type = 2;
            }
            "Approve" => {
                tx_type = 3;
            }
            _ => {
                tx_type = 99;
            }
        }

        stx.push(SmallTX {
            block: tx.block as u64,
            time: tx.tx_time,
            from: fm.clone(),
            to: to.clone(),
            tx_type,
            value: tx.tx_value.clone(),
            fee,
        });

        // Add the `to` account to account_data and principal_data
        // so the snapshot can count correctly in the code below
        if let Some(to_u64) = to {
            STABLE_STATE.with(|s| {
                s.borrow_mut()
                    .as_mut()
                    .unwrap()
                    .account_data.create_account_if_not_exists(&to_u64, tx.tx_time)
            });
            let index_type = RUNTIME_STATE.with(|s| { s.borrow().data.get_index_type() });

            if index_type != IndexerType::DfinityIcp {
                match parse_icrc_account(&tx.to_account) {
                    Some(parsed_from) => {
                        match add_to_directory(&parsed_from.0) {
                            Some(from_as_principal) => {
                                STABLE_STATE.with(|s| {
                                    s.borrow_mut()
                                        .as_mut()
                                        .unwrap()
                                        .principal_data.create_account_if_not_exists(
                                            &from_as_principal,
                                            tx.tx_time
                                        )
                                });
                            }
                            None => {}
                        };
                    }
                    None => {}
                }
            }
        }
        // check for end of simple stats 'window'
        if tx.tx_time > activity_end_time {
            // update final numbers
            let unique_accounts = get_count_of_unique_accounts(accounts_directory_refs.clone());
            let unique_principals = get_count_of_unique_accounts(principals_directory_refs.clone());
            STABLE_STATE.with(|s| {
                s.borrow_mut()
                    .as_mut()
                    .unwrap()
                    .activity_stats.add_account_to_current_snapshot(unique_accounts);
                s.borrow_mut()
                    .as_mut()
                    .unwrap()
                    .activity_stats.add_principal_to_current_snapshot(unique_principals);
            });

            // next tx is within 24 hours of last
            if tx.tx_time < activity_end_time + D1_AS_NANOS {
                // add snapshot to stable memory store and update window times, update activity end time
                (activity_start_time, activity_end_time) = push_activity_snapshot();
            } else {
                // pad missing snapshots
                let time_since = tx.tx_time - activity_end_time;
                let time_diff = (time_since as f64) / (D1_AS_NANOS as f64);
                let missing_days = time_diff.ceil() as usize;
                let mut new_times: (u64, u64) = (activity_start_time, activity_end_time);
                for _i in 0..missing_days {
                    new_times = push_padding_snapshot(new_times.0, new_times.1);
                }
                (activity_start_time, activity_end_time) = (new_times.0, new_times.1);
            }

            // clear all_directory_refs
            accounts_directory_refs.clear();
            principals_directory_refs.clear();
        }

        // for calculating unique active accounts
        if tx.tx_time >= activity_start_time && tx.tx_time < activity_end_time {
            // We push from and to as accounts
            accounts_directory_refs.push(fm);
            accounts_directory_refs.push(to);

            // We push from and to as principals
            match parse_icrc_account(&tx.from_account) {
                Some(parsed_from) => {
                    match add_to_directory(&parsed_from.0) {
                        Some(from_as_principal) =>
                            principals_directory_refs.push(Some(from_as_principal)),
                        None => {}
                    };
                }
                None => {}
            }

            match parse_icrc_account(&tx.to_account) {
                Some(parsed_to) => {
                    match add_to_directory(&parsed_to.0) {
                        Some(to_as_principal) =>
                            principals_directory_refs.push(Some(to_as_principal)),
                        None => {}
                    };
                }
                None => {}
            }
        }
    } // for

    // update count so far
    let unique_accounts = get_count_of_unique_accounts(accounts_directory_refs);
    let unique_principals = get_count_of_unique_accounts(principals_directory_refs);
    STABLE_STATE.with(|s| {
        s.borrow_mut()
            .as_mut()
            .unwrap()
            .activity_stats.add_account_to_current_snapshot(unique_accounts);
        s.borrow_mut()
            .as_mut()
            .unwrap()
            .activity_stats.add_principal_to_current_snapshot(unique_principals);
    });

    return stx;
}

pub fn processedtx_to_principal_only_smalltx(input_vec: &Vec<ProcessedTX>) -> Vec<SmallTX> {
    let mut stx: Vec<SmallTX> = Vec::new();
    for tx in input_vec {
        // get refs for from/ to accounts
        let fm: Option<u64>;
        let fm_ac = tx.from_account.as_str();
        if fm_ac != "Token Ledger" {
            if tx.from_account.contains(".") {
                let parse = parse_icrc_account(&tx.from_account).unwrap();
                fm = add_to_directory(&parse.0);
            } else {
                fm = None;
            }
        } else {
            fm = None;
        }

        let to: Option<u64>;
        let to_ac = tx.to_account.as_str();
        if to_ac != "Token Ledger" {
            if tx.to_account.contains(".") {
                let parse = parse_icrc_account(&tx.to_account).unwrap();
                to = add_to_directory(&parse.0);
            } else {
                to = None;
            }
        } else {
            to = None;
        }

        let fee: Option<u128>;
        if let Some(fee_value) = tx.tx_fee.clone() {
            fee = Some(fee_value);
        } else {
            fee = None;
        }

        let tx_type: u8;
        match tx.tx_type.as_str() {
            "Transfer" => {
                tx_type = 0;
            }
            "Mint" => {
                tx_type = 1;
            }
            "Burn" => {
                tx_type = 2;
            }
            "Approve" => {
                tx_type = 3;
            }
            _ => {
                tx_type = 99;
            }
        }

        stx.push(SmallTX {
            block: tx.block as u64,
            time: tx.tx_time,
            from: fm,
            to: to,
            tx_type,
            value: tx.tx_value.clone(),
            fee,
        });
    }
    return stx;
}
