use icrc_ledger_types::icrc1::{
    account::{ Account },
};
use candid::{ Principal };
use super::*;

#[test]
fn test_get_swaps_by_user() {
    // init(Some(Conf::new(
    //     Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").expect("Could not decode the principal."),
    //     vec![
    //         (Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."), NftCanisterConf::new(1)),
    //         (Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), NftCanisterConf::new(10))
    //     ]
    // )));

    let account = Account {
        owner: Principal::anonymous(), 
        subaccount: None
    };
    
    let _ = get_swaps_by_user(Some(account), Some(1), Some(50));
    // assert_eq!(_, "");
    let _ = get_swaps_by_user(Some(account), Some(u32::MAX), Some(50));
    let _ = get_swaps_by_user(Some(account), Some(1), Some(u32::MAX));
    let _ = get_swaps_by_user(Some(account), Some(1), Some(0));
    let _ = get_swaps_by_user(Some(account), None, Some(0));
    let _ = get_swaps_by_user(Some(account), Some(0), None);
    let _ = get_swaps_by_user(Some(account), None, Some(10));
    let _ = get_swaps_by_user(Some(account), Some(10), None);
}

#[test]
fn test_get_status_of_swap_1() {
    // init(Some(Conf::new(
    //     Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").expect("Could not decode the principal."),
    //     vec![
    //         (Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."), NftCanisterConf::new(1)),
    //         (Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), NftCanisterConf::new(10))
    //     ]
    // )));

    let status_request = GetStatusRequest {
        nft_id: "".to_string(),
        gld_nft_canister_id: Principal::anonymous(),
        sale_id: "".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());

    assert_eq!(res, Err("invalid GLD NFT canister ID: was 2vxsx-fae, expected one of []".to_string()));
}

#[test]
fn test_get_status_of_swap_2() {
    init(Some(Conf::new(
        Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").expect("Could not decode the principal."),
        vec![
            (Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."), NftCanisterConf::new(1)),
            (Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), NftCanisterConf::new(10))
        ]
    )));

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
fn test_get_status_of_swap_3() {
    init(Some(Conf::new(
        Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").expect("Could not decode the principal."),
        vec![
            (Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."), NftCanisterConf::new(1)),
            (Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), NftCanisterConf::new(10))
        ]
    )));

    let status_request = GetStatusRequest {
        nft_id: "".to_string(),
        gld_nft_canister_id: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."),
        sale_id: "".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());
    
    assert_eq!(res, Ok(GetStatusResponse { status: None }));


}

#[test]
fn test_get_status_of_swap_4() {
    init(Some(Conf::new(
        Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").expect("Could not decode the principal."),
        vec![
            (Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."), NftCanisterConf::new(1)),
            (Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), NftCanisterConf::new(10))
        ]
    )));

    let status_request = GetStatusRequest {
        nft_id: "".to_string(),
        gld_nft_canister_id: Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."),
        sale_id: "".to_string(),
    };
    
    let res = get_status_of_swap(status_request.clone());
    
    assert_eq!(res, Ok(GetStatusResponse { status: None }));


}
