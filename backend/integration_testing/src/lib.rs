use candid::{ CandidType, Deserialize, Principal };
use serde::Serialize;
use sns_governance_canister::types::NeuronId;

#[derive(Deserialize, CandidType)]
pub struct Args {
    test_mode: bool,
    pocket_ic: bool,
}

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

mod sns_init_payload;
#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use canister_time::WEEK_IN_MS;
    use ic_cdk::api::management_canister::main::CanisterId;
    use pocket_ic::{ PocketIc, PocketIcBuilder, WasmResult };
    use candid::{ decode_one, encode_one, CandidType, Decode, Principal };
    use sns_governance_canister::types::{ GetNeuronResponse, Governance, NeuronId };

    use crate::{ sns_init_payload::create_weekly_sns_canister_data, Args, GetNeuronRequest };

    // 10T cycles
    const INIT_CYCLES: u128 = 10_000_000_000_000;

    fn update_call(ic: &PocketIc, can_id: CanisterId, method: &str) -> WasmResult {
        ic.update_call(can_id, Principal::anonymous(), method, encode_one(()).unwrap()).expect(
            "Failed to call counter canister"
        )
    }

    fn query_call(ic: &PocketIc, can_id: CanisterId, method: &str) -> WasmResult {
        ic.query_call(can_id, Principal::anonymous(), method, encode_one(()).unwrap()).expect(
            "Failed to query canister"
        )
    }

    fn query_call_with_arg<T: CandidType>(
        ic: &PocketIc,
        can_id: CanisterId,
        method: &str,
        arg: T
    ) -> WasmResult {
        ic.query_call(can_id, Principal::anonymous(), method, encode_one(arg).unwrap()).expect(
            "Failed to query canister"
        )
    }

    fn get_rewards_canister_wasm() -> Vec<u8> {
        let wasm_path: OsString =
            "../canisters/sns_rewards/target/wasm32-unknown-unknown/release/sns_rewards.wasm".into();
        std::fs::read(wasm_path).unwrap()
    }

    fn get_governance_canister_wasm() -> Vec<u8> {
        let wasm_path: OsString = "./sns-governance-canister.wasm".into();
        std::fs::read(wasm_path).unwrap()
    }

    #[test]
    #[should_panic(expected = "is out of cycles")]
    fn test_sanity_with_cycles() {
        let pic = PocketIc::new();
        let canister_id = pic.create_canister();
        let wasm = b"\x00\x61\x73\x6d\x01\x00\x00\x00".to_vec();
        pic.install_canister(canister_id, wasm.clone(), vec![], None);
    }

    // test neuron sync works - happy path
    #[test]
    fn calls_a_dummy_method() {
        let pic = PocketIc::new();

        // Create a canister and charge it with 2T cycles.
        let can_id = pic.create_canister();
        pic.add_cycles(can_id, INIT_CYCLES);

        // Install the counter canister wasm file on the canister.
        let wasm = get_rewards_canister_wasm();
        let init_args = Args { test_mode: true, pocket_ic: true };
        pic.install_canister(can_id, wasm, encode_one(init_args).unwrap(), None);

        // Make some calls to the canister.
        let reply: Result<bool, String> = match update_call(&pic, can_id, "read") {
            WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
            WasmResult::Reject(_) => {
                return;
            }
        };
        assert_eq!(reply.unwrap(), true);
    }

    #[test]
    fn synchronise_neurons_happy_path() {
        let pic = PocketIcBuilder::new().with_sns_subnet().build();

        let master_principal = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

        let sns_subnet = pic.topology().get_sns().unwrap();

        // ********************************
        // Add SNS Gov canister + first week of Neuron maturity
        // ********************************

        let sns_canister_id = pic.create_canister_on_subnet(None, None, sns_subnet);
        pic.add_cycles(sns_canister_id, INIT_CYCLES);

        let (week_1_init_args, users, neuron_data) = create_weekly_sns_canister_data(1);
        // users, neurons, init_args

        pic.install_canister(
            sns_canister_id,
            get_governance_canister_wasm(),
            encode_one(week_1_init_args.clone()).unwrap(),
            None
        );

        // ********************************
        // Add Rewards Canister
        // ********************************

        let rewards_canister = pic.create_canister_on_subnet(None, None, sns_subnet);

        pic.add_cycles(rewards_canister, INIT_CYCLES);

        let wasm = get_rewards_canister_wasm();
        let init_args = Args { test_mode: true, pocket_ic: true };
        pic.install_canister(rewards_canister, wasm, encode_one(init_args).unwrap(), None);

        // ********************************
        // Test synchronise_neurons
        // ********************************

        update_call(&pic, rewards_canister, "sync_neurons_manual_trigger");

        // pic.advance_time(std::time::Duration::from_secs(60 * 60 * 24 * 8));

        // test rewards canister
        let num_neurons: usize = match query_call(&pic, rewards_canister, "get_all_neurons") {
            WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
            WasmResult::Reject(_) => {
                return;
            }
        };
        assert_eq!(num_neurons, neuron_data.len());

        // ********************************
        // Reinstall SNS Gov canister + SECOND week of Neuron maturity
        // ********************************

        let (week_2_init_args, users, neuron_data) = create_weekly_sns_canister_data(1);

        pic.reinstall_canister(
            sns_canister_id,
            get_governance_canister_wasm(),
            encode_one(week_2_init_args).unwrap(),
            None
        ).unwrap();
    }

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
}
