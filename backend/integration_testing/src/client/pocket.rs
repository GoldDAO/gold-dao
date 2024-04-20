use candid::{ CandidType, Principal };
use pocket_ic::{ PocketIc, UserError, WasmResult };
use serde::de::DeserializeOwned;
use types::CanisterId;

pub fn execute_query<P: CandidType, R: CandidType + DeserializeOwned>(
    pic: &PocketIc,
    sender: Principal,
    canister_id: CanisterId,
    method_name: &str,
    payload: &P
) -> R {
    unwrap_response(
        pic.query_call(canister_id, sender, method_name, candid::encode_one(payload).unwrap())
    )
}

pub fn execute_update<P: CandidType, R: CandidType + DeserializeOwned>(
    pic: &PocketIc,
    sender: Principal,
    canister_id: CanisterId,
    method_name: &str,
    payload: &P
) -> R {
    unwrap_response(
        pic.update_call(canister_id, sender, method_name, candid::encode_one(payload).unwrap())
    )
}

pub fn execute_update_multi_args<
    P: CandidType + candid::utils::ArgumentEncoder,
    R: CandidType + DeserializeOwned
>(pic: &PocketIc, sender: Principal, canister_id: CanisterId, method_name: &str, payload: P) -> R {
    unwrap_response(
        pic.update_call(canister_id, sender, method_name, candid::encode_args(payload).unwrap())
    )
}

fn unwrap_response<R: CandidType + DeserializeOwned>(response: Result<WasmResult, UserError>) -> R {
    match response.unwrap() {
        WasmResult::Reply(bytes) => candid::decode_one(&bytes).unwrap(),
        WasmResult::Reject(error) => panic!("FATAL ERROR: {error}"),
    }
}
