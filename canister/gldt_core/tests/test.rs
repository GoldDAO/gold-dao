// use icrc_ledger_types::icrc1::{
//     account::{ Account },
// };
// use candid::{ Principal };
// use gldt_core::*;

// #[test]
// fn test_get_swaps_by_user() {
    // init(Some(Conf::new(
    //     Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").expect("Could not decode the principal."),
    //     vec![
    //         (Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").expect("Could not decode the principal."), NftCanisterConf::new(1)),
    //         (Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").expect("Could not decode the principal."), NftCanisterConf::new(10))
    //     ]
    // )));

    // let account = Account {
    //     owner: Principal::anonymous(), 
    //     subaccount: None
    // };

    // let result = get_swaps_by_user(Some(account), Some(1), Some(50));
    // let result = get_swaps_by_user(Some(account), Some(u32::MAX), Some(50));
    // let result = get_swaps_by_user(Some(account), Some(1), Some(u32::MAX));
    // let result = get_swaps_by_user(Some(account), Some(1), Some(0));
    // let result = get_swaps_by_user(Some(account), None, Some(0));
    // let result = get_swaps_by_user(Some(account), Some(0), None);
    // let result = get_swaps_by_user(Some(account), None, Some(10));
    // let result = get_swaps_by_user(Some(account), Some(10), None);
    // println!("result = {:?}", result);
    // println!("account = {:?}", account);
// }
