use candid::{ CandidType, Deserialize };

#[derive(Deserialize, CandidType)]
pub struct Args {
    test_mode: bool,
}

#[cfg(test)]
mod tests {
    use std::{ ffi::OsString, io::Read };

    use ic_cdk::api::management_canister::main::CanisterId;
    use pocket_ic::{ PocketIc, WasmResult };
    use candid::{ decode_one, encode_one, Principal };

    use crate::Args;

    // 10T cycles
    const INIT_CYCLES: u128 = 10_000_000_000_000;

    fn call_counter_can(ic: &PocketIc, can_id: CanisterId, method: &str) -> WasmResult {
        ic.update_call(can_id, Principal::anonymous(), method, encode_one(()).unwrap()).expect(
            "Failed to call counter canister"
        )
    }

    fn sns_rewards_wasm() -> Vec<u8> {
        let wasm_path: OsString =
            "../canisters/sns_rewards/target/wasm32-unknown-unknown/release/sns_rewards.wasm".into();
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
    fn neuron_synchronisation_happy_path() {
        let pic = PocketIc::new();

        // Create a canister and charge it with 2T cycles.
        let can_id = pic.create_canister();
        pic.add_cycles(can_id, INIT_CYCLES);

        // Install the counter canister wasm file on the canister.
        let wasm = sns_rewards_wasm();
        let init_args = Args { test_mode: true };
        pic.install_canister(can_id, wasm, encode_one(init_args).unwrap(), None);

        // Make some calls to the canister.
        let reply: Result<bool, String> = match call_counter_can(&pic, can_id, "read") {
            WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
            WasmResult::Reject(_) => {
                return;
            }
        };
        assert_eq!(reply.unwrap(), true);
    }
}
