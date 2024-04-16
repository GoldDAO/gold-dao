use candid::{ CandidType, Deserialize, Principal };
use serde::Serialize;
use sns_governance_canister::types::NeuronId;

use crate::{ client::rewards::get_all_neurons, setup::setup::{ init, TestEnv } };

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn synchronise_neurons_happy_path() {
    let env = init();
    let TestEnv { pic, controller, token_ledgers, sns, rewards } = env;

    let res = get_all_neurons(&pic, Principal::anonymous(), rewards, &());

    assert_eq!(res as usize, sns.neuron_test_data.len())
    // synchronise should have 10 neurons
    // let num_neurons: usize = match query_call(&pic, rewards_canister, "get_all_neurons") {
    //     WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
    //     WasmResult::Reject(_) => {
    //         return;
    //     }
    // };
}

// 10T cycles

// #[test]
// #[should_panic(expected = "is out of cycles")]
// fn test_sanity_with_cycles() {
//     let pic = PocketIc::new();
//     let canister_id = pic.create_canister();
//     let wasm = b"\x00\x61\x73\x6d\x01\x00\x00\x00".to_vec();
//     pic.install_canister(canister_id, wasm.clone(), vec![], None);
// }

// ********************************
// Test synchronise_neurons
// ********************************

// update_call(&pic, rewards_canister, "sync_neurons_manual_trigger");

// // pic.advance_time(std::time::Duration::from_secs(60 * 60 * 24 * 8));

// // test rewards canister
// let num_neurons: usize = match query_call(&pic, rewards_canister, "get_all_neurons") {
//     WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
//     WasmResult::Reject(_) => {
//         return;
//     }
// };
// assert_eq!(num_neurons, neuron_data.len());

// // ********************************
// // Reinstall SNS Gov canister + SECOND week of Neuron maturity
// // ********************************

// let (week_2_init_args, users, neuron_data) = create_weekly_sns_canister_data(1);

// pic.reinstall_canister(
//     sns_canister_id,
//     get_governance_canister_wasm(),
//     encode_one(week_2_init_args).unwrap(),
//     None
// ).unwrap();
// }

//  // sanity
//     let reply: GetNeuronResponse = match
//         query_call_with_arg(&pic, sns_canister_id, "get_neuron", GetNeuronRequest {
//             neuron_id: NeuronId::new(
//                 "146ed81314556807536d74005f4121b8769bba1992fce6b90c2949e855d04208"
//             ).unwrap(),
//         })
//     {
//         WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
//         WasmResult::Reject(_) => {
//             return;
//         }
//     };

//     assert_eq!(reply.result.is_some(), true);
//     let n = reply.result.unwrap();

//     let nn = if let sns_governance_canister::types::get_neuron_response::Result::Neuron(n) = n {
//         Some(n) // Assign Some(n) to nn when Result::Neuron(n) matches
//     } else {
//         None // Assign None to nn for other cases (like Result::Error(e))
//     };

//     println!("{:?}", nn.unwrap().maturity_e8s_equivalent);
//     // end sanity

// sanity
// let reply: GetNeuronResponse = match
//     query_call_with_arg(&pic, sns_canister_id, "get_neuron", GetNeuronRequest {
//         neuron_id: NeuronId::new(
//             "146ed81314556807536d74005f4121b8769bba1992fce6b90c2949e855d04208"
//         ).unwrap(),
//     })
// {
//     WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
//     WasmResult::Reject(_) => {
//         return;
//     }
// };

// assert_eq!(reply.result.is_some(), true);
// let n = reply.result.unwrap();

// let nn = if let sns_governance_canister::types::get_neuron_response::Result::Neuron(n) = n {
//     Some(n) // Assign Some(n) to nn when Result::Neuron(n) matches
// } else {
//     None // Assign None to nn for other cases (like Result::Error(e))
// };

// nn.unwrap()
//     .permissions.iter()
//     .for_each(|n_pem| {
//         println!("{:?}", n_pem.principal);
//     });
// end sanity
