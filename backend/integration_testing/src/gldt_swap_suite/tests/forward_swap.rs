use crate::client::gldt_swap::get_active_swaps_by_ids;
use crate::client::gldt_swap::get_available_nfts;
use crate::client::gldt_swap::icrc3_get_blocks;
use crate::client::gldt_swap::swap_nft_for_tokens;
use crate::client::icrc1::client::balance_of;
use crate::client::icrc1::icrc1_total_supply;
use crate::gldt_swap_suite::setup::default_test_setup;
use crate::gldt_swap_suite::setup::real_data_test_setup;
use crate::gldt_swap_suite::setup::setup_one_canister::TestEnv;
use crate::gldt_swap_suite::setup::setup_real_gold_config::RealDataTestEnv;
use crate::utils::tick_n_blocks;
use assert_matches::assert_matches;
use bity_ic_canister_time::DAY_IN_MS;
use candid::Encode;
use candid::{Nat, Principal};
use gldt_swap_api_canister::swap_nft_for_tokens::SwapNftForTokensErrors;
use gldt_swap_common::general_error::GeneralError;
use gldt_swap_common::nft::Nft;
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use icrc_ledger_types::icrc3::blocks::GetBlocksRequest;
// use origyn_nft_canister::icrc37_get_token_approvals;
use std::collections::HashSet;
use std::time::Duration;

#[test]
pub fn forward_swap_basic_only() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    let nft_1 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("test".to_string()))],
    );
    origyn_nft_test_env.approve(owner_1, nft_1.clone(), gldt_swap_canister_id);

    let mut nfts_to_swap = HashSet::new();
    nfts_to_swap.insert(Nft {
        id: nft_1,
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);
    match res {
        Ok(ids) => {
            let swap_id = ids[0].clone();
            println!("swap_id: {:?}", swap_id);
        }
        Err(e) => {
            panic!("/// intent to swap errors : {e:?}");
        }
    }
}

#[test]
pub fn forward_swap_multiple_nfts() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        gldt_ledger_canister_id,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    // Mint multiple NFTs
    let nft_1 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("test1".to_string()))],
    );
    let nft_2 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("test2".to_string()))],
    );
    tick_n_blocks(pic, 5);

    // Approve NFTs for swap
    origyn_nft_test_env.approve(owner_1, nft_1.clone(), gldt_swap_canister_id);
    origyn_nft_test_env.approve(owner_1, nft_2.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 5);

    let mut nfts_to_swap = HashSet::new();
    nfts_to_swap.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    nfts_to_swap.insert(Nft {
        id: nft_2.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);
    tick_n_blocks(pic, 20);
    assert!(res.is_ok(), "Swap should succeed for multiple NFTs");
    let swap_ids = res.unwrap();
    assert_eq!(swap_ids.len(), 2, "Should return swap IDs for both NFTs");

    for id in swap_ids {
        let swap = get_active_swaps_by_ids(
            pic,
            owner_1,
            gldt_swap_canister_id,
            &vec![id].into_iter().collect::<HashSet<_>>(),
        );
        println!("Swap: {:?}", swap);
    }

    let user_nfts_after_swap = origyn_nft_test_env.balance_of(owner_1);
    assert_eq!(user_nfts_after_swap.len(), 1);
    let canister_nfts_after_swap = origyn_nft_test_env.balance_of(gldt_swap_canister_id);
    assert_eq!(canister_nfts_after_swap.len(), 1);

    let nft_1_owner = origyn_nft_test_env.owner_of(&nft_1);
    assert_eq!(
        nft_1_owner.first().unwrap().unwrap(),
        gldt_swap_canister_id.into()
    );
    let nft_2_owner = origyn_nft_test_env.owner_of(&nft_2);
    assert_eq!(
        nft_2_owner.first().unwrap().unwrap(),
        gldt_swap_canister_id.into()
    );

    let user_token_balance_after_swap = balance_of(pic, gldt_ledger_canister_id, owner_1);
    assert_eq!(
        user_token_balance_after_swap,
        Nat::from(100_000_000_000_000_u64) * Nat::from(2_u64) // - GLDT_TX_FEE
    );

    tick_n_blocks(pic, 20);
    pic.advance_time(Duration::from_millis(DAY_IN_MS));

    let get_blocks_args = vec![GetBlocksRequest {
        start: Nat::from(0u64),
        length: Nat::from(5u64),
    }];
    let blocks = icrc3_get_blocks(pic, owner_1, gldt_swap_canister_id, &get_blocks_args);
    println!("blocks: {:?}", blocks);
}

#[test]
pub fn multiple_swaps_one_by_one_tokens() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        gldt_ledger_canister_id,
        owner_1,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 5);

    let nft_1 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("multi1".to_string()))],
    );
    let nft_2 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("multi2".to_string()))],
    );
    tick_n_blocks(pic, 5);

    origyn_nft_test_env.approve(owner_1, nft_1.clone(), gldt_swap_canister_id);
    origyn_nft_test_env.approve(owner_1, nft_2.clone(), gldt_swap_canister_id);

    let mut nfts_first = HashSet::new();
    nfts_first.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    let _ = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_first);
    tick_n_blocks(pic, 10);

    let mut nfts_second = HashSet::new();
    nfts_second.insert(Nft {
        id: nft_2.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    let _ = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_second);
    tick_n_blocks(pic, 10);

    let nft_1_owner = origyn_nft_test_env.owner_of(&nft_1);
    assert_eq!(
        nft_1_owner.first().unwrap().unwrap(),
        gldt_swap_canister_id.into()
    );
    let nft_2_owner = origyn_nft_test_env.owner_of(&nft_2);
    assert_eq!(
        nft_2_owner.first().unwrap().unwrap(),
        gldt_swap_canister_id.into()
    );

    let token_balance = balance_of(pic, gldt_ledger_canister_id, owner_1);
    assert!(
        token_balance >= Nat::from(200_000_000_000_000_u64),
        "Owner should accumulate tokens from multiple swaps"
    );
}

#[test]
pub fn swap_with_duplicate_nft_entries() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 5);

    let nft_1 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("dup".to_string()))],
    );
    origyn_nft_test_env.approve(owner_1, nft_1.clone(), gldt_swap_canister_id);

    let mut nfts_to_swap = HashSet::new();
    nfts_to_swap.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    nfts_to_swap.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    let call1 = pic
        .submit_call(
            gldt_swap_canister_id,
            owner_1,
            "swap_nft_for_tokens",
            Encode!(&nfts_to_swap).unwrap(),
        )
        .unwrap();
    let call2 = pic
        .submit_call(
            gldt_swap_canister_id,
            owner_1,
            "swap_nft_for_tokens",
            Encode!(&nfts_to_swap).unwrap(),
        )
        .unwrap();

    let res1: Result<Vec<Nat>, SwapNftForTokensErrors> =
        crate::client::pocket::unwrap_response(pic.await_call(call1));
    let res2: Result<Vec<Nat>, SwapNftForTokensErrors> =
        crate::client::pocket::unwrap_response(pic.await_call(call2));

    match res1 {
        Ok(_) => {
            assert_matches!(
                res2,
                Err(SwapNftForTokensErrors::GeneralError(
                    GeneralError::AlreadyProcessing(_)
                ))
            );
        }
        Err(_) => {
            assert_eq!(matches!(res2, Ok(_)), true);
        }
    }
}

#[test]
pub fn swap_same_nft_concurrently_by_two_users() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        owner_2,
        ..
    } = env;
    let pic = &pic.borrow();

    let nft_1 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("concurrent".to_string()),
        )],
    );
    origyn_nft_test_env.approve(owner_1, nft_1.clone(), gldt_swap_canister_id);

    let mut nfts_to_swap = HashSet::new();
    nfts_to_swap.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    // User1 starts a swap
    let call1 = pic
        .submit_call(
            gldt_swap_canister_id,
            owner_1,
            "swap_nft_for_tokens",
            Encode!(&nfts_to_swap).unwrap(),
        )
        .unwrap();

    // User2 tries to swap the same NFT concurrently
    let call2 = pic
        .submit_call(
            gldt_swap_canister_id,
            owner_2,
            "swap_nft_for_tokens",
            Encode!(&nfts_to_swap).unwrap(),
        )
        .unwrap();

    let res1: Result<Vec<Nat>, SwapNftForTokensErrors> =
        crate::client::pocket::unwrap_response(pic.await_call(call1));
    let res2: Result<Vec<Nat>, SwapNftForTokensErrors> =
        crate::client::pocket::unwrap_response(pic.await_call(call2));

    // One should succeed, the other must fail
    assert!(
        (res1.is_ok() && res2.is_err()) || (res1.is_err() && res2.is_ok()),
        "Only one swap should succeed for the same NFT"
    );
}

#[test]
pub fn swap_nft_not_owned() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        owner_2,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    let nft_1 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("test".to_string()))],
    );
    tick_n_blocks(pic, 5);

    let mut nfts_to_swap = HashSet::new();
    nfts_to_swap.insert(Nft {
        id: nft_1,
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    let res = swap_nft_for_tokens(pic, owner_2, gldt_swap_canister_id, &nfts_to_swap);
    assert!(res.is_err(), "Swap should fail if user does not own NFT");
}

#[test]
pub fn swap_empty_nft_set() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        gldt_swap_canister_id,
        owner_1,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 5);

    let nfts_to_swap = HashSet::new();
    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);
    assert!(
        matches!(
            res,
            Err(SwapNftForTokensErrors::GeneralError(
                GeneralError::EmptyArgs(_)
            ))
        ),
        "Should fail for empty NFT set"
    );
}

#[test]
pub fn swap_updates_gldt_supply() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_ledger_canister_id,
        gldt_swap_canister_id,
        owner_1,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    let pre_swap_supply =
        icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger_canister_id, &());

    let nft_1 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("test".to_string()))],
    );
    tick_n_blocks(pic, 5);

    origyn_nft_test_env.approve(owner_1, nft_1.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 5);

    let mut nfts_to_swap = HashSet::new();
    nfts_to_swap.insert(Nft {
        id: nft_1,
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    let _res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);
    tick_n_blocks(pic, 10);

    let post_swap_supply =
        icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger_canister_id, &());
    assert!(
        post_swap_supply > pre_swap_supply,
        "GLDT supply should increase after NFT swap"
    );
}

#[test]
pub fn swap_without_approval_fails() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    let nft_1 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("no-approval".to_string()),
        )],
    );
    tick_n_blocks(pic, 5);

    let mut nfts_to_swap = HashSet::new();
    nfts_to_swap.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    // Attempt swap without approval
    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);
    assert!(
        res.is_err(),
        "Swap should fail if NFT was not approved for transfer"
    );

    // Verify NFT is still owned by the user
    let owner = origyn_nft_test_env.owner_of(&nft_1);
    assert_eq!(owner.first().unwrap().unwrap(), owner_1.into());
}

#[test]
pub fn swap_different_owners_in_same_batch_fails() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        owner_2,
        ..
    } = env;
    let pic = &pic.borrow();

    let nft_1 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("mixed1".to_string()))],
    );
    let nft_2 = origyn_nft_test_env.mint_nft(
        owner_2,
        vec![("name".to_string(), ICRC3Value::Text("mixed2".to_string()))],
    );
    origyn_nft_test_env.approve(owner_1, nft_1.clone(), gldt_swap_canister_id);
    origyn_nft_test_env.approve(owner_2, nft_2.clone(), gldt_swap_canister_id);

    let mut nfts_to_swap = HashSet::new();
    nfts_to_swap.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    nfts_to_swap.insert(Nft {
        id: nft_2.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);
    assert!(
        res.is_err(),
        "Batch swap should fail if not all NFTs belong to the caller"
    );
}

#[test]
pub fn forward_swap_all_gold_bar_nfts() {
    let mut env = real_data_test_setup();
    let RealDataTestEnv {
        ref mut pic,
        mut gold_1g_nft_test_env,
        mut gold_10g_nft_test_env,
        mut gold_100g_nft_test_env,
        mut gold_1000g_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    let nft_1g = gold_1g_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("gold_1g".to_string()))],
    );
    gold_1g_nft_test_env.approve(owner_1, nft_1g.clone(), gldt_swap_canister_id);
    let nft_10g = gold_10g_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("gold_10g".to_string()))],
    );
    gold_10g_nft_test_env.approve(owner_1, nft_10g.clone(), gldt_swap_canister_id);
    let nft_100g = gold_100g_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("gold_100g".to_string()),
        )],
    );
    gold_100g_nft_test_env.approve(owner_1, nft_100g.clone(), gldt_swap_canister_id);
    let nft_1000g = gold_1000g_nft_test_env.mint_nft(
        owner_1,
        vec![(
            "name".to_string(),
            ICRC3Value::Text("gold_1000g".to_string()),
        )],
    );
    gold_1000g_nft_test_env.approve(owner_1, nft_1000g.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 10);

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

    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);
    match res {
        Ok(ids) => {
            println!("swap_ids (multi): {:?}", ids);
            assert_eq!(ids.len(), 4);
        }
        Err(e) => panic!("/// intent to swap errors (multi): {e:?}"),
    }

    let available_nfts = get_available_nfts(
        pic,
        owner_1,
        gldt_swap_canister_id,
        &Some(gldt_swap_canister_id),
    );
    println!("available_nfts: {:?}", available_nfts);
}
