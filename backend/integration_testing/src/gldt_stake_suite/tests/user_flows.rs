use crate::client::gldt_stake::{_add_stake_mock, manage_stake_position_with_tick};
use crate::client::gldt_stake::{get_position, get_total_staked};
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::create_stake_position_util_mock;
use crate::gldt_stake_suite::utils::{add_rewards_to_neurons, add_stake, create_user_with_funds};
use crate::utils::wait_1_day;
use crate::{
    client::icrc1_icrc2_token::icrc2_approve, gldt_stake_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};
use bity_ic_canister_time::DAY_IN_MS;
use bity_ic_canister_time::HOUR_IN_MS;
use candid::{Nat, Principal};
use gldt_stake_api_canister::manage_stake_position;
use gldt_stake_common::ledgers::GLDT_TX_FEE;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use gldt_stake_common::manage_stake_position_interface::StartDissolvingErrors;
use icrc_ledger_types::icrc1::account::Account;
use std::collections::HashMap;
use std::time::Duration;
use types::TokenSymbol;

#[test]
fn test_can_claim_gldt_stake_rewards() {
    // --- Setup test environment ---
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

    // --- Create stake positions ---
    let users: Vec<_> = (0..10)
        .map(|_| {
            create_stake_position_util_mock(
                pic,
                controller,
                &token_ledgers,
                gldt_stake_canister_id,
                10_000_000_000u128,
            )
            .0
        })
        .collect();
    let user = users[0].clone();
    let user1 = users[1].clone();

    // --- Add rewards to neurons ---
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
    wait_1_day(pic);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS));
    tick_n_blocks(pic, 100);

    // --- Check that the rewards are available for the user ---
    let user_position = get_position(pic, user, gldt_stake_canister_id, &user).unwrap();
    let rewards = &user_position.claimable_rewards;
    assert_eq!(rewards[&TokenSymbol::GOLDAO], Nat::from(4_714_275_714_u64));
    assert_eq!(rewards[&TokenSymbol::OGY], Nat::from(4_714_265_714_u64));
    assert_eq!(rewards[&TokenSymbol::ICP], Nat::from(4_714_284_714_u64));
    println!("User position: {:?}", user_position);

    pic.advance_time(Duration::from_secs(2));
    tick_n_blocks(pic, 50);

    // --- Start dissolving 50% of stake ---
    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::StartDissolving { fraction: 50 },
    );
    println!("Start dissolving response: {:?}", response);
    assert!(response.is_ok());
    pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(pic, 50);
    wait_1_day(pic);

    // --- Check that the user who dissolved his stake has less claimable rewards over time ---
    let user_position = get_position(pic, user, gldt_stake_canister_id, &user);
    println!("User position: {:?}", user_position);
    let user1_position = get_position(pic, user1, gldt_stake_canister_id, &user1);
    println!("User 1 position: {:?}", user1_position);

    let user_total: candid::Nat = user_position
        .as_ref()
        .unwrap()
        .claimable_rewards
        .values()
        .cloned()
        .fold(candid::Nat::from(0u64), |acc, x| acc + x);

    let user1_total: candid::Nat = user1_position
        .as_ref()
        .unwrap()
        .claimable_rewards
        .values()
        .cloned()
        .fold(candid::Nat::from(0u64), |acc, x| acc + x);

    assert!(user1_total > user_total);

    // --- Get the rewards pool balances ---
    let unallocated_rewards =
        unallocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Unallocated rewards balance: {:?}", unallocated_rewards);

    let processing_rewards =
        processing_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Processing rewards balance: {:?}", processing_rewards);

    let allocated_rewards =
        allocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Allocated rewards balance: {:?}", allocated_rewards);

    // --- Get the total claimable rewards for all stake positions by token ---
    let mut total_claimable_per_token: HashMap<TokenSymbol, candid::Nat> = HashMap::new();

    for user_principal in users {
        let user_position = get_position(
            pic,
            user_principal,
            gldt_stake_canister_id.clone(),
            &user_principal,
        );

        if let Some(pos) = user_position {
            for (token_symbol, amount) in pos.claimable_rewards.iter() {
                total_claimable_per_token
                    .entry(token_symbol.clone())
                    .and_modify(|e| *e += amount.clone())
                    .or_insert(amount.clone());
            }
        } else {
            println!("User {:?} has no stake position.", user_principal);
        }
    }

    for (token, amount) in total_claimable_per_token.iter() {
        println!("Total claimable for token {}: {:?}", token, amount);
    }

    for (token, claimable_amount) in &total_claimable_per_token {
        let allocated_amount = allocated_rewards.get(token).cloned().unwrap().unwrap();

        println!(
            "Checking token {}: claimable = {:?}, allocated = {:?}",
            token, claimable_amount, allocated_amount
        );

        assert!(
            *claimable_amount <= allocated_amount,
            "Claimable exceeds allocated for token {}",
            token
        );
    }

    let mut total_pools: HashMap<TokenSymbol, Nat> = HashMap::new();

    for (token, amount) in allocated_rewards {
        total_pools.insert(token.clone(), amount.unwrap());
    }
    for (token, amount) in processing_rewards {
        total_pools
            .entry(token.clone())
            .and_modify(|e| *e += amount.clone().unwrap())
            .or_insert(amount.unwrap());
    }
    for (token, amount) in unallocated_rewards {
        total_pools
            .entry(token.clone())
            .and_modify(|e| *e += amount.clone().unwrap())
            .or_insert(amount.unwrap());
    }

    let expected = Nat::from(500_000_000_000u64) * 2u64;
    for (token, total) in total_pools {
        println!("Token {} total pools: {:?}", token, total);
        assert_eq!(
            total,
            // NOTE: there are 4 fees for each token for this test, because the jobs were triggered for several days
            expected.clone()
                - Nat::from(4_u64) * ledger_fees.get(token.symbol()).cloned().unwrap_or_default(),
            "Total pools for token {} doesn't match expected {:?}",
            token,
            expected - ledger_fees.get(token.symbol()).cloned().unwrap_or_default()
        );
    }
}

#[test]
fn dissolve_50_and_dissolve_instantly_50_test() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();

    // --- Create stake position ---
    let (user, _) = create_stake_position_util_mock(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        10_000_000_000u128,
    );

    pic.advance_time(Duration::from_secs(2));
    tick_n_blocks(pic, 50);

    // --- Start dissolving 50% of stake ---
    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::StartDissolving { fraction: 50 },
    );
    assert!(response.is_ok());

    pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(pic, 10);

    let user_position = get_position(pic, user, gldt_stake_canister_id, &user);
    println!("User position: {:?}", user_position);
    pic.advance_time(Duration::from_secs(60));
    tick_n_blocks(pic, 10);

    // --- Start dissolving 50% of stake ---
    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::DissolveInstantly { fraction: 100 },
    );
    assert!(response.is_ok());

    // --- Advance time to complete dissolve period ---
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 7));
    tick_n_blocks(pic, 10);

    // --- Successfully withdraw ---
    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::Withdraw {},
    );
    println!("response: {:?}", response);
    assert!(response.is_ok());

    // --- Verify position staked is 0 ---
    let position =
        get_position(pic, user, gldt_stake_canister_id, &user).expect("Position should exist");
    assert_eq!(position.staked, Nat::from(0_u64));

    // --- Try to withdraw again (should fail, nothing to withdraw) ---
    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::Withdraw {},
    );
    assert!(matches!(
        response,
        Err(ManageStakePositionError::WithdrawError(_))
    ));
}

#[test]
fn withdraw_flow_test() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers.get("gldt_ledger_canister_id").unwrap();

    // --- Create user and fund it ---
    let user = create_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        10_000_000_000u128,
    );

    // --- Approve and stake ---
    let stake_amount = 1_000_000_000u128;
    let total_approval = stake_amount + GLDT_TX_FEE as u128;
    let approval_result = icrc2_approve(
        pic,
        user,
        gldt_ledger_id.clone(),
        &crate::client::icrc1_icrc2_token::icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: gldt_stake_canister_id,
                subaccount: None,
            },
            amount: Nat::from(total_approval),
            expected_allowance: Some(Nat::from(0_u64)),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        },
    );
    assert!(matches!(
        approval_result,
        crate::client::icrc1_icrc2_token::icrc2_approve::Response::Ok(_)
    ));
    tick_n_blocks(pic, 10);

    let stake_response = _add_stake_mock(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::AddStake {
            amount: Nat::from(total_approval),
        },
    )
    .expect("Failed to add stake");
    tick_n_blocks(pic, 10);
    assert_eq!(stake_response.staked, Nat::from(stake_amount));

    // --- Start dissolving full stake ---
    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::StartDissolving { fraction: 100 },
    );
    tick_n_blocks(pic, 10);
    println!("Start dissolving response: {:?}", response);
    assert!(response.is_ok());

    // --- Attempt to unstake immediately (should not fail, should be already dissolvable) ---
    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::Withdraw {},
    );
    assert!(matches!(response, Ok(_)));

    // --- Advance time to complete dissolve period ---
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 7));
    tick_n_blocks(pic, 10);

    let logs = pic
        .fetch_canister_logs(gldt_stake_canister_id, controller)
        .expect("Failed to fetch logs from the canister");
    let log_strings: Vec<String> = logs
        .iter()
        .map(|entry| {
            String::from_utf8(entry.content.clone())
                .unwrap_or_else(|_| "<Invalid UTF-8>".to_string())
        })
        .collect();
    // Print all logs
    println!("Logs from canister {}:", gldt_stake_canister_id);
    for (i, log) in log_strings.iter().enumerate() {
        println!("  [{}] {}", i, log);
    }

    // --- Verify position staked is 0 ---
    let position = get_position(pic, user, gldt_stake_canister_id, &user);
    assert!(position.is_none());
}

#[test]
fn test_get_pool_balances() {
    // --- Setup test environment ---
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

    // --- Add rewards to neurons ---
    add_rewards_to_neurons(
        pic,
        neuron_data.clone(),
        controller,
        &token_ledgers,
        gld_rewards_canister_id,
        gldt_stake_canister_id,
        ledger_fees.clone(),
    );

    pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(pic, 5);

    let unallocated_rewards =
        unallocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Unallocated rewards balance: {:?}", unallocated_rewards);

    let processing_rewards =
        processing_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Processing rewards balance: {:?}", processing_rewards);

    let allocated_rewards =
        allocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Allocated rewards balance: {:?}", allocated_rewards);
}

use crate::client::gldt_stake::allocated_rewards_balance;
use crate::client::gldt_stake::processing_rewards_balance;
use crate::client::gldt_stake::unallocated_rewards_balance;
