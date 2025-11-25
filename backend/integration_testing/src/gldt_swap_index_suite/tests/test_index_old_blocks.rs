use crate::client::gldt_swap::_insert_fake_old_bulk_swaps;
use crate::client::gldt_swap::icrc3_get_blocks;
use crate::client::gldt_swap_index::get_blocks;
use crate::gldt_swap_index_suite::setup::default_test_setup;
use crate::gldt_swap_index_suite::setup::setup::TestEnv;
use crate::utils::tick_n_blocks;
use candid::Nat;
use gldt_swap_api_archive::swap::SwapDetailForward;
use gldt_swap_api_archive::swap::SwapDetailReverse;
use gldt_swap_api_archive::types::swap::SwapInfo;
use gldt_swap_index_api_canister::index::IndexType;
use icrc_ledger_types::icrc3::blocks::GetBlocksRequest;
use std::time::Duration;

#[test]
pub fn test_index_for_old_blocks() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        gldt_swap_canister_id,
        owner_1,
        index_canister_id,
        controller,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    let default_forwerd_swap = SwapInfo::Forward(SwapDetailForward::default());
    let default_reverse_swap = SwapInfo::Reverse(SwapDetailReverse::default());
    let mut swaps = std::collections::HashMap::new();
    swaps.insert(Nat::from(1u64), default_forwerd_swap.clone());
    swaps.insert(Nat::from(2u64), default_reverse_swap.clone());

    let _ = _insert_fake_old_bulk_swaps(pic, controller, gldt_swap_canister_id, &swaps).unwrap();

    // Advance time to make indexer job work
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(60000));
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(60000));

    let get_blocks_args = vec![GetBlocksRequest {
        start: Nat::from(0u64),
        length: Nat::from(10u64),
    }];
    let blocks = icrc3_get_blocks(pic, owner_1, gldt_swap_canister_id, &get_blocks_args);
    println!("blocks: {:?}", blocks);

    let blocks_from_indexer = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![],
            sort_by: None,
        },
    );
    println!("blocks_from_indexer: {:?}", blocks_from_indexer);
    assert_eq!(blocks_from_indexer.blocks.len(), blocks.blocks.len());

    let blocks_from_indexer_filter_1 = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![IndexType::BlockType("forward_swap_old".to_string())],

            sort_by: None,
        },
    );
    println!(
        "blocks_from_indexer_filter_1: {:?}",
        blocks_from_indexer_filter_1
    );
    assert_eq!(blocks_from_indexer_filter_1.blocks.len(), 1);

    let blocks_from_indexer_filter_2 = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![IndexType::BlockType("reverse_swap_old".to_string())],

            sort_by: None,
        },
    );
    assert_eq!(blocks_from_indexer_filter_2.blocks.len(), 1);
}
