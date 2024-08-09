use ic_cdk::export_candid;

pub mod core;
pub mod stats;
pub mod timers;
mod test_data;

use stats::queries::*;
use stats::updates::*;
use core::api::*;
use core::init_and_upgrade::*;

export_candid!();

mod tests {
    use super_stats_v3_api::{
        account_tree::AccountTree,
        core::constants::D1_AS_NANOS,
        custom_types::{ IndexerType, ProcessedTX, SmallTX, TransactionType },
        fetch_data::dfinity_icrc2::DEFAULT_SUBACCOUNT,
        process_data::process_time_stats::StatsType,
        runtime::RUNTIME_STATE,
        stable_memory::STABLE_STATE,
        stats::constants::DAY_AS_NANOS,
        types::IDKey,
    };

    use crate::{
        stats::{
            process_data::{
                process_index::process_smtx_to_index,
                process_time_stats::{ calculate_time_stats, top_x_by_txvalue },
                small_tx::processedtx_to_smalltx,
            },
            utils::{
                nearest_day_start,
                nearest_past_hour,
                parse_icrc_account,
                principal_subaccount_to_string,
            },
        },
        test_data::{ ptx_test_data, test_state_init },
    };

    #[test]
    fn test_string_to_key() {
        let input: String =
            "8c79044b039ee8afbf1e8cc679d93cdfddfdf28710691aa9c81b85d7ef253206".to_string();
        let as_key: IDKey = IDKey::from_string(&input).unwrap();
        let output: String = as_key.to_string().unwrap();
        assert_eq!(input, output);

        let input2: String =
            "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe.0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let as_key2: IDKey = IDKey::from_string(&input2).unwrap();
        let output2: String = as_key2.to_string().unwrap();
        assert_eq!(input2, output2);

        let input3: String = "q6osm".to_string();
        let as_key3: IDKey = IDKey::from_string(&input3).unwrap();
        let output3: String = as_key3.to_string().unwrap();
        assert_eq!(input3, output3);
    }

    #[test]
    fn test_process_to_small_tx_format() {
        // init test Stable/ Runtime state
        test_state_init();

        let ptx = ptx_test_data();
        let stx = processedtx_to_smalltx(&ptx);

        // TRANSACTION TYPE
        // Processed TX 10
        let first_ptx = ptx[10].clone();
        // Small TX 10
        let first_stx = stx[10].clone();

        // from account to u32 ref (using Directory)
        let id_ref_from = STABLE_STATE.with(|s| {
            s.borrow().as_ref().unwrap().directory_data.get_ref(&first_ptx.from_account)
        });

        // to account to u32 ref (using Directory)
        let id_ref_to = STABLE_STATE.with(|s| {
            s.borrow().as_ref().unwrap().directory_data.get_ref(&first_ptx.to_account)
        });

        // check from ac on Small TX = from ac on Processed TX
        assert_eq!(first_stx.from, id_ref_from);
        // check to ac on Small TX = to ac on Processed TX
        assert_eq!(first_stx.to, id_ref_to);
        // check time
        assert_eq!(first_stx.time, first_ptx.tx_time);
        // check type
        assert_eq!(first_stx.tx_type, 0_u8); // 0 = transfer, 1 = Mint, 2 = Burn. 3 = approve.
        // check value
        assert_eq!(first_stx.value, first_ptx.tx_value);
        // check block
        assert_eq!(first_stx.block, first_ptx.block);

        // MINT TYPE - TX0
        let mint_ptx = ptx[0].clone();
        // Small TX 10
        let mint_stx = stx[0].clone();

        // from account to u32 ref (using Directory)
        let mint_ref_from = STABLE_STATE.with(|s| {
            s.borrow().as_ref().unwrap().directory_data.get_ref(&mint_ptx.from_account)
        });

        // to account to u32 ref (using Directory)
        let mint_ref_to = STABLE_STATE.with(|s| {
            s.borrow().as_ref().unwrap().directory_data.get_ref(&mint_ptx.to_account)
        });

        // check from ac on Small TX = from ac on Processed TX
        assert_eq!(mint_stx.from, mint_ref_from);
        // check to ac on Small TX = to ac on Processed TX
        assert_eq!(mint_stx.to, mint_ref_to);
        // check time
        assert_eq!(mint_stx.time, mint_ptx.tx_time);
        // check type
        assert_eq!(mint_stx.tx_type, 1_u8); // 0 = transfer, 1 = Mint, 2 = Burn. 3 = approve.
        // check value
        assert_eq!(mint_stx.value, mint_ptx.tx_value);
        // check block
        assert_eq!(mint_stx.block, mint_ptx.block);

        // // BURN TYPE
        let burn_ptx = ptx[16].clone();
        // Small TX 10
        let burn_stx = stx[16].clone();

        // from account to u32 ref (using Directory)
        let burn_ref_from = STABLE_STATE.with(|s| {
            s.borrow().as_ref().unwrap().directory_data.get_ref(&burn_ptx.from_account)
        });

        // to account to u32 ref (using Directory)
        let burn_ref_to = STABLE_STATE.with(|s| {
            s.borrow().as_ref().unwrap().directory_data.get_ref(&burn_ptx.to_account)
        });

        // check from ac on Small TX = from ac on Processed TX
        assert_eq!(burn_stx.from, burn_ref_from);
        // check to ac on Small TX = to ac on Processed TX
        assert_eq!(burn_stx.to, burn_ref_to);
        // check time
        assert_eq!(burn_stx.time, burn_ptx.tx_time);
        // check type
        assert_eq!(burn_stx.tx_type, 2_u8); // 0 = transfer, 1 = Mint, 2 = Burn. 3 = approve.
        // check value
        assert_eq!(burn_stx.value, burn_ptx.tx_value);
        // check block
        assert_eq!(burn_stx.block, burn_ptx.block);

        // // APPROVE TYPE
        let ap_ptx = ptx[30].clone();
        // Small TX 10
        let ap_stx = stx[30].clone();

        // from account to u32 ref (using Directory)
        let ap_ref_from = STABLE_STATE.with(|s| {
            s.borrow().as_ref().unwrap().directory_data.get_ref(&ap_ptx.from_account)
        });

        // to account to u32 ref (using Directory)
        let ap_ref_to = STABLE_STATE.with(|s| {
            s.borrow().as_ref().unwrap().directory_data.get_ref(&ap_ptx.to_account)
        });

        // check from ac on Small TX = from ac on Processed TX
        assert_eq!(ap_stx.from, ap_ref_from);
        // check to ac on Small TX = to ac on Processed TX
        assert_eq!(ap_stx.to, ap_ref_to);
        // check time
        assert_eq!(ap_stx.time, ap_ptx.tx_time);
        // check type
        assert_eq!(ap_stx.tx_type, 3_u8); // 0 = transfer, 1 = Mint, 2 = Burn. 3 = approve.
        // check value
        assert_eq!(ap_stx.value, ap_ptx.tx_value);
        // check block
        assert_eq!(ap_stx.block, ap_ptx.block);

        // check input length == output length.
        assert_eq!(ptx.len(), stx.len());
    }

    #[test]
    fn test_nearest_past_hour() {
        //  Input already on an hour boundary
        let time_ms_1: u64 = 1687856400000000000; // 27/06/23 0900 gmt
        assert_eq!(nearest_past_hour(time_ms_1), time_ms_1);

        //  Input at half past hour
        let time_ms_2: u64 = 1687858200000000000; // 09:30:00 gmt
        assert_eq!(nearest_past_hour(time_ms_2), 1687856400000000000); // 27/06/23 0900 gmt

        // 1 nano before the hour
        let time_ms_3: u64 = 1687856399999999999;
        assert_eq!(nearest_past_hour(time_ms_3), 1687852800000000000); // 27/06/23 0800 gmt

        //  1 nano before end of the hour
        let time_ms_4: u64 = 1687859999999999999;
        assert_eq!(nearest_past_hour(time_ms_4), 1687856400000000000); // 27/06/23 0900 gmt
    }

    #[test]
    fn test_nearest_past_day() {
        //  Input already on an day boundary
        let time_ms_1: u64 = 1687824000000000000; // 27/06/23 0000 gmt
        assert_eq!(nearest_day_start(time_ms_1), time_ms_1);

        //  Input at mid-day (12:00)
        let time_ms_2: u64 = 1687867200000000000;
        assert_eq!(nearest_day_start(time_ms_2), time_ms_1); // 27/06/23 0000 gmt

        // 1 nano after midnight
        let time_ms_3: u64 = 1687824000000000001;
        assert_eq!(nearest_day_start(time_ms_3), time_ms_1); // 27/06/23 0000 gmt

        //  1 nano before next day
        let time_ms_4: u64 = 1687910399999999999;
        assert_eq!(nearest_day_start(time_ms_4), time_ms_1); // 27/06/23 0000 gmt
    }

    #[test]
    fn test_combine_ps() {
        let pr = String::from("2vxsx-fae");
        let sa = DEFAULT_SUBACCOUNT.to_string();
        let st = principal_subaccount_to_string(pr, sa);
        let res = String::from(
            "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(res, st);
    }

    #[test]
    fn test_parse_account() {
        let inpt = String::from(
            "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000"
        );
        let output = parse_icrc_account(&inpt).unwrap();
        let pr = output.0;
        let ac = output.1;
        assert_eq!(pr, String::from("2vxsx-fae"));
        assert_eq!(
            ac,
            String::from("0000000000000000000000000000000000000000000000000000000000000000")
        );
    }

    // ICP LEDGER TYPE - 30 days
    // #[test]
    fn process_daily_data() {
        // set hourly time
        RUNTIME_STATE.with(|s| {
            s.borrow_mut().data.latest_blocks.hours_nano = 30_u64 * DAY_AS_NANOS;
        });
        // set return len
        RUNTIME_STATE.with(|s| {
            s.borrow_mut().data.max_return_length = 5;
        });
        let time_now = 1_688_888_888_889_999_888;
        let process_from = RUNTIME_STATE.with(|s| {
            let hr_nanos = s.borrow().data.latest_blocks.hours_nano;
            return time_now - hr_nanos;
        });

        let test_txs = ptx_test_data();
        RUNTIME_STATE.with(|s| { s.borrow_mut().data.latest_blocks.push_tx_vec(test_txs) });

        let res = calculate_time_stats(
            process_from,
            StatsType::Daily,
            IndexerType::DfinityIcp,
            time_now
        );

        println!("RES :: {:?}", res);
        // totals
        assert_eq!(res.total_transaction_count, 31);
        assert_eq!(res.total_transaction_value, 1_004_132_500_001);
        assert_eq!(res.total_transaction_average, 32391370967.774193);

        // Mint/ Burn/ Transfer/ Approve stats
        let sum_all =
            res.burn_stats.total_value +
            res.mint_stats.total_value +
            res.transfer_stats.total_value +
            res.approve_stats.total_value;

        let count_all =
            res.burn_stats.count +
            res.mint_stats.count +
            res.transfer_stats.count +
            res.approve_stats.count;

        assert_eq!(sum_all, 1_005_132_500_001); // 1_000_000_000 higher than total_transaction_value b/c approve is counted
        assert_eq!(count_all, 31);

        // Count over time
        let cot = res.count_over_time.clone();
        assert_eq!(cot.len(), 30); // 30 days of data

        let mut mint_count = 0;
        let mut burn_count = 0;
        let mut trsfr_count = 0;
        let mut approve_count = 0;
        for cnk in cot {
            mint_count += cnk.mint_count;
            burn_count += cnk.burn_count;
            trsfr_count += cnk.transfer_count;
            approve_count += cnk.approve_count;
        }
        let total_cnk = mint_count + burn_count + trsfr_count + approve_count;
        assert_eq!(total_cnk, 31); // all txs accounted for
    }

    #[test]
    fn test_top_by_value() {
        let txs = ptx_test_data();
        let mut trsf: Vec<ProcessedTX> = Vec::new();
        for tx in txs {
            if tx.tx_type == "Transfer" {
                trsf.push(tx);
            }
        }
        let top = top_x_by_txvalue(trsf, 5);

        assert_eq!(top[0].tx_value, 500000000);
        assert_eq!(top[1].tx_value, 100000000);
        assert_eq!(top[2].tx_value, 100000000);
        assert_eq!(top[3].tx_value, 80000000);
        assert_eq!(top[4].tx_value, 30000000);
    }

    #[test]
    fn test_update_account_balance() {
        test_state_init();

        let mut account_tree = AccountTree::default();

        let account1 = 1000u64;
        let account2 = 1002u64;
        let account3 = 1003u64;

        // Day 1
        // account1: 100_000_000_000
        // account2: 0
        // account3: 0

        let stx1 = SmallTX {
            block: 0,
            time: 86_400 * 1_000_000_000, // Day 1
            tx_type: 1u8, // Mint
            value: 100_000_000_000,
            fee: Some(0),
            to: Some(account1),
            from: None,
        };
        assert_eq!(
            account_tree.process_transfer_to(&account1, &stx1),
            Ok("Transfer Processed".to_string())
        );
        account_tree.accounts_history.debug_print();
        let day1_acc1_history = account_tree.accounts_history
            .get(&(account1, 1))
            .unwrap()
            .to_owned();
        assert_eq!(day1_acc1_history.balance, 100_000_000_000);

        // Day 2
        // account1: 90_000_000_000
        // account2: 10_000_000_000
        // account3: 0
        let stx2 = SmallTX {
            block: 1,
            time: 2 * 86_400 * 1_000_000_000, // Day 2
            tx_type: 0u8, // Transfer
            value: 10_000_000_000,
            fee: Some(0),
            to: Some(account2),
            from: Some(account1),
        };
        assert_eq!(
            account_tree.process_transfer_to(&account2, &stx2),
            Ok("Transfer Processed".to_string())
        );
        assert_eq!(
            account_tree.process_transfer_from(&account1, &stx2),
            Ok("Processed OK".to_string())
        );
        account_tree.accounts_history.debug_print();

        let day2_acc1_history = account_tree.accounts_history
            .get(&(account1, 2))
            .unwrap()
            .to_owned();
        assert_eq!(day2_acc1_history.balance, 90_000_000_000);
        let day2_acc2_history = account_tree.accounts_history
            .get(&(account2, 2))
            .unwrap()
            .to_owned();
        assert_eq!(day2_acc2_history.balance, 10_000_000_000);

        // Day 3
        // account1: 95_000_000_000
        // account2: 5_000_000_000
        // account3: 0
        let stx3 = SmallTX {
            block: 2,
            time: 3 * 86_400 * 1_000_000_000, // Day 3
            tx_type: 0u8, // Transfer
            value: 5_000_000_000,
            fee: Some(0),
            to: Some(account1),
            from: Some(account2),
        };
        assert_eq!(
            account_tree.process_transfer_to(&account1, &stx3),
            Ok("Transfer Processed".to_string())
        );
        assert_eq!(
            account_tree.process_transfer_from(&account2, &stx3),
            Ok("Processed OK".to_string())
        );
        let day3_acc1_history = account_tree.accounts_history
            .get(&(account1, 3))
            .unwrap()
            .to_owned();
        assert_eq!(day3_acc1_history.balance, 95_000_000_000);
        let day3_acc2_history = account_tree.accounts_history
            .get(&(account2, 3))
            .unwrap()
            .to_owned();
        assert_eq!(day3_acc2_history.balance, 5_000_000_000);

        // Day 3
        // account1: 80_000_000_000
        // account2: 5_000_000_000
        // account3: 15_000_000_000
        let stx4 = SmallTX {
            block: 3,
            time: 4 * 86_400 * 1_000_000_000, // Day 4
            tx_type: 0u8, // Transfer
            value: 15_000_000_000,
            fee: Some(0),
            to: Some(account3),
            from: Some(account1),
        };
        assert_eq!(
            account_tree.process_transfer_to(&account3, &stx4),
            Ok("Transfer Processed".to_string())
        );
        assert_eq!(
            account_tree.process_transfer_from(&account1, &stx4),
            Ok("Processed OK".to_string())
        );
        let day4_acc1_history = account_tree.accounts_history
            .get(&(account1, 4))
            .unwrap()
            .to_owned();
        assert_eq!(day4_acc1_history.balance, 80_000_000_000);
        let day4_acc3_history = account_tree.accounts_history
            .get(&(account3, 4))
            .unwrap()
            .to_owned();
        assert_eq!(day4_acc3_history.balance, 15_000_000_000);
    }

    #[test]
    fn test_activity_stats() {
        // init test Stable/ Runtime state
        test_state_init();

        let ptx = ptx_test_data();
        let _stx = processedtx_to_smalltx(&ptx);
        let snapshots = STABLE_STATE.with(|s| {
            s.borrow().as_ref().unwrap().activity_stats.get_daily_snapshots(99)
        });
        println!("{:?}", snapshots);
        // First tx date - 28 June 2023
        // last tx date - Sunday, 9 July 2023
        // 12 days
        assert_eq!(snapshots.len(), 12);
        assert_eq!(snapshots[0].total_unique_accounts, 10);
        assert_eq!(snapshots[0].accounts_active_during_snapshot, 10);
        assert_eq!(snapshots[0].start_time, 1687939200000000000); // first tx time
        assert_eq!(snapshots[0].end_time, 1687996800000000000); // next midnight
        assert_eq!(snapshots[1].end_time - snapshots[0].end_time, D1_AS_NANOS); // window is 24 hours.
    }
}
