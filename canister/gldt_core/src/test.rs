use crate::declarations::gld_nft::{ SubAccountInfo_account, CandyShared };

use super::*;
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;

use declarations::gld_nft::{
    Account as OrigynAccount,
    AskFeature,
    AuctionStateShared,
    PricingConfigShared,
    AuctionStateShared_status,
    ICTokenSpec,
    ICTokenSpec_standard,
    SaleStatusShared,
    SaleStatusShared_sale_type,
    SubAccountInfo,
    TokenSpec,
};
use serde_bytes::ByteBuf;
use records::{ GldtRecord };

// --------------------------------- constants ----------------------------------

const CANISTER_ID_GLDT_CORE: &str = "m45be-jaaaa-aaaak-qcgnq-cai";
const CANISTER_ID_GLDT_LEDGER: &str = "6uad6-fqaaa-aaaam-abovq-cai";
const CANISTER_ID_GLD_NFT_1G: &str = "obapm-2iaaa-aaaak-qcgca-cai";
const CANISTER_ID_YUMI_KYC: &str = "2qft3-raaaa-aaaag-qci4a-cai";
const TEST_PRINCIPAL_ID: &str = "thrhh-hnmzu-kjquw-6ebmf-vdhed-yf2ry-avwy7-2jrrm-byg34-zoqaz-wqe";

// --------------------------------- helpers ----------------------------------
fn init_service() {
    init(
        Some(
            Conf::new(
                Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").expect(
                    "Could not decode the principal."
                ),
                vec![
                    (
                        Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect(
                            "Could not decode the principal."
                        ),
                        NftCanisterConf::new(1),
                    ),
                    (
                        Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                            "Could not decode the principal."
                        ),
                        NftCanisterConf::new(10),
                    )
                ]
            )
        )
    );
}

fn init_records() {
    let _ = add_record(
        "random_nft_id_1".to_string(),
        Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect(
            "Could not decode the principal."
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
        SwapInfo::new(
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
        page: Some(u32::MAX),
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
        limit: Some(u32::MAX),
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
        limit: Some(u32::MAX),
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
    let account = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let get_swap_request: GetSwapsRequest = GetSwapsRequest {
        account: Some(account),
        page: Some(u32::MAX),
        limit: Some(1),
    };

    let res = get_historical_swaps_by_user(get_swap_request);

    assert_eq!(res, Err("Overflow when calculating end".to_string()));
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
                        RecordStatusInfo { status: RecordStatus::Ongoing, message: None }
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
                        RecordStatusInfo { status: RecordStatus::Ongoing, message: None }
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
                        RecordStatusInfo { status: RecordStatus::Ongoing, message: None }
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
                        RecordStatusInfo { status: RecordStatus::Ongoing, message: None }
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
            (
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ).clone(),
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
            (
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ).clone(),
            swap_info.clone()
        );

        swap_info.set_ledger_entry(
            GldtLedgerEntry::Minted(
                GldtLedgerInfo::new(Nat::from(0), GldtNumTokens::new(Nat::from(0)).unwrap())
            )
        );

        let _ = registry.update_minted(
            (
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ).clone(),
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
            (
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ).clone(),
            swap_info.clone()
        );

        swap_info.set_ledger_entry(
            GldtLedgerEntry::Minted(
                GldtLedgerInfo::new(Nat::from(0), GldtNumTokens::new(Nat::from(0)).unwrap())
            )
        );

        let _ = registry.update_minted(
            (
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ).clone(),
            swap_info.clone()
        );

        swap_info.set_swapped(GldtSwapped::new("randomSellId1".to_string(), Nat::from(0)));

        let _ = registry.update_swapped(
            (
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ).clone(),
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
            (
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ).clone(),
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
            (
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ).clone(),
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
// #[test]
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
        page: Some(u32::MAX),
        limit: Some(50),
    };

    let res = get_records(records_request);

    assert_eq!(res, Err("Overflow when calculating start".to_string()));
}

#[test]
fn test_get_records_a3() {
    let records_request = GetRecordsRequest {
        page: Some(50),
        limit: Some(u32::MAX),
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
        page: Some(u32::MAX),
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
        limit: Some(u32::MAX),
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
                        RecordStatusInfo { status: RecordStatus::Ongoing, message: None }
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
                        RecordStatusInfo { status: RecordStatus::Ongoing, message: None }
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
                        RecordStatusInfo { status: RecordStatus::Ongoing, message: None }
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
                        RecordStatusInfo { status: RecordStatus::Ongoing, message: None }
                    )
                ]
            ),
        })
    );
}

// // ------------------------- notify_sale_nft_origyn tests -----------------------------

#[tokio::test]
async fn test_notify_sale_nft_origyn_a1() {
    init_service();

    let mut sale_nft_request: SubscriberNotification = dummy_sale_nft_request();

    sale_nft_request.collection = Principal::anonymous();

    let res = notify_sale_nft_origyn(sale_nft_request).await;
    assert_eq!(
        res,
        Err(
            "ERROR :: invalid caller: was 2vxsx-fae, expected one of [Principal { len: 10, bytes: [0, 0, 0, 0, 1, 80, 17, 132, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, Principal { len: 10, bytes: [0, 0, 0, 0, 1, 112, 15, 122, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }]".to_string()
        )
    );
}

#[tokio::test]
async fn test_notify_sale_nft_origyn_a2() {
    init_service();

    let mut sale_nft_request = dummy_sale_nft_request();

    sale_nft_request.sale.token_id = "".to_string();

    let res = notify_sale_nft_origyn(sale_nft_request).await;
    assert_eq!(res, Err("ERROR :: NFT ID cannot be empty".to_string()));
}

#[tokio::test]
async fn test_notify_sale_nft_origyn_a3() {
    init_service();

    let mut sale_nft_request: SubscriberNotification = dummy_sale_nft_request();

    sale_nft_request.escrow_info.account.sub_account = ByteBuf::from([
        199, 215, 43, 85, 161, 120, 243, 11, 166, 239, 227, 201, 223, 184, 203, 131, 205, 117, 219, 100,
        109, 105, 126, 235, 115, 10, 77, 39, 179, 197, 134,
    ]);

    let res = notify_sale_nft_origyn(sale_nft_request).await;
    assert_eq!(
        res,
        Err("ERROR :: ERROR: expected a subaccount of length 32 but it was 31".to_string())
    );
}

#[tokio::test]
async fn test_notify_sale_nft_origyn_a4() {
    init_service();

    let token: TokenSpec = TokenSpec::ic(ICTokenSpec {
        id: None,
        fee: Some(Nat::from(0)),
        decimals: Nat::from(8),
        canister: Principal::from_text(CANISTER_ID_GLDT_LEDGER).expect(
            "Could not decode the principal."
        ),
        standard: ICTokenSpec_standard::ICRC1,
        symbol: "GLDT".to_string(),
    });

    let mut sale_nft_request: SubscriberNotification = dummy_sale_nft_request();

    let SaleStatusShared_sale_type::auction(ref mut t) = sale_nft_request.sale.sale_type;

    t.token = token.clone();

    let res = notify_sale_nft_origyn(sale_nft_request).await;

    assert_eq!(
        res,
        Err(
            "ERROR :: Token specification are not correct. Expected ic(ICTokenSpec { id: None, fee: Some(Nat(10000)), decimals: Nat(8), canister: Principal { len: 10, bytes: [0, 0, 0, 0, 1, 128, 11, 171, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, standard: ICRC1, symbol: \"GLDT\" }), received: ic(ICTokenSpec { id: None, fee: Some(Nat(0)), decimals: Nat(8), canister: Principal { len: 10, bytes: [0, 0, 0, 0, 1, 128, 11, 171, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, standard: ICRC1, symbol: \"GLDT\" })".to_string()
        )
    );
}

#[tokio::test]
async fn test_notify_sale_nft_origyn_a5() {
    init_service();

    let token: TokenSpec = TokenSpec::ic(ICTokenSpec {
        id: None,
        fee: Some(Nat::from(GLDT_TX_FEE)),
        decimals: Nat::from(1),
        canister: Principal::from_text(CANISTER_ID_GLDT_LEDGER).expect(
            "Could not decode the principal."
        ),
        standard: ICTokenSpec_standard::ICRC1,
        symbol: "GLDT".to_string(),
    });

    let mut sale_nft_request: SubscriberNotification = dummy_sale_nft_request();

    let SaleStatusShared_sale_type::auction(ref mut t) = sale_nft_request.sale.sale_type;

    t.token = token.clone();

    let res = notify_sale_nft_origyn(sale_nft_request).await;

    assert_eq!(
        res,
        Err(
            "ERROR :: Token specification are not correct. Expected ic(ICTokenSpec { id: None, fee: Some(Nat(10000)), decimals: Nat(8), canister: Principal { len: 10, bytes: [0, 0, 0, 0, 1, 128, 11, 171, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, standard: ICRC1, symbol: \"GLDT\" }), received: ic(ICTokenSpec { id: None, fee: Some(Nat(10000)), decimals: Nat(1), canister: Principal { len: 10, bytes: [0, 0, 0, 0, 1, 128, 11, 171, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, standard: ICRC1, symbol: \"GLDT\" })".to_string()
        )
    );
}

#[tokio::test]
async fn test_notify_sale_nft_origyn_a6() {
    init_service();

    let token: TokenSpec = TokenSpec::ic(ICTokenSpec {
        id: None,
        fee: Some(Nat::from(GLDT_TX_FEE)),
        decimals: Nat::from(8),
        canister: Principal::from_text(CANISTER_ID_GLDT_LEDGER).expect(
            "Could not decode the principal."
        ),
        standard: ICTokenSpec_standard::EXTFungible,
        symbol: "GLDT".to_string(),
    });

    let mut sale_nft_request: SubscriberNotification = dummy_sale_nft_request();

    let SaleStatusShared_sale_type::auction(ref mut t) = sale_nft_request.sale.sale_type;

    t.token = token.clone();

    let res = notify_sale_nft_origyn(sale_nft_request).await;

    assert_eq!(
        res,
        Err(
            "ERROR :: Token specification are not correct. Expected ic(ICTokenSpec { id: None, fee: Some(Nat(10000)), decimals: Nat(8), canister: Principal { len: 10, bytes: [0, 0, 0, 0, 1, 128, 11, 171, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, standard: ICRC1, symbol: \"GLDT\" }), received: ic(ICTokenSpec { id: None, fee: Some(Nat(10000)), decimals: Nat(8), canister: Principal { len: 10, bytes: [0, 0, 0, 0, 1, 128, 11, 171, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, standard: EXTFungible, symbol: \"GLDT\" })".to_string()
        )
    );
}

#[tokio::test]
async fn test_notify_sale_nft_origyn_a7() {
    init_service();

    let token: TokenSpec = TokenSpec::ic(ICTokenSpec {
        id: None,
        fee: Some(Nat::from(GLDT_TX_FEE)),
        decimals: Nat::from(8),
        canister: Principal::from_text(CANISTER_ID_GLDT_LEDGER).expect(
            "Could not decode the principal."
        ),
        standard: ICTokenSpec_standard::EXTFungible,
        symbol: "GLDT2".to_string(),
    });

    let mut sale_nft_request: SubscriberNotification = dummy_sale_nft_request();

    let SaleStatusShared_sale_type::auction(ref mut t) = sale_nft_request.sale.sale_type;

    t.token = token.clone();

    let res = notify_sale_nft_origyn(sale_nft_request).await;

    assert_eq!(
        res,
        Err(
            "ERROR :: Token specification are not correct. Expected ic(ICTokenSpec { id: None, fee: Some(Nat(10000)), decimals: Nat(8), canister: Principal { len: 10, bytes: [0, 0, 0, 0, 1, 128, 11, 171, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, standard: ICRC1, symbol: \"GLDT\" }), received: ic(ICTokenSpec { id: None, fee: Some(Nat(10000)), decimals: Nat(8), canister: Principal { len: 10, bytes: [0, 0, 0, 0, 1, 128, 11, 171, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }, standard: EXTFungible, symbol: \"GLDT2\" })".to_string()
        )
    );
}

#[tokio::test]
async fn test_notify_sale_nft_origyn_a8() {
    init_service();

    let token: TokenSpec = TokenSpec::ic(ICTokenSpec {
        id: None,
        fee: Some(Nat::from(GLDT_TX_FEE)),
        decimals: Nat::from(8),
        canister: Principal::from_text(CANISTER_ID_GLDT_LEDGER).expect(
            "Could not decode the principal."
        ),
        standard: ICTokenSpec_standard::EXTFungible,
        symbol: "GLDT".to_string(),
    });

    let config = PricingConfigShared::ask(
        Some(
            Vec::from([
                AskFeature::reserve(Nat::from(10000000000 as u64)),
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
    );

    let mut sale_nft_request: SubscriberNotification = dummy_sale_nft_request();

    let SaleStatusShared_sale_type::auction(ref mut t) = sale_nft_request.sale.sale_type;

    t.config = config.clone();

    let res = notify_sale_nft_origyn(sale_nft_request).await;

    assert_eq!(
        res,
        Err(
            "ERROR :: Unexpected feature in asked, only token, notify and buy_now accepted and received AskFeature::reserve(Nat(10000000000))".to_string()
        )
    );
}

#[tokio::test]
async fn test_notify_sale_nft_origyn_a9() {
    init_service();

    let token: TokenSpec = TokenSpec::ic(ICTokenSpec {
        id: None,
        fee: Some(Nat::from(GLDT_TX_FEE)),
        decimals: Nat::from(8),
        canister: Principal::from_text(CANISTER_ID_GLDT_LEDGER).expect(
            "Could not decode the principal."
        ),
        standard: ICTokenSpec_standard::EXTFungible,
        symbol: "GLDT".to_string(),
    });

    let config: PricingConfigShared = PricingConfigShared::extensible(
        Box::new(CandyShared::Nat64(1 as u64))
    );

    let mut sale_nft_request: SubscriberNotification = dummy_sale_nft_request();

    let SaleStatusShared_sale_type::auction(ref mut t) = sale_nft_request.sale.sale_type;

    t.config = config.clone();

    let res = notify_sale_nft_origyn(sale_nft_request).await;

    assert_eq!(
        res,
        Err(
            "ERROR :: Unexpected pricing_config_shared value, only ask value is accepted and received PricingConfigShared::extensible(Nat64(1))".to_string()
        )
    );
}

// // ---------------------------------- nft_info --------------------------------
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
            (
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ).clone(),
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
            (
                Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect(
                    "Could not decode the principal."
                ),
                "random_nft_id_1".to_string(),
            ).clone(),
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
