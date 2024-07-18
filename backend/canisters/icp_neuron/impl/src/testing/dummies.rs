use std::collections::HashMap;

use candid::Principal;
use icrc_ledger_types::icrc1::account::{ Account, Subaccount };
use nns_governance_canister::types::{ Neuron, NeuronId };

pub fn dummy_principal() -> Principal {
    Principal::from_text("thrhh-hnmzu-kjquw-6ebmf-vdhed-yf2ry-avwy7-2jrrm-byg34-zoqaz-wqe").unwrap()
}

pub fn dummy_account(subaccount: Option<Subaccount>) -> Account {
    Account {
        owner: dummy_principal(),
        subaccount,
    }
}

// pub fn dummy_nns_neuron(id: Option<u64>) -> Neuron {
//     Neuron {
//         id: Some(NeuronId { id: id.unwrap_or(17_481_076_647_658_761_488) }),
//         account: vec![
//             149,
//             128,
//             178,
//             23,
//             182,
//             54,
//             48,
//             115,
//             178,
//             174,
//             154,
//             119,
//             21,
//             182,
//             104,
//             106,
//             141,
//             106,
//             190,
//             141,
//             3,
//             144,
//             216,
//             56,
//             228,
//             185,
//             230,
//             194,
//             1,
//             119,
//             126,
//             193
//         ],
//         controller: None,
//         hot_keys: vec![],
//         cached_neuron_stake_e8s: 0,
//         neuron_fees_e8s: 0,
//         created_timestamp_seconds: 0,
//         aging_since_timestamp_seconds: 0,
//         spawn_at_timestamp_seconds: None,
//         followees: HashMap::default(),
//         recent_ballots: vec![],
//         kyc_verified: false,
//         maturity_e8s_equivalent: 0,
//         staked_maturity_e8s_equivalent: None,
//         auto_stake_maturity: None,
//         not_for_profit: false,
//         joined_community_fund_timestamp_seconds: None,
//         known_neuron_data: None,
//         dissolve_state: None,
//     }
// }
