use std::str::FromStr;

use candid::Principal;
use ic_ledger_types::Tokens;
use icrc_ledger_types::icrc1::account::Account;

// TODO: Fill this with principals of Foundation
pub const TEAM_PRINCIPALS: [Account; 3] = [
    // fv4gl-4digp-v54d7-yclkh-mokfc-3ugji-uim47-pb357-6cjtl-bz5sf-gqe
    Account {
        owner: Principal::from_slice(
            &[
                104, 51, 235, 222, 15, 248, 18, 212, 118, 57, 69, 22, 232, 100, 162, 136, 103, 62,
                240, 239, 191, 240, 147, 53, 135, 61, 145, 77, 2,
            ]
        ),
        subaccount: None,
    },
    // 3432o-lw3ke-mhdyc-yejn7-uh2f4-jhxji-qyovs-47lfe-kd7eh-aenvp-uqe
    Account {
        owner: Principal::from_slice(
            &[
                219, 81, 24, 113, 224, 88, 34, 91, 250, 31, 69, 226, 79, 116, 162, 24, 117, 101,
                207, 172, 164, 80, 254, 67, 128, 141, 171, 233, 2,
            ]
        ),
        subaccount: None,
    },
    // SNS Rewards Canister
    // 2f5ll-gqaaa-aaaak-qcfuq-cai
    Account {
        owner: Principal::from_slice(&[0, 0, 0, 0, 1, 80, 17, 105, 1, 1]),
        subaccount: Some([
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ]),
    },
];

