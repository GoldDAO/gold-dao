use lazy_static::lazy_static;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use types::CanisterWasm;

lazy_static! {
    pub static ref IC_ICRC1_LEDGER: CanisterWasm = get_canister_wasm("ic_icrc1_ledger");
    pub static ref SNS_GOVERNANCE: CanisterWasm = get_canister_wasm("sns_governance");
    pub static ref REWARDS: CanisterWasm = get_rewards_canister_wasm();
}

fn get_rewards_canister_wasm() -> Vec<u8> {
    read_file_from_local_bin(&format!("sns_rewards_canister.wasm.gz"))
}

fn get_canister_wasm(canister_name: &str) -> CanisterWasm {
    read_file_from_local_bin(&format!("{canister_name}_canister.wasm"))
}

fn read_file_from_local_bin(file_name: &str) -> Vec<u8> {
    let mut file_path = local_bin();
    file_path.push(file_name);

    let mut file = File::open(&file_path).unwrap_or_else(|_|
        panic!("Failed to open file: {}", file_path.to_str().unwrap())
    );
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("Failed to read file");
    bytes
}

pub fn local_bin() -> PathBuf {
    let mut file_path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("Failed to read CARGO_MANIFEST_DIR env variable")
    );
    file_path.push("wasms");
    file_path
}
