use crate::client::gldt_stake::icrc3_get_blocks;
use crate::client::gldt_stake::icrc3_get_properties;
use crate::client::gldt_stake::icrc3_get_tip_certificate;
use crate::client::gldt_stake::icrc3_supported_block_types;
use crate::client::gldt_stake::manage_stake_position;
use crate::gldt_stake_suite::setup::{default_test_setup, setup::GldtStakeTestEnv};
use crate::gldt_stake_suite::utils::create_stake_position_util_for_user;
use crate::gldt_stake_suite::utils::create_whitelisted_user_with_funds;
use crate::utils::tick_n_blocks;
use candid::Nat;
use canister_time::MINUTE_IN_MS;
use gldt_stake_api_canister::manage_stake_position;
use icrc_ledger_types::icrc3::blocks::GetBlocksRequest;
use std::time::Duration;

#[test]
fn icrc3_works() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    println!("test_env: {:?}", test_env.gldt_stake_canister_id);

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers
        .get("gldt_ledger_canister_id")
        .expect("Missing GLDT ledger canister ID");

    let user = create_whitelisted_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        2_000_000_000u128,
    );

    // --- Create stake position and add stake to it 9 more times ---
    for _ in 0..10 {
        let _ = create_stake_position_util_for_user(
            pic,
            controller,
            &token_ledgers,
            gldt_stake_canister_id,
            5_000_000_000u128,
            user,
        );
        tick_n_blocks(pic, 20);
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 30));
    }

    let get_blocks_args = vec![GetBlocksRequest {
        start: Nat::from(0u64),
        length: Nat::from(100u64),
    }];
    let blocks = icrc3_get_blocks(pic, controller, gldt_stake_canister_id, &get_blocks_args);
    println!("blocks: {blocks:?}");
    println!("blocks len: {:?}", blocks.blocks.len());
    assert_eq!(blocks.blocks.len(), 1);
    assert_eq!(blocks.archived_blocks.len(), 1);
    assert_eq!(blocks.archived_blocks[0].args.len(), 1);
    assert_eq!(
        blocks.archived_blocks[0].args[0],
        GetBlocksRequest {
            start: Nat::from(0u64),
            length: Nat::from(9u64),
        }
    );

    let archived_blocks = icrc3_get_blocks(
        pic,
        controller,
        blocks.archived_blocks[0].callback.canister_id,
        &vec![GetBlocksRequest {
            start: Nat::from(0u64),
            length: Nat::from(9u64),
        }],
    );

    for i in 0..9 {
        match &archived_blocks.blocks[i].block {
            icrc_ledger_types::icrc::generic_value::ICRC3Value::Map(map) => {
                assert_eq!(
                    map.get("btype"),
                    Some(&icrc_ledger_types::icrc::generic_value::ICRC3Value::Text(
                        "add_stake".to_string()
                    )),
                    "Block {} is not a mint transaction",
                    i
                );
            }
            _ => panic!("Block is not a map"),
        }
    }

    // --- Create 5 legit dissolvements ---
    for _ in 0..=4 {
        let response = manage_stake_position(
            pic,
            user,
            gldt_stake_canister_id,
            &manage_stake_position::Args::StartDissolving { fraction: 20 },
        );

        assert!(response.is_ok());
    }

    tick_n_blocks(pic, 1);

    let get_blocks_args = vec![GetBlocksRequest {
        start: Nat::from(0u64),
        length: Nat::from(100u64),
    }];
    let blocks = icrc3_get_blocks(pic, controller, gldt_stake_canister_id, &get_blocks_args);
    println!("blocks: {blocks:?}");
    println!("blocks len: {:?}", blocks.blocks.len());
    assert_eq!(blocks.blocks.len(), 3);
    assert_eq!(blocks.archived_blocks.len(), 1);
    assert_eq!(blocks.archived_blocks[0].args.len(), 1);
    assert_eq!(
        blocks.archived_blocks[0].args[0],
        GetBlocksRequest {
            start: Nat::from(0u64),
            length: Nat::from(12u64),
        }
    );
    let archived_blocks = icrc3_get_blocks(
        pic,
        controller,
        blocks.archived_blocks[0].callback.canister_id,
        &vec![GetBlocksRequest {
            start: Nat::from(10u64),
            length: Nat::from(12u64),
        }],
    );

    for i in 0..2 {
        match &archived_blocks.blocks[i].block {
            icrc_ledger_types::icrc::generic_value::ICRC3Value::Map(map) => {
                assert_eq!(
                    map.get("btype"),
                    Some(&icrc_ledger_types::icrc::generic_value::ICRC3Value::Text(
                        "start_dissolving".to_string()
                    )),
                    "Block {} is not a mint transaction",
                    i
                );
            }
            _ => panic!("Block is not a map"),
        }
    }
}

#[test]
fn test_icrc3_get_tip_certificate() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    println!("test_env: {:?}", test_env.gldt_stake_canister_id);

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers
        .get("gldt_ledger_canister_id")
        .expect("Missing GLDT ledger canister ID");

    let user = create_whitelisted_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        2_000_000_000u128,
    );

    let _ = create_stake_position_util_for_user(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        5_000_000_000u128,
        user,
    );
    tick_n_blocks(pic, 20);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 30));

    let certificate = icrc3_get_tip_certificate(pic, controller, gldt_stake_canister_id, &());

    assert!(
        !certificate.certificate.iter().all(|&x| x == 0),
        "Tip certificate should contain non-zero bytes"
    );
    assert!(
        !certificate.hash_tree.iter().all(|&x| x == 0),
        "Hash tree should contain non-zero bytes"
    );

    let certificate_2 = icrc3_get_tip_certificate(pic, controller, gldt_stake_canister_id, &());

    assert_eq!(
        certificate.certificate, certificate_2.certificate,
        "Certificate should be the same"
    );
    assert_eq!(
        certificate.hash_tree, certificate_2.hash_tree,
        "Hash tree should be the same"
    );

    let _ = create_stake_position_util_for_user(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        5_000_000_000u128,
        user,
    );
    tick_n_blocks(pic, 20);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 30));

    let new_certificate = icrc3_get_tip_certificate(pic, controller, gldt_stake_canister_id, &());

    assert_ne!(
        certificate.certificate, new_certificate.certificate,
        "Certificate should change after transfer"
    );
    assert_ne!(
        certificate.hash_tree, new_certificate.hash_tree,
        "Hash tree should change after transfer"
    );
}

#[test]
fn test_icrc3_supported_block_types() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    println!("test_env: {:?}", test_env.gldt_stake_canister_id);

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();

    // Get supported block types
    let block_types = icrc3_supported_block_types(pic, controller, gldt_stake_canister_id, &());

    // Verify all expected block types are present
    let expected_types = vec![
        "add_stake",
        "claim_rewards",
        "dissolve_instantly",
        "start_dissolving",
        "withdraw",
    ];

    assert!(
        block_types.len() >= expected_types.len(),
        "Expected at least {} block types, got {}",
        expected_types.len(),
        block_types.len()
    );

    for expected_type in expected_types {
        let found = block_types.iter().any(|bt| bt.block_type == expected_type);
        assert!(
            found,
            "Expected block type '{}' not found in supported types",
            expected_type
        );
    }
}

#[test]
fn test_icrc3_get_properties() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    println!("test_env: {:?}", test_env.gldt_stake_canister_id);

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();

    let props = icrc3_get_properties(pic, controller, gldt_stake_canister_id, &());

    println!("props: {:?}", props);
    assert!(
        props.max_transactions_in_window > Nat::from(0u64),
        "Max transactions per request should be > 0"
    );
    assert!(
        props.max_blocks_per_response > Nat::from(0u64),
        "Max blocks per response should be > 0"
    );
}
