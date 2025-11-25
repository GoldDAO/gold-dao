use crate::client::gldt_swap::get_available_nfts;
use crate::client::gldt_swap::icrc3_get_blocks;
use crate::client::gldt_swap::swap_nft_for_tokens;
use crate::client::gldt_swap_index::get_blocks;
use crate::gldt_swap_index_suite::setup::default_test_setup;
use crate::gldt_swap_index_suite::setup::setup::TestEnv;
use crate::utils::tick_n_blocks;
use candid::Nat;
use gldt_swap_common::nft::Nft;
use gldt_swap_common::swap::SwapStatus;
use gldt_swap_index_api_canister::index::IndexType;
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use icrc_ledger_types::icrc3::blocks::GetBlocksRequest;
use std::collections::HashSet;
use std::time::Duration;

#[test]
pub fn test_index_for_forward_swap() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut gold_1g_nft_test_env,
        mut gold_10g_nft_test_env,
        mut gold_100g_nft_test_env,
        mut gold_1000g_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        index_canister_id,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    let nft_1g = gold_1g_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("gold_1g".to_string()))],
    );
    tick_n_blocks(pic, 100);
    gold_1g_nft_test_env.approve(owner_1, nft_1g.clone(), gldt_swap_canister_id);
    let nft_10g = gold_10g_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("gold_10g".to_string()))],
    );
    tick_n_blocks(pic, 100);
    gold_10g_nft_test_env.approve(owner_1, nft_10g.clone(), gldt_swap_canister_id);
    let nft_100g = gold_100g_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("gold_100g".to_string()),
        )],
    );
    tick_n_blocks(pic, 100);
    gold_100g_nft_test_env.approve(owner_1, nft_100g.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 100);
    let nft_1000g = gold_1000g_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("gold_1000g".to_string()),
        )],
    );
    gold_1000g_nft_test_env.approve(owner_1, nft_1000g.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 100);

    // Add both to swap set
    let mut nfts_to_swap = HashSet::new();
    nfts_to_swap.insert(Nft {
        id: nft_1g,
        canister_id: gold_1g_nft_test_env.collection_canister_id,
    });
    nfts_to_swap.insert(Nft {
        id: nft_10g,
        canister_id: gold_10g_nft_test_env.collection_canister_id,
    });
    nfts_to_swap.insert(Nft {
        id: nft_100g,
        canister_id: gold_100g_nft_test_env.collection_canister_id,
    });
    nfts_to_swap.insert(Nft {
        id: nft_1000g,
        canister_id: gold_1000g_nft_test_env.collection_canister_id,
    });

    tick_n_blocks(pic, 100);
    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);
    match res {
        Ok(ids) => {
            println!("swap_ids (multi): {:?}", ids);
            assert_eq!(ids.len(), 4);
        }
        Err(e) => panic!("/// intent to swap errors (multi): {e:?}"),
    }
    tick_n_blocks(pic, 100);
    pic.advance_time(Duration::from_millis(1000));
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(1000));

    let available_nfts = get_available_nfts(
        pic,
        owner_1,
        gldt_swap_canister_id,
        &Some(gldt_swap_canister_id),
    )
    .unwrap();
    assert_eq!(available_nfts.len(), 4);

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
            filters: vec![IndexType::BlockType("forward_swap".to_string())],

            sort_by: None,
        },
    );
    println!(
        "blocks_from_indexer_filter_1: {:?}",
        blocks_from_indexer_filter_1
    );
    assert_eq!(blocks_from_indexer_filter_1.blocks.len(), 4);

    let blocks_from_indexer_filter_2 = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![IndexType::BlockType("reverse_swap".to_string())],

            sort_by: None,
        },
    );
    assert_eq!(blocks_from_indexer_filter_2.blocks.len(), 0);

    let blocks_from_indexer_filter_3 = get_blocks(
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
    assert_eq!(blocks_from_indexer_filter_3.blocks.len(), 0);

    let blocks_from_indexer_filter_4 = get_blocks(
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
    assert_eq!(blocks_from_indexer_filter_4.blocks.len(), 0);
}

#[test]
pub fn test_index_nft_filteres() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut gold_1g_nft_test_env,
        mut gold_10g_nft_test_env,
        mut gold_100g_nft_test_env,
        mut gold_1000g_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        index_canister_id,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    let nft_1g = gold_1g_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("gold_1g".to_string()))],
    );
    tick_n_blocks(pic, 100);
    gold_1g_nft_test_env.approve(owner_1, nft_1g.clone(), gldt_swap_canister_id);
    let nft_10g = gold_10g_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("gold_10g".to_string()))],
    );
    tick_n_blocks(pic, 100);
    gold_10g_nft_test_env.approve(owner_1, nft_10g.clone(), gldt_swap_canister_id);
    let nft_100g = gold_100g_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("gold_100g".to_string()),
        )],
    );
    tick_n_blocks(pic, 100);
    gold_100g_nft_test_env.approve(owner_1, nft_100g.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 100);
    let nft_1000g = gold_1000g_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("gold_1000g".to_string()),
        )],
    );
    gold_1000g_nft_test_env.approve(owner_1, nft_1000g.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 100);

    // Add both to swap set
    let mut nfts_to_swap = HashSet::new();
    let nft_1 = Nft {
        id: nft_1g,
        canister_id: gold_1g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_1.clone());

    let nft_2 = Nft {
        id: nft_10g,
        canister_id: gold_10g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_2.clone());

    let nft_3 = Nft {
        id: nft_100g,
        canister_id: gold_100g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_3.clone());

    let nft_4 = Nft {
        id: nft_1000g,
        canister_id: gold_1000g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_4.clone());

    tick_n_blocks(pic, 100);
    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);
    match res {
        Ok(ids) => {
            println!("swap_ids (multi): {:?}", ids);
            assert_eq!(ids.len(), 4);
        }
        Err(e) => panic!("/// intent to swap errors (multi): {e:?}"),
    }
    tick_n_blocks(pic, 100);
    pic.advance_time(Duration::from_millis(1000));
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(1000));

    let available_nfts = get_available_nfts(
        pic,
        owner_1,
        gldt_swap_canister_id,
        &Some(gldt_swap_canister_id),
    )
    .unwrap();
    assert_eq!(available_nfts.len(), 4);

    // Advance time to make indexer job work
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(60000));
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(60000));
    tick_n_blocks(pic, 10);

    let blocks_from_indexer_filter_1 = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![IndexType::Nft(nft_1)],
            sort_by: None,
        },
    );
    assert_eq!(blocks_from_indexer_filter_1.blocks.len(), 1);

    let blocks_from_indexer_filter_2 = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![IndexType::Nft(nft_2)],
            sort_by: None,
        },
    );
    assert_eq!(blocks_from_indexer_filter_2.blocks.len(), 1);

    let blocks_from_indexer_filter_3 = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![IndexType::Nft(nft_3)],
            sort_by: None,
        },
    );
    assert_eq!(blocks_from_indexer_filter_3.blocks.len(), 1);

    let blocks_from_indexer_filter_4 = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![IndexType::Nft(nft_4)],
            sort_by: None,
        },
    );
    assert_eq!(blocks_from_indexer_filter_4.blocks.len(), 1);
}

#[test]
pub fn test_index_account_filteres() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut gold_1g_nft_test_env,
        mut gold_10g_nft_test_env,
        mut gold_100g_nft_test_env,
        mut gold_1000g_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        index_canister_id,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    let nft_1g = gold_1g_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("gold_1g".to_string()))],
    );
    tick_n_blocks(pic, 100);
    gold_1g_nft_test_env.approve(owner_1, nft_1g.clone(), gldt_swap_canister_id);
    let nft_10g = gold_10g_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("gold_10g".to_string()))],
    );
    tick_n_blocks(pic, 100);
    gold_10g_nft_test_env.approve(owner_1, nft_10g.clone(), gldt_swap_canister_id);
    let nft_100g = gold_100g_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("gold_100g".to_string()),
        )],
    );
    tick_n_blocks(pic, 100);
    gold_100g_nft_test_env.approve(owner_1, nft_100g.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 100);
    let nft_1000g = gold_1000g_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("gold_1000g".to_string()),
        )],
    );
    gold_1000g_nft_test_env.approve(owner_1, nft_1000g.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 100);

    // Add both to swap set
    let mut nfts_to_swap = HashSet::new();
    let nft_1 = Nft {
        id: nft_1g,
        canister_id: gold_1g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_1.clone());

    let nft_2 = Nft {
        id: nft_10g,
        canister_id: gold_10g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_2.clone());

    let nft_3 = Nft {
        id: nft_100g,
        canister_id: gold_100g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_3.clone());

    let nft_4 = Nft {
        id: nft_1000g,
        canister_id: gold_1000g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_4.clone());

    tick_n_blocks(pic, 100);
    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);
    match res {
        Ok(ids) => {
            println!("swap_ids (multi): {:?}", ids);
            assert_eq!(ids.len(), 4);
        }
        Err(e) => panic!("/// intent to swap errors (multi): {e:?}"),
    }
    tick_n_blocks(pic, 100);
    pic.advance_time(Duration::from_millis(1000));
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(1000));

    let available_nfts = get_available_nfts(
        pic,
        owner_1,
        gldt_swap_canister_id,
        &Some(gldt_swap_canister_id),
    )
    .unwrap();
    assert_eq!(available_nfts.len(), 4);

    // Advance time to make indexer job work
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(60000));
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(60000));

    let blocks_from_indexer_filter_1 = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![IndexType::Nft(nft_1)],
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
            filters: vec![IndexType::Nft(nft_2)],
            sort_by: None,
        },
    );
    assert_eq!(blocks_from_indexer_filter_2.blocks.len(), 1);

    let blocks_from_indexer_filter_3 = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![IndexType::Nft(nft_3)],
            sort_by: None,
        },
    );
    assert_eq!(blocks_from_indexer_filter_3.blocks.len(), 1);

    let blocks_from_indexer_filter_4 = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![IndexType::Nft(nft_4)],
            sort_by: None,
        },
    );
    assert_eq!(blocks_from_indexer_filter_4.blocks.len(), 1);
}

#[test]
pub fn test_index_statuses_filteres() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut gold_1g_nft_test_env,
        mut gold_10g_nft_test_env,
        mut gold_100g_nft_test_env,
        mut gold_1000g_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        index_canister_id,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    let nft_1g = gold_1g_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("gold_1g".to_string()))],
    );
    tick_n_blocks(pic, 100);
    gold_1g_nft_test_env.approve(owner_1, nft_1g.clone(), gldt_swap_canister_id);
    let nft_10g = gold_10g_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("gold_10g".to_string()))],
    );
    tick_n_blocks(pic, 100);
    gold_10g_nft_test_env.approve(owner_1, nft_10g.clone(), gldt_swap_canister_id);
    let nft_100g = gold_100g_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("gold_100g".to_string()),
        )],
    );
    tick_n_blocks(pic, 100);
    gold_100g_nft_test_env.approve(owner_1, nft_100g.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 100);
    let nft_1000g = gold_1000g_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("gold_1000g".to_string()),
        )],
    );
    gold_1000g_nft_test_env.approve(owner_1, nft_1000g.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 100);

    // Add both to swap set
    let mut nfts_to_swap = HashSet::new();
    let nft_1 = Nft {
        id: nft_1g,
        canister_id: gold_1g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_1.clone());

    let nft_2 = Nft {
        id: nft_10g,
        canister_id: gold_10g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_2.clone());

    let nft_3 = Nft {
        id: nft_100g,
        canister_id: gold_100g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_3.clone());

    let nft_4 = Nft {
        id: nft_1000g,
        canister_id: gold_1000g_nft_test_env.collection_canister_id,
    };
    nfts_to_swap.insert(nft_4.clone());

    tick_n_blocks(pic, 100);
    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);
    match res {
        Ok(ids) => {
            println!("swap_ids (multi): {:?}", ids);
            assert_eq!(ids.len(), 4);
        }
        Err(e) => panic!("/// intent to swap errors (multi): {e:?}"),
    }
    tick_n_blocks(pic, 100);
    pic.advance_time(Duration::from_millis(1000));
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(1000));

    let available_nfts = get_available_nfts(
        pic,
        owner_1,
        gldt_swap_canister_id,
        &Some(gldt_swap_canister_id),
    )
    .unwrap();
    assert_eq!(available_nfts.len(), 4);

    // Advance time to make indexer job work
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(60000));
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(60000));
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(60000));
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(60000));

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

    let blocks_from_indexer_filter_1 = get_blocks(
        pic,
        owner_1,
        index_canister_id,
        &gldt_swap_index_api_canister::get_blocks::Args {
            start: 0,
            length: 10,
            filters: vec![IndexType::Status(SwapStatus::Init)],
            sort_by: None,
        },
    );
    println!(
        "blocks_from_indexer_filter_1: {:?}",
        blocks_from_indexer_filter_1
    );
    assert_eq!(blocks_from_indexer_filter_1.total, 4);
}
