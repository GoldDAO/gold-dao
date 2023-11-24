use super::*;
use serde_json::{ json, Value };
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use registry;

// ------------------- COMPENSATION_FACTOR -------------------
#[test]
fn test_get_compensation_factor_b1() {
    let factor = get_compensation_factor();

    assert_eq!(factor, 10);
}

#[test]
fn test_set_compensation_factor_b1() {
    let _ = set_compensation_factor(5);
    let factor = get_compensation_factor();

    assert_eq!(factor, 5);
}
#[test]
fn test_set_compensation_factor_b2() {
    let res = set_compensation_factor(15);

    let factor = get_compensation_factor();

    assert_eq!(factor, 10);
    assert_eq!(
        res,
        Err(
            CustomError::new_with_message(
                ErrorType::Other,
                "Compensation factor value has to be between (including) 1 and 10 (mean 0.1% and 1%).".to_string()
            )
        )
    );
}
#[test]
fn test_set_compensation_factor_b3() {
    let res = set_compensation_factor(0);

    let factor = get_compensation_factor();

    assert_eq!(factor, 10);
    assert_eq!(
        res,
        Err(
            CustomError::new_with_message(
                ErrorType::Other,
                "Compensation factor value has to be between (including) 1 and 10 (mean 0.1% and 1%).".to_string()
            )
        )
    );
}

// ------------------- EXPORT -------------------
#[test]
fn test_export() {
    let right =
        json!({
        "registry": {},
        "configuration": {
            "compensation_factor": 10,
            "enabled":false,
            "execution_delay_secs": 20,
            "fallback_timer_interval_secs": 3600,
            "gld_nft_canister_conf":[],
            "gldt_canister_id":"2vxsx-fae",
            "gldt_ledger_canister_id":"2vxsx-fae"
        },
        "managers": [],
    }).to_string();

    let export = fetch_metadata();

    assert_eq!(export, right);
}

#[test]
fn test_export_2() {
    let key: (Account, String) = (
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        "tmp".to_string(),
    );

    let fee = FeeRegistryEntry {
        amount: Nat::from(0),
        block_height: None,
        gld_nft_canister_id: Principal::anonymous(),
        history_index: Nat::from(0),
        status: registry::Status::Success,
        timestamp: 0,
        previous_entry: None,
    };

    REGISTRY.with(|cell| {
        let mut registry = cell.borrow_mut();
        registry.init_entry(&key, &fee);
    });

    let right =
        json!({
        "registry": {
            "2vxsx-fae|tmp": fee
        },
        "configuration": {
            "compensation_factor": 10,
            "enabled":false,
            "execution_delay_secs": 20,
            "fallback_timer_interval_secs": 3600,
            "gld_nft_canister_conf":[],
            "gldt_canister_id":"2vxsx-fae",
            "gldt_ledger_canister_id":"2vxsx-fae"
        },
        "managers": [],
    }).to_string();

    let export = fetch_metadata();

    assert_eq!(export, right);
}

// ------------------- Import -------------------

#[test]
fn test_import() {
    let fee = FeeRegistryEntry {
        amount: Nat::from(0),
        block_height: None,
        gld_nft_canister_id: Principal::anonymous(),
        history_index: Nat::from(0),
        status: registry::Status::Success,
        timestamp: 0,
        previous_entry: None,
    };

    let import =
        json!({
        "registry": {
            "2vxsx-fae|tmp": fee
        },
        "configuration": {
            "compensation_factor": 10,
            "enabled":false,
            "execution_delay_secs": 20,
            "fallback_timer_interval_secs": 3600,
            "gld_nft_canister_conf":[],
            "gldt_canister_id":"obapm-2iaaa-aaaak-qcgca-cai",
            "gldt_ledger_canister_id":"obapm-2iaaa-aaaak-qcgca-cai"
        },
        "managers": [],
    }).to_string();

    let import = import_data(import);

    let right =
        json!({
        "registry": {
            "2vxsx-fae|tmp": fee
        },
        "configuration": {
            "compensation_factor": 10,
            "enabled":false,
            "execution_delay_secs": 20,
            "fallback_timer_interval_secs": 3600,
            "gld_nft_canister_conf":[],
            "gldt_canister_id":"obapm-2iaaa-aaaak-qcgca-cai",
            "gldt_ledger_canister_id":"obapm-2iaaa-aaaak-qcgca-cai"
        },
        "managers": [],
    }).to_string();

    let export = fetch_metadata();

    assert_eq!(
        import,
        Ok(
            json!({
        "registry": {},
        "configuration": {
            "compensation_factor": 10,
            "enabled":false,
            "execution_delay_secs": 20,
            "fallback_timer_interval_secs": 3600,
            "gld_nft_canister_conf":[],
            "gldt_canister_id":"2vxsx-fae",
            "gldt_ledger_canister_id":"2vxsx-fae"
        },
        "managers": [],
    }).to_string()
        )
    );
    assert_eq!(export, right);
}
