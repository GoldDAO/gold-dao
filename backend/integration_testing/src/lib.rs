use candid::{ CandidType, Deserialize, Principal };
use serde::Serialize;

#[derive(Deserialize, CandidType)]
pub struct Args {
    test_mode: bool,
    pocket_ic: bool,
}

mod sns_init_payload;
#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use ic_cdk::api::management_canister::main::CanisterId;
    use pocket_ic::{ PocketIc, PocketIcBuilder, WasmResult };
    use candid::{ decode_one, encode_one, Principal };

    use crate::{ sns_init_payload::get_sns_init_args, Args };

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

    fn get_rewards_canister_wasm() -> Vec<u8> {
        let wasm_path: OsString =
            "../canisters/sns_rewards/target/wasm32-unknown-unknown/release/sns_rewards.wasm".into();
        std::fs::read(wasm_path).unwrap()
    }

    fn get_governance_canister_wasm() -> Vec<u8> {
        let wasm_path: OsString = "./sns-governance-canister.wasm".into();
        std::fs::read(wasm_path).unwrap()
    }
    fn get_nns_wasm() -> Vec<u8> {
        let wasm_path: OsString = "./nns.wasm".into();
        std::fs::read(wasm_path).unwrap()
    }

    #[test]
    #[should_panic(expected = "is out of cycles")]
    fn test_sanity() {
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
        let pic = PocketIcBuilder::new().with_nns_subnet().with_sns_subnet().build();

        let master_principal = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        let sns_root_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
        let sns_ledger_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 3]);
        let sns_swap_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 4]);

        let nns_subnet = pic.topology().get_nns().unwrap();
        let sns_subnet = pic.topology().get_sns().unwrap();

        let sns_canister_id = pic.create_canister_on_subnet(None, None, sns_subnet);
        pic.add_cycles(sns_canister_id, INIT_CYCLES);

        let init_args = get_sns_init_args(
            sns_ledger_canister_id.clone(),
            sns_root_canister_id.clone(),
            sns_swap_canister_id.clone()
        );

        pic.install_canister(
            sns_canister_id,
            get_governance_canister_wasm(),
            encode_one(init_args.clone()).unwrap(),
            None
        );

        // install rewards canister
        let rewards_canister = pic.create_canister_on_subnet(None, None, sns_subnet);
        pic.add_cycles(rewards_canister, INIT_CYCLES);

        let wasm = get_rewards_canister_wasm();
        let init_args = Args { test_mode: true, pocket_ic: true };
        pic.install_canister(rewards_canister, wasm, encode_one(init_args).unwrap(), None);

        // test rewards canister
        let reply: usize = match query_call(&pic, rewards_canister, "get_all_neurons") {
            WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
            WasmResult::Reject(_) => {
                return;
            }
        };
        // there should be 0 neurons
        pic.advance_time(std::time::Duration::from_secs(20));
        assert_eq!(reply, 0);

        // add some neurons

        // advance time for maturity
    }

    // fn synchronise_neurons_happy_path_old() {
    //     // let config = SubnetConfigSet {
    //     //     ..Default::default()
    //     // };
    //     // let pic = PocketIc::from_config(config);
    //     // // goes on NNS
    //     // let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    //     // let actual_canister_id = pic.create_canister_with_id(None, None, canister_id).unwrap();

    //     // let sns_subnet_id = pic.topology().get_sns().unwrap();

    //     // // ----------- install fake governance canister ----------
    //     // let governance_canister = pic.create_canister_on_subnet(None, None, sns_subnet_id);
    //     // pic.add_cycles(governance_canister, INIT_CYCLES);

    //     // let wasm = get_governance_canister_wasm();

    //     // pic.install_canister(governance_canister, wasm, encode_one(()).unwrap(), None);

    //     // // install rewards canister
    //     // let rewards_canister = pic.create_canister_on_subnet(None, None, sns_subnet_id);
    //     // pic.add_cycles(rewards_canister, INIT_CYCLES);

    //     // let wasm = get_rewards_canister_wasm();
    //     // let init_args = Args { test_mode: true, pocket_ic: true };
    //     // pic.install_canister(rewards_canister, wasm, encode_one(init_args).unwrap(), None);

    //     // // test rewards canister
    //     // let reply: usize = match query_call(&pic, rewards_canister, "get_all_neurons") {
    //     //     WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
    //     //     WasmResult::Reject(_) => {
    //     //         return;
    //     //     }
    //     // };
    //     // // there should be 0 neurons
    //     // pic.advance_time(std::time::Duration::from_secs(20));
    //     // assert_eq!(reply, 0);

    //     // add some neurons

    //     // advance time for maturity
    // }
}
