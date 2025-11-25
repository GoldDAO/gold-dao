use crate::client::gldt_swap::get_active_swaps_by_ids;
use crate::client::gldt_swap::swap_nft_for_tokens;
use crate::client::gldt_swap::swap_tokens_for_nft;
use crate::client::icrc1::client::balance_of;
use crate::client::icrc1::client::transfer;
use crate::client::icrc1_icrc2_token::icrc2_approve;
use crate::gldt_swap_suite::setup::default_test_setup;
use crate::gldt_swap_suite::setup::setup_one_canister::TestEnv;
use crate::utils::tick_n_blocks;
use candid::Nat;
use gldt_swap_api_canister::swap_tokens_for_nft::SwapTokensForNftErrors;
use gldt_swap_common::general_error::GeneralError;
use gldt_swap_common::gldt::GLDT_SWAP_FEE_ACCOUNT;
use gldt_swap_common::nft::Nft;
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use icrc_ledger_types::icrc1::account::Account;
use std::collections::HashSet;

#[test]
pub fn reverse_swap_basic_only() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        gldt_ledger_canister_id,
        owner_2,
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

    let mut nfts_to_sell = HashSet::new();
    nfts_to_sell.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    nfts_to_sell.insert(Nft {
        id: nft_2.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    // swap 0/1
    let _ = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_sell);
    tick_n_blocks(pic, 20);

    let _ = transfer(
        &pic,
        owner_1,
        gldt_ledger_canister_id,
        None,
        owner_2,
        150_000_000_000_000,
    )
    .unwrap();
    let user_token_balance_before_swap = balance_of(pic, gldt_ledger_canister_id, owner_2);
    println!(
        "user_token_balance_before_swap: {:?}",
        user_token_balance_before_swap
    );

    // Approve before swap
    let total_approval = 100_000_000_000_000 + 100_000_000_u128 + 10_000_000_u128; // 100_000_000_u128 - GLDT swap fee

    let approval_result = icrc2_approve(
        pic,
        owner_2,
        gldt_ledger_canister_id.clone(),
        &icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: gldt_swap_canister_id,
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
    println!("{:?}", approval_result);

    let mut nfts_to_buy = HashSet::new();
    nfts_to_buy.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    // NOTE: it works with not enough funds
    // swap 2 - nft 1
    let res = swap_tokens_for_nft(pic, owner_2, gldt_swap_canister_id, &nfts_to_buy);
    println!("{:?}", res);

    let swap = get_active_swaps_by_ids(
        pic,
        owner_1,
        gldt_swap_canister_id,
        &vec![res.unwrap().first().unwrap().clone()]
            .into_iter()
            .collect::<HashSet<_>>(),
    );
    println!("swap: {:?}", swap);

    let user_token_balance_after_swap = balance_of(pic, gldt_ledger_canister_id, owner_2);
    println!(
        "user_token_balance_after_swap: {:?}",
        user_token_balance_after_swap
    );
    assert_eq!(
        user_token_balance_after_swap,
        Nat::from(49_999_880_000_000_u64) // - fee
    );
    let nft_1_owner = origyn_nft_test_env.owner_of(&nft_1);
    assert_eq!(nft_1_owner.first().unwrap().unwrap(), owner_2.into());
    let nft_2_owner = origyn_nft_test_env.owner_of(&nft_2);
    assert_eq!(
        nft_2_owner.first().unwrap().unwrap(),
        gldt_swap_canister_id.into()
    );

    let fee_account_balance = balance_of(
        pic,
        gldt_ledger_canister_id,
        Account {
            owner: gldt_swap_canister_id,
            subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
        },
    );
    println!("fee_account_balance: {:?}", fee_account_balance)
}

#[test]
pub fn reverse_swap_not_enough_tokens() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        gldt_ledger_canister_id,
        owner_2,
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

    let mut nfts_to_sell = HashSet::new();
    nfts_to_sell.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    nfts_to_sell.insert(Nft {
        id: nft_2.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_sell);
    println!("res: {:?}", res);
    tick_n_blocks(pic, 20);

    let _ = transfer(
        &pic,
        owner_1,
        gldt_ledger_canister_id,
        None,
        owner_2,
        150_000_000_000_000,
    )
    .unwrap();

    let mut nfts_to_buy = HashSet::new();
    nfts_to_buy.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    // NOTE: it works with not enough funds
    let res = swap_tokens_for_nft(pic, owner_1, gldt_swap_canister_id, &nfts_to_buy);
    match res {
        Err(SwapTokensForNftErrors::GeneralError(GeneralError::CallError(msg))) => {
            assert_eq!(msg, "No token burns succeeded; NFTs not transferred.");
        }
        other => panic!("unexpected result: {:?}", other),
    }

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
}

#[test]
pub fn reverse_swap_multiple_not_enough_tokens() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        owner_1,
        gldt_ledger_canister_id,
        owner_2,
        ..
    } = env;
    let pic = &pic.borrow();
    tick_n_blocks(pic, 10);

    // Mint two NFTs
    let nft_1 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("nft1".to_string()))],
    );
    let nft_2 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("nft2".to_string()))],
    );
    let nft_3 = origyn_nft_test_env.mint_nft(
        owner_1,
        vec![("name".to_string(), ICRC3Value::Text("nft3".to_string()))],
    );

    // Approve NFTs for swap
    origyn_nft_test_env.approve(owner_1, nft_1.clone(), gldt_swap_canister_id);
    origyn_nft_test_env.approve(owner_1, nft_2.clone(), gldt_swap_canister_id);
    origyn_nft_test_env.approve(owner_1, nft_3.clone(), gldt_swap_canister_id);
    tick_n_blocks(pic, 5);

    // Swap all NFTs for tokens
    let mut nfts_to_sell = HashSet::new();
    nfts_to_sell.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    nfts_to_sell.insert(Nft {
        id: nft_2.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    let response = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_sell);
    println!("swap result 1: {:?}", response);
    tick_n_blocks(pic, 20);

    // Approve only *half* the needed tokens (enough for 1 NFT, not 2)
    let tokens_to_transfer = 100_000_000_000_000;
    let _ = transfer(
        &pic,
        owner_1,
        gldt_ledger_canister_id,
        None,
        owner_2,
        tokens_to_transfer,
    )
    .unwrap();

    // Approve before swap
    let total_approval = 100_000_000_000_000 + 100_000_000_u128; // 100_000_000_u128 - GLDT swap fee

    let approval_result = icrc2_approve(
        pic,
        owner_2,
        gldt_ledger_canister_id.clone(),
        &icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: gldt_swap_canister_id,
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
    println!("{:?}", approval_result);

    // Try to buy back both NFTs
    let mut nfts_to_buy = HashSet::new();
    nfts_to_buy.insert(Nft {
        id: nft_1.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    nfts_to_buy.insert(Nft {
        id: nft_2.clone(),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    let res = swap_tokens_for_nft(pic, owner_1, gldt_swap_canister_id, &nfts_to_buy);
    tick_n_blocks(pic, 20);
    match res {
        Err(SwapTokensForNftErrors::GeneralError(GeneralError::CallError(msg))) => {
            assert_eq!(msg, "No token burns succeeded; NFTs not transferred.");
        }
        other => panic!("unexpected result: {:?}", other),
    }
}

// TODO: throttle from icrc7 fix
// #[test]
// pub fn reverse_swap_multiple_nfts_success() {
//     let mut env = default_test_setup();
//     let TestEnv {
//         ref mut pic,
//         mut origyn_nft_test_env,
//         gldt_swap_canister_id,
//         owner_1,
//         gldt_ledger_canister_id,
//         owner_2,
//         ..
//     } = env;
//     let pic = &pic.borrow();

//     // Mint and approve 3 NFTs
//     let mut nfts_to_sell = HashSet::new();
//     for i in 0..3 {
//         let nft = origyn_nft_test_env.mint_nft(
//             owner_1,
//             vec![("name".to_string(), ICRC3Value::Text(format!("rev-{i}")))],
//         );
//         origyn_nft_test_env.approve(owner_1, nft.clone(), gldt_swap_canister_id);
//         nfts_to_sell.insert(Nft {
//             id: nft,
//             canister_id: origyn_nft_test_env.collection_canister_id,
//         });
//     }

//     // Swap NFTs for tokens
//     let _ = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_sell);
//     tick_n_blocks(pic, 20);

//     // Transfer tokens to buyer
//     let total_tokens = 300_000_000_000_000;
//     let _ = transfer(
//         &pic,
//         owner_1,
//         gldt_ledger_canister_id,
//         None,
//         owner_2,
//         total_tokens - GLDT_TX_FEE as u128,
//     )
//     .unwrap();

//     // Approve GLDT for swap
//     let approval_result = icrc2_approve(
//         pic,
//         owner_2,
//         gldt_ledger_canister_id.clone(),
//         &icrc2_approve::Args {
//             from_subaccount: None,
//             spender: Account {
//                 owner: gldt_swap_canister_id,
//                 subaccount: None,
//             },
//             amount: Nat::from(200_000_000_000_000 + 2 * GLDT_TX_FEE as u128),
//             expected_allowance: Some(Nat::from(0_u64)),
//             expires_at: None,
//             fee: None,
//             memo: None,
//             created_at_time: None,
//         },
//     );
//     println!("Approval result: {:?}", approval_result);

//     // Buy back all NFTs
//     let nfts_to_buy: HashSet<Nft> = nfts_to_sell.iter().take(2).cloned().collect();
//     let res = swap_tokens_for_nft(pic, owner_2, gldt_swap_canister_id, &nfts_to_buy).unwrap();
//     tick_n_blocks(pic, 20);
//     println!("res: {:?}", res);
//     for id in res {
//         let swap = get_swap(pic, owner_1, gldt_swap_canister_id, &id);
//         println!("Swap: {:?}", swap);
//     }

//     for nft in &nfts_to_buy {
//         let owner = origyn_nft_test_env.owner_of(&nft.id);
//         assert_eq!(owner.first().unwrap().unwrap(), owner_2.into());
//     }
// }

#[test]
pub fn reverse_swap_nft_not_existing() {
    let mut env = default_test_setup();
    let TestEnv {
        ref mut pic,
        mut origyn_nft_test_env,
        gldt_swap_canister_id,
        owner_2,
        ..
    } = env;
    let pic = &pic.borrow();

    // Attempt to buy NFT that never was sold
    let mut nfts_to_buy = HashSet::new();
    nfts_to_buy.insert(Nft {
        id: Nat::from(999_u64),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    let res = swap_tokens_for_nft(pic, owner_2, gldt_swap_canister_id, &nfts_to_buy);
    assert!(
        res.is_err(),
        "Swap should fail if NFT is not in the swap canister"
    );
}

#[test]
pub fn reverse_swap_nft_not_available() {
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
        vec![("name".to_string(), ICRC3Value::Text("test".to_string()))],
    );
    tick_n_blocks(pic, 10);

    let mut nfts_to_swap = HashSet::new();
    nfts_to_swap.insert(Nft {
        id: nft_1,
        canister_id: origyn_nft_test_env.collection_canister_id,
    });
    let res = swap_nft_for_tokens(pic, owner_1, gldt_swap_canister_id, &nfts_to_swap);

    // Attempt to buy NFT that never was sold
    let mut nfts_to_buy = HashSet::new();
    nfts_to_buy.insert(Nft {
        id: Nat::from(0_u64),
        canister_id: origyn_nft_test_env.collection_canister_id,
    });

    let res = swap_tokens_for_nft(pic, owner_2, gldt_swap_canister_id, &nfts_to_buy);
    assert!(
        res.is_err(),
        "Swap should fail if NFT is not in the swap canister"
    );
}
