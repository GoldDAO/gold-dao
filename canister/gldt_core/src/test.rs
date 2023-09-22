use icrc_ledger_types::icrc1::{
    account::{ Account },
};
use candid::{ Principal };
use super::*;



// --------------------------------- helpers ----------------------------------
fn init_service() {
    init(Some(Conf::new(
        Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").expect("Could not decode the principal."),
        vec![
            (Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."), NftCanisterConf::new(1)),
            (Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), NftCanisterConf::new(10))
        ]
    )));
}

// ------------------------- get_swaps_by_user tests --------------------------
#[test]
fn test_get_swaps_by_user_a1() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(1), Some(50));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_a2() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(u32::MAX), Some(50));

    assert_eq!(res, Err("Overflow when calculating start".to_string()));
}

#[test]
fn test_get_swaps_by_user_a3() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(50), Some(u32::MAX));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}


#[test]
fn test_get_swaps_by_user_a4() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(1), Some(u32::MAX));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_a5() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(u32::MAX), Some(1));

    assert_eq!(res, Err("Overflow when calculating end".to_string()));
}

#[test]
fn test_get_swaps_by_user_a6() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(1), Some(0));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_a7() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(0), Some(1));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_a8() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), None, Some(0));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_a9() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(0), None);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_a10() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), None, Some(10));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_a11() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(10), None);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_b1() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(1), Some(50));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_b2() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(u32::MAX), Some(50));

    assert_eq!(res, Err("Overflow when calculating start".to_string()));
}

#[test]
fn test_get_swaps_by_user_b3() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(50), Some(u32::MAX));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}


#[test]
fn test_get_swaps_by_user_b4() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(1), Some(u32::MAX));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_b5() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(u32::MAX), Some(1));

    assert_eq!(res, Err("Overflow when calculating end".to_string()));
}

#[test]
fn test_get_swaps_by_user_b6() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(1), Some(0));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_b7() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(0), Some(1));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_b8() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), None, Some(0));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_b9() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(0), None);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_b10() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), None, Some(10));

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_swaps_by_user_b11() {
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(10), None);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}


// ------------------------- get_status_of_swap tests -------------------------


#[test]
fn test_get_status_of_swap_a1() {
    let status_request = GetStatusRequest {
        nft_id: "".to_string(),
        gld_nft_canister_id: Principal::anonymous(),
        sale_id: "".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Err("invalid GLD NFT canister ID: was 2vxsx-fae, expected one of []".to_string()));
}

#[test]
fn test_get_status_of_swap_a2() {
    init_service();

    let status_request = GetStatusRequest {
        nft_id: "".to_string(),
        gld_nft_canister_id: Principal::anonymous(),
        sale_id: "".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Err(format!("invalid GLD NFT canister ID: was 2vxsx-fae, expected one of {:?}",        vec![
            Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."),
            Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal.")
        ]
    )));
}

#[test]
fn test_get_status_of_swap_a3() {
    init_service();

    let status_request = GetStatusRequest {
        nft_id: "".to_string(),
        gld_nft_canister_id: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."),
        sale_id: "".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());
    
    assert_eq!(res, Ok(GetStatusResponse { status: None }));
}

#[test]
fn test_get_status_of_swap_a4() {
    init_service();

    let status_request = GetStatusRequest {
        nft_id: "".to_string(),
        gld_nft_canister_id: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."),
        sale_id: "".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());
    
    assert_eq!(res, Ok(GetStatusResponse { status: None }));
}


#[test]
fn test_get_status_of_swap_b1() {
    init_service();

    SERVICE.with(|s| {
        let registry = &mut s.borrow_mut().registry;

        match registry.entry((Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), "random_nft_id_2".to_string()).clone()) {
            btree_map::Entry::Vacant(v) => { v.insert(GldNft {
                gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
                to_subaccount: [0u8; 32],
                nft_sale_id: "randomSellId".to_string(),
                grams: 10,
                receiving_account: Account {
                    owner: Principal::anonymous(),
                    subaccount: Some([0u8; 32]),
                },
                gldt_minting_timestamp_seconds: 0,
                requested_memo: Memo::from(0),
                minted: None,
                swapped: None,
                older_record: None,
            }.clone()); }
            _ => { }
        }
    });

    let status_request = GetStatusRequest {
        nft_id: "random_nft_id_2".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
        sale_id: "randomSellId".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Ok(GetStatusResponse { status: Some(SwappingStates::Initialised) }));
}


#[test]
fn test_get_status_of_swap_b2() {
    init_service();
    
    SERVICE.with(|s| {
        let registry = &mut s.borrow_mut().registry;

        match registry.entry((Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), "random_nft_id_2".to_string()).clone()) {
            btree_map::Entry::Vacant(v) => { v.insert(GldNft {
                gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
                to_subaccount: [0u8; 32],
                nft_sale_id: "randomSellId".to_string(),
                grams: 10,
                receiving_account: Account {
                    owner: Principal::anonymous(),
                    subaccount: Some([0u8; 32]),
                },
                gldt_minting_timestamp_seconds: 0,
                requested_memo: Memo::from(0),
                minted: Some(
                    GldtMinted {
                        mint_block_height: None,
                        last_audited_timestamp_seconds: 0,
                        burned: None,
                        num_tokens: None
                    }
                ),
                swapped: None,
                older_record: None,
            }.clone()); }
            _ => { }
        }
    });

    let status_request = GetStatusRequest {
        nft_id: "random_nft_id_2".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
        sale_id: "randomSellId".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Ok(GetStatusResponse { status: Some(SwappingStates::Minted) }));
}


#[test]
fn test_get_status_of_swap_b3() {
    init_service();
    SERVICE.with(|s| {
        let registry = &mut s.borrow_mut().registry;

        match registry.entry((Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), "random_nft_id_2".to_string()).clone()) {
            btree_map::Entry::Vacant(v) => { v.insert(GldNft {
                gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
                to_subaccount: [0u8; 32],
                nft_sale_id: "randomSellId".to_string(),
                grams: 10,
                receiving_account: Account {
                    owner: Principal::anonymous(),
                    subaccount: Some([0u8; 32]),
                },
                gldt_minting_timestamp_seconds: 0,
                requested_memo: Memo::from(0),
                minted: Some(
                    GldtMinted {
                        mint_block_height: None,
                        last_audited_timestamp_seconds: 0,
                        burned: None,
                        num_tokens: None
                    }
                ),
                swapped: Some(
                    GldtSwapped {
                        sale_id: "randomSellId".to_string(),
                        index: Nat::from(0),
                    }
                ),
                older_record: None,
            }.clone()); }
            _ => { }
        }
    });

    let status_request = GetStatusRequest {
        nft_id: "random_nft_id_2".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
        sale_id: "randomSellId".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Ok(GetStatusResponse { status: Some(SwappingStates::Swapped) }));
}


#[test]
fn test_get_status_of_swap_b4() {
    init_service();
    SERVICE.with(|s| {
        let registry = &mut s.borrow_mut().registry;

        match registry.entry((Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), "random_nft_id_2".to_string()).clone()) {
            btree_map::Entry::Vacant(v) => { v.insert(GldNft {
                gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
                to_subaccount: [0u8; 32],
                nft_sale_id: "randomSellId".to_string(),
                grams: 10,
                receiving_account: Account {
                    owner: Principal::anonymous(),
                    subaccount: Some([0u8; 32]),
                },
                gldt_minting_timestamp_seconds: 0,
                requested_memo: Memo::from(0),
                minted: Some(
                    GldtMinted {
                        mint_block_height: None,
                        last_audited_timestamp_seconds: 0,
                        burned: Some( GldtBurned {
                            burn_block_height: 0
                        }),
                        num_tokens: None
                    }
                ),
                swapped: Some(
                    GldtSwapped {
                        sale_id: "randomSellId".to_string(),
                        index: Nat::from(0),
                    }
                ),
                older_record: None,
            }.clone()); }
            _ => { }
        }
    });

    let status_request = GetStatusRequest {
        nft_id: "random_nft_id_2".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
        sale_id: "randomSellId".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Ok(GetStatusResponse { status: Some(SwappingStates::Burned) }));
}

#[test]
fn test_get_status_of_swap_b5() {
    init_service();
    SERVICE.with(|s| {
        let registry = &mut s.borrow_mut().registry;

        match registry.entry((Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), "random_nft_id_2".to_string()).clone()) {
            btree_map::Entry::Vacant(v) => { v.insert(GldNft {
                gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
                to_subaccount: [0u8; 32],
                nft_sale_id: "randomSellId".to_string(),
                grams: 10,
                receiving_account: Account {
                    owner: Principal::anonymous(),
                    subaccount: Some([0u8; 32]),
                },
                gldt_minting_timestamp_seconds: 0,
                requested_memo: Memo::from(0),
                minted: None,
                swapped: Some(
                    GldtSwapped {
                        sale_id: "randomSellId".to_string(),
                        index: Nat::from(0),
                    }
                ),
                older_record: None,
            }.clone()); }
            _ => { }
        }
    });

    let status_request = GetStatusRequest {
        nft_id: "random_nft_id_2".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
        sale_id: "randomSellId".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Err("Swap status is corrupted.".to_string()));
}

#[test]
fn test_get_status_of_swap_b6() {
    init_service();
    SERVICE.with(|s| {
        let registry = &mut s.borrow_mut().registry;

        match registry.entry((Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), "random_nft_id_2".to_string()).clone()) {
            btree_map::Entry::Vacant(v) => { v.insert(GldNft {
                gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
                to_subaccount: [0u8; 32],
                nft_sale_id: "randomSellId".to_string(),
                grams: 10,
                receiving_account: Account {
                    owner: Principal::anonymous(),
                    subaccount: Some([0u8; 32]),
                },
                gldt_minting_timestamp_seconds: 0,
                requested_memo: Memo::from(0),
                minted: Some(
                    GldtMinted {
                        mint_block_height: None,
                        last_audited_timestamp_seconds: 0,
                        burned: None,
                        num_tokens: None
                    }
                ),
                swapped: None,
                older_record: None,
            }.clone()); }
            _ => { }
        }
    });

    let status_request = GetStatusRequest {
        nft_id: "random_nft_id_2".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
        sale_id: "".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Ok(GetStatusResponse { status: None }));
}


#[test]
fn test_get_status_of_swap_b7() {
    init_service();
    SERVICE.with(|s| {
        let registry = &mut s.borrow_mut().registry;

        match registry.entry((Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), "random_nft_id_2".to_string()).clone()) {
            btree_map::Entry::Vacant(v) => { v.insert(GldNft {
                gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
                to_subaccount: [0u8; 32],
                nft_sale_id: "randomSellId".to_string(),
                grams: 10,
                receiving_account: Account {
                    owner: Principal::anonymous(),
                    subaccount: Some([0u8; 32]),
                },
                gldt_minting_timestamp_seconds: 0,
                requested_memo: Memo::from(0),
                minted: Some(
                    GldtMinted {
                        mint_block_height: None,
                        last_audited_timestamp_seconds: 0,
                        burned: None,
                        num_tokens: None
                    }
                ),
                swapped: None,
                older_record: None,
            }.clone()); }
            _ => { }
        }
    });

    let status_request = GetStatusRequest {
        nft_id: "".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
        sale_id: "randomSellId".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Ok(GetStatusResponse { status: None }));
}