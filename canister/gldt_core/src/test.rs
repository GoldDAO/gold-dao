use super::*;
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;

use gldt_libs::{
    gld_nft::{
        Account as OrigynAccount,
        AskFeature,
        AuctionStateShared,
        AuctionStateShared_status,
        ICTokenSpec,
        ICTokenSpec_standard,
        PricingConfigShared,
        SaleStatusShared,
        SaleStatusShared_sale_type,
        SubAccountInfo,
        SubAccountInfo_account,
        TokenSpec,
    },
    constants::GLDT_SUBDIVIDABLE_BY,
};
use serde_bytes::ByteBuf;
use records::{ GldtRecord, MAX_NUMBER_OF_RECORDS };
use registry::{ MAX_NUMBER_OF_ENTRIES, MAX_HISTORY_REGISTRY };

// --------------------------------- constants ----------------------------------

const CANISTER_ID_GLDT_CORE: &str = "m45be-jaaaa-aaaak-qcgnq-cai";
const CANISTER_ID_GLDT_FEE_COMPENSATION: &str = "ccjse-eaaaa-aaaao-a2ixq-cai";
const CANISTER_ID_GLDT_LEDGER: &str = "6uad6-fqaaa-aaaam-abovq-cai";
const CANISTER_ID_GLD_NFT_1G: &str = "obapm-2iaaa-aaaak-qcgca-cai";
const CANISTER_ID_GLD_NFT_10G: &str = "xyo2o-gyaaa-aaaal-qb55a-cai";
const CANISTER_ID_YUMI_KYC: &str = "2qft3-raaaa-aaaag-qci4a-cai";
const TEST_PRINCIPAL_ID: &str = "thrhh-hnmzu-kjquw-6ebmf-vdhed-yf2ry-avwy7-2jrrm-byg34-zoqaz-wqe";

// --------------------------------- helpers ----------------------------------

fn init_service() {
    init(
        Some(
            Conf::new(
                Principal::from_text(CANISTER_ID_GLDT_LEDGER).expect(
                    "Could not decode the principal."
                ),
                vec![
                    (
                        Principal::from_text(CANISTER_ID_GLD_NFT_1G).expect(
                            "Could not decode the principal."
                        ),
                        NftCanisterConf::new(1),
                    ),
                    (
                        Principal::from_text(CANISTER_ID_GLD_NFT_10G).expect(
                            "Could not decode the principal."
                        ),
                        NftCanisterConf::new(10),
                    )
                ],
                Principal::from_text(CANISTER_ID_GLDT_FEE_COMPENSATION).expect(
                    "Could not decode the principal."
                )
            )
        )
    );
}

fn init_entry(weight: NftWeight) -> SwapInfo {
    SwapInfo::new(
        "test_sale_id".to_string(),
        [0u8; 32],
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        0,
        calculate_tokens_from_weight(weight).unwrap()
    )
}
fn init_registry(num_entries_per_weight: usize) {
    assert!(num_entries_per_weight < 1000);
    let gld_nft_canister_id = [
        Principal::from_text(CANISTER_ID_GLD_NFT_1G).expect("Could not decode the principal."),
        Principal::from_text(CANISTER_ID_GLD_NFT_10G).expect("Could not decode the principal."),
    ];
    let weights: [NftWeight; 2] = [1, 10];
    for (i, g) in weights.iter().enumerate() {
        let entry = init_entry(*g);
        for id in 0..num_entries_per_weight {
            let nft_id = format!("gold-{id}-{g}g");
            let _ = update_registry(
                &UpdateType::Init,
                nft_id,
                gld_nft_canister_id[i],
                entry.clone()
            );
        }
    }
}

fn update_registry_to_swapped() {
    for (key, val) in REGISTRY.with(|r| r.borrow().get().clone()) {
        let entry = val.clone();
        let mut swap_info = entry.get_issue_info().clone();
        // 1. update to minted
        swap_info.set_ledger_entry(
            GldtLedgerEntry::Minted(
                GldtLedgerInfo::new(
                    Nat::from(0),
                    GldtNumTokens::new(Nat::from(100 * GLDT_SUBDIVIDABLE_BY)).unwrap()
                )
            )
        );
        let _ = update_registry(&UpdateType::Mint, key.1.clone(), key.0, swap_info.clone());
        // 2. update to swapped
        swap_info.set_swapped(GldtSwapped::new("test_sale_id".to_string(), Nat::from(100)));
        let _ = update_registry(&UpdateType::Swap, key.1, key.0, swap_info);
    }
}

fn init_records() {
    let _ = add_record(
        "random_nft_id_1".to_string(),
        Principal::from_text(CANISTER_ID_GLD_NFT_1G).expect("Could not decode the principal."),
        &SwapInfo::new(
            "randomSellId1".to_string(),
            [0u8; 32],
            Account {
                owner: Principal::anonymous(),
                subaccount: Some([0u8; 32]),
            },
            0,
            GldtNumTokens::new(Nat::from(0)).unwrap()
        ),
        RecordStatusInfo {
            status: RecordStatus::Ongoing,
            message: None,
        }
    );

    let _ = add_record(
        "random_nft_id_2".to_string(),
        Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
            "Could not decode the principal."
        ),
        &SwapInfo::new(
            "randomSellId2".to_string(),
            [0u8; 32],
            Account {
                owner: Principal::anonymous(),
                subaccount: Some([0u8; 32]),
            },
            0,
            GldtNumTokens::new(Nat::from(0)).unwrap()
        ),
        RecordStatusInfo {
            status: RecordStatus::Ongoing,
            message: None,
        }
    );
}

fn dummy_sale_nft_request() -> SubscriberNotification {
    let token: TokenSpec = TokenSpec::ic(ICTokenSpec {
        id: None,
        fee: Some(Nat::from(10000)),
        decimals: Nat::from(8),
        canister: Principal::from_text(CANISTER_ID_GLDT_LEDGER).expect(
            "Could not decode the principal."
        ),
        standard: ICTokenSpec_standard::ICRC1,
        symbol: "GLDT".to_string(),
    });
    SubscriberNotification {
        escrow_info: SubAccountInfo {
            account_id: ByteBuf::from([
                105, 135, 242, 243, 165, 181, 162, 160, 1, 21, 162, 41, 181, 82, 9, 143, 106, 45,
                220, 234, 128, 124, 41, 191, 175, 77, 115, 154, 207, 39, 8, 14,
            ]),
            principal: Principal::from_text(TEST_PRINCIPAL_ID).expect(
                "Could not decode the principal."
            ),
            account_id_text: "6987f2f3a5b5a2a00115a229b552098f6a2ddcea807c29bfaf4d739acf27080e".to_string(),
            account: SubAccountInfo_account {
                principal: Principal::from_text(TEST_PRINCIPAL_ID).expect(
                    "Could not decode the principal."
                ),
                sub_account: ByteBuf::from([
                    199, 215, 43, 85, 161, 120, 243, 11, 166, 239, 227, 201, 223, 184, 203, 131,
                    205, 117, 219, 100, 109, 105, 126, 235, 115, 10, 77, 39, 179, 197, 134, 24,
                ]),
            },
        },
        sale: SaleStatusShared {
            token_id: "Gold-00001".to_string(),
            sale_type: SaleStatusShared_sale_type::auction(AuctionStateShared {
                status: AuctionStateShared_status::open,
                participants: Vec::from([
                    (
                        Principal::from_text(TEST_PRINCIPAL_ID).expect(
                            "Could not decode the principal."
                        ),
                        candid::Int::default(),
                    ),
                ]),
                token: token.clone(),
                current_bid_amount: Nat::from(0),
                winner: None,
                end_date: candid::Int::default(),
                start_date: candid::Int::default(),
                wait_for_quiet_count: Some(Nat::from(0)),
                current_escrow: None,
                allow_list: None,
                current_broker_id: None,
                min_next_bid: Nat::from(10000000000 as u64),
                config: PricingConfigShared::ask(
                    Some(
                        Vec::from([
                            AskFeature::buy_now(Nat::from(10000000000 as u64)),
                            AskFeature::notify(
                                Vec::from([
                                    Principal::from_text(CANISTER_ID_GLDT_CORE).expect(
                                        "Could not decode the principal."
                                    ),
                                ])
                            ),
                            AskFeature::token(token),
                        ])
                    )
                ),
            }),
            broker_id: None,
            original_broker_id: None,
            sale_id: "9f6c0fdc44d7cca4af7f1c25fbf1e92d86c317addf53577f08d73fadfa78e93f".to_string(),
        },
        seller: OrigynAccount::principal(
            Principal::from_text(
                "mqov6-tdewf-wfzea-raa7y-kxcpv-res3j-g3dc2-wklml-eamt2-5wt7h-fae"
            ).expect("Could not decode the principal.")
        ),
        collection: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect(
            "Could not decode the principal."
        ),
    }
}

// ------------------------- get_historical_swaps_by_user tests --------------------------
#[test]
fn test_get_historical_swaps_by_user_a1() {
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(1),
        limit: Some(50),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_a2() {
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(usize::MAX),
        limit: Some(50),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(res, Err("Overflow when calculating start".to_string()));
}

#[test]
fn test_get_historical_swaps_by_user_a3() {
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(50),
        limit: Some(usize::MAX),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_a4() {
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(1),
        limit: Some(usize::MAX),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_a5() {
    init_records();
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(usize::MAX),
        limit: Some(1),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 2,
            data: None,
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_a6() {
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(1),
        limit: Some(0),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_a7() {
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(0),
        limit: Some(1),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_a8() {
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: None,
        limit: Some(0),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_a9() {
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(0),
        limit: None,
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_a10() {
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: None,
        limit: Some(10),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_a11() {
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(10),
        limit: None,
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_b1() {
    init_records();
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(0),
        limit: Some(50),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 2,
            data: Some(
                vec![
                    GldtRecord::new(
                        RecordType::Mint,
                        0,
                        Account {
                            owner: Principal::anonymous(),
                            subaccount: Some([
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ]),
                        },
                        Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                            "Could not decode the principal."
                        ),
                        "random_nft_id_2".to_string(),
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                        ],
                        "randomSellId2".to_string(),
                        0,
                        GldtNumTokens::new(Nat::from(0)).unwrap(),
                        Nat::from(0),
                        RecordStatusInfo {
                            status: RecordStatus::Ongoing,
                            message: None,
                        }
                    ),
                    GldtRecord::new(
                        RecordType::Mint,
                        0,
                        Account {
                            owner: Principal::anonymous(),
                            subaccount: Some([
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ]),
                        },
                        Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect(
                            "Could not decode the principal."
                        ),
                        "random_nft_id_1".to_string(),
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                        ],
                        "randomSellId1".to_string(),
                        0,
                        GldtNumTokens::new(Nat::from(0)).unwrap(),
                        Nat::from(0),
                        RecordStatusInfo {
                            status: RecordStatus::Ongoing,
                            message: None,
                        }
                    )
                ]
            ),
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_b2() {
    init_records();
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(1),
        limit: Some(50),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 2,
            data: None,
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_b3() {
    init_records();
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(1),
        limit: Some(1),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 2,
            data: Some(
                vec![
                    GldtRecord::new(
                        RecordType::Mint,
                        0,
                        Account {
                            owner: Principal::anonymous(),
                            subaccount: Some([
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ]),
                        },
                        Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect(
                            "Could not decode the principal."
                        ),
                        "random_nft_id_1".to_string(),
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                        ],
                        "randomSellId1".to_string(),
                        0,
                        GldtNumTokens::new(Nat::from(0)).unwrap(),
                        Nat::from(0),
                        RecordStatusInfo {
                            status: RecordStatus::Ongoing,
                            message: None,
                        }
                    )
                ]
            ),
        })
    );
}

#[test]
fn test_get_historical_swaps_by_user_b4() {
    init_records();
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(0),
        limit: Some(1),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 2,
            data: Some(
                vec![
                    GldtRecord::new(
                        RecordType::Mint,
                        0,
                        Account {
                            owner: Principal::anonymous(),
                            subaccount: Some([
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ]),
                        },
                        Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                            "Could not decode the principal."
                        ),
                        "random_nft_id_2".to_string(),
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                        ],
                        "randomSellId2".to_string(),
                        0,
                        GldtNumTokens::new(Nat::from(0)).unwrap(),
                        Nat::from(0),
                        RecordStatusInfo {
                            status: RecordStatus::Ongoing,
                            message: None,
                        }
                    )
                ]
            ),
        })
    );
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

    assert_eq!(
        res,
        Err("invalid GLD NFT canister ID: was 2vxsx-fae, expected one of []".to_string())
    );
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

    assert_eq!(
        res,
        Err(
            format!(
                "invalid GLD NFT canister ID: was 2vxsx-fae, expected one of {:?}",
                vec![
                    Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect(
                        "Could not decode the principal."
                    ),
                    Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                        "Could not decode the principal."
                    )
                ]
            )
        )
    );
}

#[test]
fn test_get_status_of_swap_a3() {
    init_service();

    let status_request = GetStatusRequest {
        nft_id: "".to_string(),
        gld_nft_canister_id: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect(
            "Could not decode the principal."
        ),
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
        gld_nft_canister_id: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect(
            "Could not decode the principal."
        ),
        sale_id: "".to_string(),
    };

    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Ok(GetStatusResponse { status: None }));
}

#[test]
fn test_get_status_of_swap_b1() {
    init_service();

    REGISTRY.with(|r| {
        let registry = &mut r.borrow_mut();

        let _ = registry.init(
            &(
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ),
            SwapInfo::new(
                "randomSellId1".to_string(),
                [0u8; 32],
                Account {
                    owner: Principal::anonymous(),
                    subaccount: Some([0u8; 32]),
                },
                0,
                GldtNumTokens::new(Nat::from(0)).unwrap()
            )
        );
    });

    let status_request = GetStatusRequest {
        nft_id: "random_nft_id_1".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
            "Could not decode the principal."
        ),
        sale_id: "randomSellId1".to_string(),
    };

    let res = get_status_of_swap(status_request.clone());

    assert_eq!(
        res,
        Ok(GetStatusResponse {
            status: Some(SwappingStates::Initialised),
        })
    );
}

#[test]
fn test_get_status_of_swap_b2() {
    init_service();

    REGISTRY.with(|r| {
        let registry = &mut r.borrow_mut();

        let mut swap_info = SwapInfo::new(
            "randomSellId1".to_string(),
            [0u8; 32],
            Account {
                owner: Principal::anonymous(),
                subaccount: Some([0u8; 32]),
            },
            0,
            GldtNumTokens::new(Nat::from(0)).unwrap()
        );

        let _ = registry.init(
            &(
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ),
            swap_info.clone()
        );

        swap_info.set_ledger_entry(
            GldtLedgerEntry::Minted(
                GldtLedgerInfo::new(Nat::from(0), GldtNumTokens::new(Nat::from(0)).unwrap())
            )
        );

        let _ = registry.update_minted(
            &(
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ),
            swap_info.clone()
        );
    });

    let status_request = GetStatusRequest {
        nft_id: "random_nft_id_1".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
            "Could not decode the principal."
        ),
        sale_id: "randomSellId1".to_string(),
    };

    let res = get_status_of_swap(status_request.clone());

    assert_eq!(
        res,
        Ok(GetStatusResponse {
            status: Some(SwappingStates::Minted),
        })
    );
}

#[test]
fn test_get_status_of_swap_b3() {
    init_service();

    REGISTRY.with(|r| {
        let registry = &mut r.borrow_mut();

        let mut swap_info = SwapInfo::new(
            "randomSellId1".to_string(),
            [0u8; 32],
            Account {
                owner: Principal::anonymous(),
                subaccount: Some([0u8; 32]),
            },
            0,
            GldtNumTokens::new(Nat::from(0)).unwrap()
        );

        let _ = registry.init(
            &(
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ),
            swap_info.clone()
        );

        swap_info.set_ledger_entry(
            GldtLedgerEntry::Minted(
                GldtLedgerInfo::new(Nat::from(0), GldtNumTokens::new(Nat::from(0)).unwrap())
            )
        );

        let _ = registry.update_minted(
            &(
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ),
            swap_info.clone()
        );

        swap_info.set_swapped(GldtSwapped::new("randomSellId1".to_string(), Nat::from(0)));

        let _ = registry.update_swapped(
            &(
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ),
            swap_info.clone()
        );
    });

    let status_request = GetStatusRequest {
        nft_id: "random_nft_id_1".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
            "Could not decode the principal."
        ),
        sale_id: "randomSellId1".to_string(),
    };

    let res = get_status_of_swap(status_request.clone());

    assert_eq!(
        res,
        Ok(GetStatusResponse {
            status: Some(SwappingStates::Swapped),
        })
    );
}

// TEST FOR BURNING STATUS. Burning not yet implemented
// #[test]
// fn test_get_status_of_swap_b4() {
//     init_service();

//     REGISTRY.with(|r| {
//         let registry = &mut r.borrow_mut();

//         let mut swap_info = SwapInfo::new(
//             "randomSellId1".to_string(),
//             [0u8; 32],
//             Account {
//                 owner: Principal::anonymous(),
//                 subaccount: Some([0u8; 32]),
//             },
//             0,
//             GldtNumTokens::new(Nat::from(0)).unwrap()
//         );

//         let _ = registry.init(
//             (
//                 Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
//                     "Could not decode the principal."
//                 ),
//                 "random_nft_id_1".to_string(),
//             ).clone(),
//             swap_info.clone()
//         );

//         swap_info.set_ledger_entry(
//             GldtLedgerEntry::Minted(
//                 GldtLedgerInfo::new(Nat::from(0), GldtNumTokens::new(Nat::from(0)).unwrap())
//             )
//         );

//         let _ = registry.update_minted(
//             (
//                 Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
//                     "Could not decode the principal."
//                 ),
//                 "random_nft_id_1".to_string(),
//             ).clone(),
//             swap_info.clone()
//         );

//         swap_info.set_swapped(GldtSwapped::new("randomSellId1".to_string(), Nat::from(0)));

//         let _ = registry.update_swapped(
//             (
//                 Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
//                     "Could not decode the principal."
//                 ),
//                 "random_nft_id_1".to_string(),
//             ).clone(),
//             swap_info.clone()
//         );
//     });
//     let status_request = GetStatusRequest {
//         nft_id: "random_nft_id_1".to_string(),
//         gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
//             "Could not decode the principal."
//         ),
//         sale_id: "randomSellId1".to_string(),
//     };

//     let res = get_status_of_swap(status_request.clone());

//     assert_eq!(
//         res,
//         Ok(GetStatusResponse {
//             status: Some(SwappingStates::Burned),
//         })
//     );
// }

// #[test]
// fn test_get_status_of_swap_b5() {
//     init_service();
//     REGISTRY.with(|r| {
//         let registry = &mut r.borrow_mut();

//         let mut swap_info = SwapInfo::new(
//             "randomSellId1".to_string(),
//             [0u8; 32],
//             Account {
//                 owner: Principal::anonymous(),
//                 subaccount: Some([0u8; 32]),
//             },
//             0,
//             GldtNumTokens::new(Nat::from(0)).unwrap()
//         );

//         let _ = registry.init(
//             (
//                 Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
//                     "Could not decode the principal."
//                 ),
//                 "random_nft_id_1".to_string(),
//             ).clone(),
//             swap_info.clone()
//         );

//         swap_info.set_swapped(GldtSwapped::new("randomSellId1".to_string(), Nat::from(0)));

//         let _ = registry.update_swapped(
//             (
//                 Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
//                     "Could not decode the principal."
//                 ),
//                 "random_nft_id_1".to_string(),
//             ).clone(),
//             swap_info.clone()
//         );
//     });

//     let status_request = GetStatusRequest {
//         nft_id: "random_nft_id_1".to_string(),
//         gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
//             "Could not decode the principal."
//         ),
//         sale_id: "randomSellId1".to_string(),
//     };

//     let res = get_status_of_swap(status_request.clone());

//     assert_eq!(res, Err("Swap status is corrupted.".to_string()));
// }

#[test]
fn test_get_status_of_swap_b6() {
    init_service();

    REGISTRY.with(|r| {
        let registry = &mut r.borrow_mut();

        let swap_info = SwapInfo::new(
            "randomSellId1".to_string(),
            [0u8; 32],
            Account {
                owner: Principal::anonymous(),
                subaccount: Some([0u8; 32]),
            },
            0,
            GldtNumTokens::new(Nat::from(0)).unwrap()
        );

        let _ = registry.init(
            &(
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ),
            swap_info.clone()
        );
    });

    let status_request = GetStatusRequest {
        nft_id: "random_nft_id_1".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
            "Could not decode the principal."
        ),
        sale_id: "".to_string(),
    };

    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Ok(GetStatusResponse { status: None }));
}

#[test]
fn test_get_status_of_swap_b7() {
    init_service();

    REGISTRY.with(|r| {
        let registry = &mut r.borrow_mut();

        let swap_info = SwapInfo::new(
            "randomSellId1".to_string(),
            [0u8; 32],
            Account {
                owner: Principal::anonymous(),
                subaccount: Some([0u8; 32]),
            },
            0,
            GldtNumTokens::new(Nat::from(0)).unwrap()
        );

        let _ = registry.init(
            &(
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ),
            swap_info.clone()
        );
    });

    let status_request = GetStatusRequest {
        nft_id: "".to_string(),
        gld_nft_canister_id: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
            "Could not decode the principal."
        ),
        sale_id: "randomSellId1".to_string(),
    };

    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Ok(GetStatusResponse { status: None }));
}

// // --------------------------------- get_records ----------------------------------
#[test]
fn test_get_records_a1() {
    let records_request = GetRecordsRequest {
        page: Some(1),
        limit: Some(50),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_records_a2() {
    let records_request = GetRecordsRequest {
        page: Some(usize::MAX),
        limit: Some(50),
    };

    let res = get_records(records_request);

    assert_eq!(res, Err("Overflow when calculating start".to_string()));
}

#[test]
fn test_get_records_a3() {
    let records_request = GetRecordsRequest {
        page: Some(50),
        limit: Some(usize::MAX),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_records_a4() {
    let records_request = GetRecordsRequest {
        page: Some(usize::MAX),
        limit: Some(1),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_records_a5() {
    let records_request = GetRecordsRequest {
        page: Some(1),
        limit: Some(usize::MAX),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_records_a6() {
    let records_request = GetRecordsRequest {
        page: Some(1),
        limit: Some(0),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_records_a7() {
    let records_request = GetRecordsRequest {
        page: Some(0),
        limit: Some(1),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_records_a8() {
    let records_request = GetRecordsRequest {
        page: Some(0),
        limit: None,
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_records_a9() {
    let records_request = GetRecordsRequest {
        page: None,
        limit: Some(0),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_records_a10() {
    let records_request = GetRecordsRequest {
        page: Some(10),
        limit: None,
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_records_a11() {
    let records_request = GetRecordsRequest {
        page: None,
        limit: Some(10),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 0,
            data: None,
        })
    );
}

#[test]
fn test_get_records_b1() {
    init_records();

    let records_request = GetRecordsRequest {
        page: Some(1),
        limit: Some(50),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 2,
            data: None,
        })
    );
}

#[test]
fn test_get_records_b2() {
    init_records();

    let records_request = GetRecordsRequest {
        page: Some(0),
        limit: Some(50),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 2,
            data: Some(
                vec![
                    GldtRecord::new(
                        RecordType::Mint,
                        0,
                        Account {
                            owner: Principal::anonymous(),
                            subaccount: Some([
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ]),
                        },
                        Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect(
                            "Could not decode the principal."
                        ),
                        "random_nft_id_1".to_string(),
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                        ],
                        "randomSellId1".to_string(),
                        0,
                        GldtNumTokens::new(Nat::from(0)).unwrap(),
                        Nat::from(0),
                        RecordStatusInfo {
                            status: RecordStatus::Ongoing,
                            message: None,
                        }
                    ),
                    GldtRecord::new(
                        RecordType::Mint,
                        0,
                        Account {
                            owner: Principal::anonymous(),
                            subaccount: Some([
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ]),
                        },
                        Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                            "Could not decode the principal."
                        ),
                        "random_nft_id_2".to_string(),
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                        ],
                        "randomSellId2".to_string(),
                        0,
                        GldtNumTokens::new(Nat::from(0)).unwrap(),
                        Nat::from(0),
                        RecordStatusInfo {
                            status: RecordStatus::Ongoing,
                            message: None,
                        }
                    )
                ]
            ),
        })
    );
}

#[test]
fn test_get_records_b3() {
    init_records();

    let records_request = GetRecordsRequest {
        page: Some(0),
        limit: Some(1),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 2,
            data: Some(
                vec![
                    GldtRecord::new(
                        RecordType::Mint,
                        0,
                        Account {
                            owner: Principal::anonymous(),
                            subaccount: Some([
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ]),
                        },
                        Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect(
                            "Could not decode the principal."
                        ),
                        "random_nft_id_1".to_string(),
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                        ],
                        "randomSellId1".to_string(),
                        0,
                        GldtNumTokens::new(Nat::from(0)).unwrap(),
                        Nat::from(0),
                        RecordStatusInfo {
                            status: RecordStatus::Ongoing,
                            message: None,
                        }
                    )
                ]
            ),
        })
    );
}

#[test]
fn test_get_records_b4() {
    init_records();

    let records_request = GetRecordsRequest {
        page: Some(1),
        limit: Some(1),
    };

    let res = get_records(records_request);

    assert_eq!(
        res,
        Ok(GetRecordsResponse {
            total: 2,
            data: Some(
                vec![
                    GldtRecord::new(
                        RecordType::Mint,
                        0,
                        Account {
                            owner: Principal::anonymous(),
                            subaccount: Some([
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ]),
                        },
                        Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                            "Could not decode the principal."
                        ),
                        "random_nft_id_2".to_string(),
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0,
                        ],
                        "randomSellId2".to_string(),
                        0,
                        GldtNumTokens::new(Nat::from(0)).unwrap(),
                        Nat::from(0),
                        RecordStatusInfo {
                            status: RecordStatus::Ongoing,
                            message: None,
                        }
                    )
                ]
            ),
        })
    );
}

// ---------------------------------- nft_info --------------------------------
#[test]
fn test_nft_info_a1() {
    let info_request = InfoRequest {
        source_canister: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect(
            "Could not decode the principal."
        ),
        nft_id: "random_nft_id_1".to_string(),
    };

    let res = nft_info(info_request);

    assert_eq!(res, NftInfo { info: None });
}

#[test]
fn test_nft_info_a2() {
    init_service();

    let swap_info = SwapInfo::new(
        "randomSellId1".to_string(),
        [0u8; 32],
        Account {
            owner: Principal::anonymous(),
            subaccount: Some([0u8; 32]),
        },
        0,
        GldtNumTokens::new(Nat::from(0)).unwrap()
    );

    REGISTRY.with(|r| {
        let registry = &mut r.borrow_mut();

        let _ = registry.init(
            &(
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ),
            swap_info.clone()
        );
    });

    let info_request = InfoRequest {
        source_canister: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
            "Could not decode the principal."
        ),
        nft_id: "random_nft_id_1".to_string(),
    };

    let res = nft_info(info_request);

    assert_eq!(res, NftInfo {
        info: Some(GldtRegistryEntry::new(swap_info)),
    });
}

#[test]
fn test_nft_info_a3() {
    init_service();

    REGISTRY.with(|r| {
        let registry = &mut r.borrow_mut();

        let _ = registry.init(
            &(
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ),
            SwapInfo::new(
                "randomSellId1".to_string(),
                [0u8; 32],
                Account {
                    owner: Principal::anonymous(),
                    subaccount: Some([0u8; 32]),
                },
                0,
                GldtNumTokens::new(Nat::from(0)).unwrap()
            )
        );
    });

    let info_request = InfoRequest {
        source_canister: Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
            "Could not decode the principal."
        ),
        nft_id: "random_nft_id_2".to_string(),
    };

    let res = nft_info(info_request);

    assert_eq!(res, NftInfo { info: None });
}

// ------------------------- get_locked_info tests -------------------------

#[test]
fn test_get_locked_info_a1() {
    let num_entries_per_weight = 0;

    init_service();
    init_registry(num_entries_per_weight);
    update_registry_to_swapped();

    let res = get_locked_info();

    assert_eq!(res, LockedInfoResponse {
        total_number_of_bars_locked: 0,
        total_weight_locked: 0,
    });
}

#[test]
fn test_get_locked_info_a2() {
    let num_entries_per_weight = 10;

    init_service();
    init_registry(num_entries_per_weight);
    update_registry_to_swapped();

    let res = get_locked_info();

    assert_eq!(res, LockedInfoResponse {
        total_number_of_bars_locked: num_entries_per_weight * 2,
        total_weight_locked: num_entries_per_weight * 11, // x 11 because only 1g and 10g are in the registry and an equal number is present
    });
}
