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

fn init_records() {
    let _ = add_record("random_nft_id_1".to_string(), GldNft {
            gld_nft_canister_id: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."),
            to_subaccount: [0u8; 32],
            nft_sale_id: "randomSellId1".to_string(),
            grams: 1,
            receiving_account: Account {
                owner: Principal::anonymous(),
                subaccount: Some([0u8; 32]),
            },
            gldt_minting_timestamp_seconds: 0,
            requested_memo: Memo::from(0),
            minted: None,
            swapped: None,
            older_record: None,
        }.clone());

    let _ = add_record("random_nft_id_2".to_string(), GldNft {
            gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
            to_subaccount: [0u8; 32],
            nft_sale_id: "randomSellId2".to_string(),
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
        }.clone());

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
    init_records();
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(0), Some(50));

    assert_eq!(res, Ok(
        GetRecordsResponse { 
            total: 2, 
            data: Some(
                vec![
                    GldtRecord { 
                        record_type: RecordType::Mint, 
                        timestamp: 0, 
                        counterparty: Account { 
                            owner: Principal::anonymous(), 
                            subaccount: Some([0u8; 32])
                        },
                        gld_nft_canister_id: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."), 
                        nft_id: "random_nft_id_1".to_string(),
                        escrow_subaccount: Some([0u8; 32]), 
                        nft_sale_id: "randomSellId1".to_string(), 
                        grams: 1, 
                        num_tokens: GldtNumTokens { 
                            value: Nat::from(0) 
                        }, 
                        block_height: Nat::from(0), 
                        memo: Memo::from(0)
                    }, 
                    GldtRecord { 
                        record_type: RecordType::Mint, 
                        timestamp: 0, 
                        counterparty: Account {
                            owner: Principal::anonymous(), 
                            subaccount: Some([0u8; 32])
                        },
                        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
                        nft_id: "random_nft_id_2".to_string(), 
                        escrow_subaccount: Some([0u8; 32]), 
                        nft_sale_id: "randomSellId2".to_string(), 
                        grams: 10, 
                        num_tokens: GldtNumTokens { 
                            value: Nat::from(0) 
                        }, 
                        block_height: Nat::from(0), 
                        memo: Memo::from(0)
                    }
                ]
        ) 
    })) 
}

#[test]
fn test_get_swaps_by_user_b2() {
    init_records();
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(1), Some(50));

    assert_eq!(res, Ok(GetRecordsResponse { total: 2, data: None }));
}

#[test]
fn test_get_swaps_by_user_b3() {
    init_records();
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(0), Some(1));

    assert_eq!(res, Ok(
        GetRecordsResponse { 
            total: 2, 
            data: Some(
                vec![
                    GldtRecord { 
                        record_type: RecordType::Mint, 
                        timestamp: 0, 
                        counterparty: Account { 
                            owner: Principal::anonymous(), 
                            subaccount: Some([0u8; 32])
                        },
                        gld_nft_canister_id: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."), 
                        nft_id: "random_nft_id_1".to_string(),
                        escrow_subaccount: Some([0u8; 32]), 
                        nft_sale_id: "randomSellId1".to_string(), 
                        grams: 1, 
                        num_tokens: GldtNumTokens { 
                            value: Nat::from(0) 
                        }, 
                        block_height: Nat::from(0), 
                        memo: Memo::from(0)
                    }, 
                ]
        ) 
    })) 
}

#[test]
fn test_get_swaps_by_user_b4() {
    init_records();
    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let res = get_swaps_by_user(Some(account), Some(1), Some(1));

    assert_eq!(res, Ok(
        GetRecordsResponse { 
            total: 2, 
            data: Some(
                vec![
                    GldtRecord { 
                        record_type: RecordType::Mint, 
                        timestamp: 0, 
                        counterparty: Account {
                            owner: Principal::anonymous(), 
                            subaccount: Some([0u8; 32])
                        },
                        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
                        nft_id: "random_nft_id_2".to_string(), 
                        escrow_subaccount: Some([0u8; 32]), 
                        nft_sale_id: "randomSellId2".to_string(), 
                        grams: 10, 
                        num_tokens: GldtNumTokens { 
                            value: Nat::from(0) 
                        }, 
                        block_height: Nat::from(0), 
                        memo: Memo::from(0)
                    }
                ]
        ) 
    })) 
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

// --------------------------------- get_records ----------------------------------
#[test]
fn test_get_records_a1() {
    let records_request = GetRecordsRequest {
        page: Some(1), 
        limit: Some(50)    
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_records_a2() {
    let records_request = GetRecordsRequest {
        page: Some(u32::MAX), 
        limit: Some(50)    
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Err("Overflow when calculating start".to_string()));
}

#[test]
fn test_get_records_a3() {
    let records_request = GetRecordsRequest {
        page: Some(50), 
        limit: Some(u32::MAX)
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}


#[test]
fn test_get_records_a4() {
    let records_request = GetRecordsRequest {
        page: Some(u32::MAX), 
        limit: Some(1)
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_records_a5() {
    let records_request = GetRecordsRequest {
        page: Some(1), 
        limit: Some(u32::MAX)
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_records_a6() {
    let records_request = GetRecordsRequest {
        page: Some(1), 
        limit: Some(0)
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_records_a7() {
    let records_request = GetRecordsRequest {
        page: Some(0), 
        limit: Some(1)
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_records_a8() {
    let records_request = GetRecordsRequest {
        page: Some(0), 
        limit: None
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_records_a9() {
    let records_request = GetRecordsRequest {
        page: None, 
        limit: Some(0)
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_records_a10() {
    let records_request = GetRecordsRequest {
        page: Some(10), 
        limit: None
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_records_a11() {
    let records_request = GetRecordsRequest {
        page: None, 
        limit: Some(10)
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 0, data: None }));
}

#[test]
fn test_get_records_b1() {
    init_records();

    let records_request = GetRecordsRequest {
        page: Some(1), 
        limit: Some(50)    
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 2, data: None }));
}

#[test]
fn test_get_records_b2() {
    init_records();

    let records_request = GetRecordsRequest {
        page: Some(0), 
        limit: Some(50)    
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 2, data: Some(
                vec![
                    GldtRecord { 
                        record_type: RecordType::Mint, 
                        timestamp: 0, 
                        counterparty: Account { 
                            owner: Principal::anonymous(), 
                            subaccount: Some([0u8; 32])
                        },
                        gld_nft_canister_id: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."), 
                        nft_id: "random_nft_id_1".to_string(),
                        escrow_subaccount: Some([0u8; 32]), 
                        nft_sale_id: "randomSellId1".to_string(), 
                        grams: 1, 
                        num_tokens: GldtNumTokens { 
                            value: Nat::from(0) 
                        }, 
                        block_height: Nat::from(0), 
                        memo: Memo::from(0)
                    }, 
                    GldtRecord { 
                        record_type: RecordType::Mint, 
                        timestamp: 0, 
                        counterparty: Account {
                            owner: Principal::anonymous(), 
                            subaccount: Some([0u8; 32])
                        },
                        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
                        nft_id: "random_nft_id_2".to_string(), 
                        escrow_subaccount: Some([0u8; 32]), 
                        nft_sale_id: "randomSellId2".to_string(), 
                        grams: 10, 
                        num_tokens: GldtNumTokens { 
                            value: Nat::from(0) 
                        }, 
                        block_height: Nat::from(0), 
                        memo: Memo::from(0)
                    }
                ]
        ) 
 }));
}

#[test]
fn test_get_records_b3() {
    init_records();

    let records_request = GetRecordsRequest {
        page: Some(0), 
        limit: Some(1)    
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 2, data: Some(
                vec![
                    GldtRecord { 
                        record_type: RecordType::Mint, 
                        timestamp: 0, 
                        counterparty: Account { 
                            owner: Principal::anonymous(), 
                            subaccount: Some([0u8; 32])
                        },
                        gld_nft_canister_id: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."), 
                        nft_id: "random_nft_id_1".to_string(),
                        escrow_subaccount: Some([0u8; 32]), 
                        nft_sale_id: "randomSellId1".to_string(), 
                        grams: 1, 
                        num_tokens: GldtNumTokens { 
                            value: Nat::from(0) 
                        }, 
                        block_height: Nat::from(0), 
                        memo: Memo::from(0)
                    }, 
                ]
        )  }));
}

#[test]
fn test_get_records_b4() {
    init_records();

    let records_request = GetRecordsRequest {
        page: Some(1), 
        limit: Some(1)    
    };
    
    let res = get_records(records_request);

    assert_eq!(res, Ok(GetRecordsResponse { total: 2, data: Some(
                vec![
                    GldtRecord { 
                        record_type: RecordType::Mint, 
                        timestamp: 0, 
                        counterparty: Account {
                            owner: Principal::anonymous(), 
                            subaccount: Some([0u8; 32])
                        },
                        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."),
                        nft_id: "random_nft_id_2".to_string(), 
                        escrow_subaccount: Some([0u8; 32]), 
                        nft_sale_id: "randomSellId2".to_string(), 
                        grams: 10, 
                        num_tokens: GldtNumTokens { 
                            value: Nat::from(0) 
                        }, 
                        block_height: Nat::from(0), 
                        memo: Memo::from(0)
                    }
                ]
        )  }));
}
