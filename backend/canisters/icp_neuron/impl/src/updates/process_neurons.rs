use ic_cdk::update;

use crate::guards::is_test_mode;

#[update(hidden = true, guard = "is_test_mode")]
pub fn process_neurons_manual() {
    crate::jobs::process_neurons::run()
}

// #[cfg(test)]
// mod tests {
//     use candid::Principal;
//     use icrc_ledger_types::icrc1::account::Account;
//     use types::{ RewardsRecipient, RewardsRecipientList };
//     use utils::env::CanisterEnv;

//     use crate::{
//         state::{ init_state, Data, RuntimeState },
//         updates::process_neurons::process_neurons_manual,
//     };

//     #[test]
//     fn process_neurons_not_test() {
//         init_runtime_state(false);

//         assert_eq!(process_neurons_manual(), ())
//     }

//     fn init_runtime_state(test_mode: bool) {
//         init_state(RuntimeState {
//             env: CanisterEnv::new(test_mode),
//             data: Data::new(RewardsRecipientList::new(vec![dummy_recipient(10000)]).unwrap()),
//         });
//     }

//     fn dummy_account() -> Account {
//         Account {
//             owner: Principal::from_text(
//                 "thrhh-hnmzu-kjquw-6ebmf-vdhed-yf2ry-avwy7-2jrrm-byg34-zoqaz-wqe"
//             ).unwrap(),
//             subaccount: None,
//         }
//     }
//     fn dummy_recipient(reward_weight: u16) -> RewardsRecipient {
//         RewardsRecipient {
//             account: dummy_account(),
//             tag: "test".to_string(),
//             reward_weight,
//         }
//     }
// }
