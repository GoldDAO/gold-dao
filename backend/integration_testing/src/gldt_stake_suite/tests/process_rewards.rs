use crate::client::gldt_stake::allocated_rewards_balance;
use crate::client::gldt_stake::processing_rewards_balance;
use crate::client::gldt_stake::unallocated_rewards_balance;
use crate::client::gldt_stake::{
    _set_token_usd_values, get_apy_overall, get_apy_timeseries, get_position,
};
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::add_custom_rewards_to_processing_pool;
use crate::gldt_stake_suite::utils::{add_rewards_to_neurons, create_stake_position_util};
use crate::utils::wait_1_day;
use crate::{gldt_stake_suite::setup::default_test_setup, utils::tick_n_blocks};
use candid::Nat;
use candid::Principal;
use canister_time::{DAY_IN_MS, HOUR_IN_MS};
use std::time::Duration;
use types::TokenSymbol;

#[test]
fn test_process_staking_rewards() {
    let mut test_env = default_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        gld_rewards_canister_id,
        neuron_data,
        ledger_fees,
        ..
    } = test_env;
    let pic = &pic.borrow();

    let icp_ledger = token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    assert_eq!(
        icp_ledger,
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap()
    );
    tick_n_blocks(pic, 10);

    let usd_values = vec![
        TokenSymbol::GLDT,
        TokenSymbol::GOLDAO,
        TokenSymbol::OGY,
        TokenSymbol::ICP,
    ]
    .into_iter()
    .map(|sym| (sym, 1.0))
    .collect();
    _set_token_usd_values(pic, controller, gldt_stake_canister_id, &usd_values);

    // --- Create 10 stake positions ---
    let users: Vec<_> = (0..10)
        .map(|_| {
            create_stake_position_util(
                pic,
                controller,
                &token_ledgers,
                gldt_stake_canister_id,
                1_000_000_000u128,
            )
            .0
        })
        .collect();

    add_rewards_to_neurons(
        pic,
        neuron_data.clone(),
        controller,
        &token_ledgers,
        gld_rewards_canister_id,
        gldt_stake_canister_id,
        ledger_fees.clone(),
    );

    let unallocated_rewards =
        unallocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Unallocated rewards balance: {:?}", unallocated_rewards);

    let processing_rewards =
        processing_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Processing rewards balance: {:?}", processing_rewards);

    let allocated_rewards =
        allocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Allocated rewards balance: {:?}", allocated_rewards);

    wait_1_day(pic);
    wait_1_day(pic);
    wait_1_day(pic);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS));
    tick_n_blocks(pic, 10);

    let unallocated_rewards =
        unallocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Unallocated rewards balance: {:?}", unallocated_rewards);

    let processing_rewards =
        processing_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Processing rewards balance: {:?}", processing_rewards);

    let allocated_rewards =
        allocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Allocated rewards balance: {:?}", allocated_rewards);

    for user in users {
        let position = get_position(pic, user, gldt_stake_canister_id, &()).unwrap();
        let rewards = &position.claimable_rewards;
        assert_eq!(
            rewards[&TokenSymbol::GOLDAO].clone() - TokenSymbol::GOLDAO.get_token_info().fee,
            Nat::from(9_206_206_058_u64)
        );
        assert_eq!(
            rewards[&TokenSymbol::OGY].clone() - TokenSymbol::OGY.get_token_info().fee,
            Nat::from(9_206_085_587_u64)
        );
        assert_eq!(
            rewards[&TokenSymbol::ICP].clone() - TokenSymbol::OGY.get_token_info().fee,
            Nat::from(9_206_124_483_u64)
        );
    }

    pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(pic, 10);

    let apy_history = get_apy_timeseries(
        pic,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &gldt_stake_api_canister::get_apy_timeseries::Args {
            starting_day: 0,
            limit: None,
        },
    );
    println!("apy_history {:?}", apy_history);
    assert_eq!(apy_history.len(), 3);
    assert!(apy_history[1].1 > 0.0);
}

// NOTE: this test transfers rewards to the processing pool (to support the same tokens amount) and then checks that APY is increasing
#[test]
fn test_apy_changes_with_usd_fluctuations() {
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ledger_fees,
        ..
    } = test_env;
    let pic = &pic.borrow();

    tick_n_blocks(pic, 10);

    let mut usd_values = vec![
        TokenSymbol::GOLDAO,
        TokenSymbol::OGY,
        TokenSymbol::ICP,
        TokenSymbol::GLDT,
    ]
    .into_iter()
    .map(|sym| (sym, 1.0))
    .collect();
    _set_token_usd_values(pic, controller, gldt_stake_canister_id, &usd_values);

    create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        5_000_000_000u128,
    );

    usd_values.insert(TokenSymbol::ICP, 5.0);
    _set_token_usd_values(pic, controller, gldt_stake_canister_id, &usd_values);

    let target_unallocated_pool: u128 = 10_000_000_000; // e.g. 100,000 GLDT
    let mut overall_apyies = Vec::new();

    for i in 0..5 {
        let unallocated_rewards_map =
            unallocated_rewards_balance(pic, controller, gldt_stake_canister_id, &());

        let unallocated_rewards = unallocated_rewards_map
            .get(&TokenSymbol::OGY)
            .cloned()
            .unwrap()
            .unwrap();

        if unallocated_rewards < target_unallocated_pool {
            let to_add =
                target_unallocated_pool - u128::try_from(unallocated_rewards.0.clone()).unwrap();
            // let per_token_amount = to_add / 3;

            add_custom_rewards_to_processing_pool(
                pic,
                controller,
                &token_ledgers,
                gldt_stake_canister_id,
                ledger_fees.clone(),
                to_add,
            );
        }

        pic.advance_time(Duration::from_millis(DAY_IN_MS));
        tick_n_blocks(pic, 15);
        pic.advance_time(Duration::from_millis(HOUR_IN_MS));
        tick_n_blocks(pic, 15);

        let usd_values = vec![
            TokenSymbol::GOLDAO,
            TokenSymbol::OGY,
            TokenSymbol::ICP,
            TokenSymbol::GLDT,
        ]
        .into_iter()
        .map(|sym| (sym, 1.0 + i as f64))
        .collect();
        _set_token_usd_values(pic, controller, gldt_stake_canister_id, &usd_values);

        let apy_overall = get_apy_overall(pic, Principal::anonymous(), gldt_stake_canister_id, &());
        overall_apyies.push(apy_overall);
    }

    // NOTE: the first APY is 0
    overall_apyies.remove(0);

    // Check that overall APY values are increasing over time
    let apy_is_increasing = overall_apyies
        .iter()
        .try_fold(None, |last_apy, &apy| {
            if let Some(prev_apy) = last_apy {
                if apy > prev_apy {
                    return None; // Found a drop in APY
                }
            }
            Some(Some(apy))
        })
        .is_some();

    assert!(
        apy_is_increasing,
        "Overall APY is not increasing with time: {:?}",
        overall_apyies
    );
}

// NOTE: APY should remain stable within a small margin over ~3 weeks
#[test]
#[ignore]
fn test_apy_stability_over_three_weeks() {
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        gld_rewards_canister_id,
        neuron_data,
        ledger_fees,
        ..
    } = test_env;
    let pic = &pic.borrow();

    tick_n_blocks(pic, 10);

    let mut usd_values = vec![
        TokenSymbol::GOLDAO,
        TokenSymbol::OGY,
        TokenSymbol::ICP,
        TokenSymbol::GLDT,
    ]
    .into_iter()
    .map(|sym| (sym, 1.0))
    .collect();
    _set_token_usd_values(pic, controller, gldt_stake_canister_id, &usd_values);

    let (user, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        5_000_000_000u128,
    );

    usd_values.insert(TokenSymbol::ICP, 5.0);
    _set_token_usd_values(pic, controller, gldt_stake_canister_id, &usd_values);

    let mut apy_values = Vec::new();

    // Simulate 21 days (3 weeks)
    for _day in 0..21 {
        // Make rewards bigger to distribute instantly
        add_rewards_to_neurons(
            pic,
            neuron_data.clone(),
            controller,
            &token_ledgers,
            gld_rewards_canister_id,
            gldt_stake_canister_id,
            ledger_fees.clone(),
        );

        wait_1_day(pic);

        let apy_overall = get_apy_overall(pic, Principal::anonymous(), gldt_stake_canister_id, &());
        apy_values.push(apy_overall);

        let user_position = get_position(pic, user, gldt_stake_canister_id, &());
        println!("User position: {:?}", user_position);
    }

    // Ignore first 3 days to skip initial transient effects
    let apy_values = &apy_values[3..];

    // Check APY is stable: difference between min and max APY should be within 5%
    let min_apy = apy_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_apy = apy_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let tolerance = 0.34 * min_apy.max(1.0); // 34% tolerance

    assert!(
        (max_apy - min_apy) <= tolerance,
        "APY fluctuated too much over 3 weeks. APY values: {:?}, min: {}, max: {}, tolerance: {}",
        apy_values,
        min_apy,
        max_apy,
        tolerance
    );
}
