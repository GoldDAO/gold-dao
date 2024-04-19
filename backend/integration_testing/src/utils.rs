// will eventually hold common functions for integration testing
use candid::Principal;
use icrc_ledger_types::icrc1::account::Subaccount;
use pocket_ic::PocketIc;
use rand::{ RngCore, thread_rng };
use sns_rewards::state::RuntimeState;
use std::fs::{ File, OpenOptions };
use std::io::prelude::*;

pub fn random_principal() -> Principal {
    let mut bytes = [0u8; 29];
    thread_rng().fill_bytes(&mut bytes);
    Principal::from_slice(&bytes)
}

pub fn hex_to_subaccount(hx_str: &str) -> Subaccount {
    match hex::decode(hx_str) {
        Ok(bytes) => {
            let mut array: [u8; 32] = [0; 32];
            array.copy_from_slice(&bytes);
            return Subaccount::from(array);
        }
        Err(e) => {
            panic!("Error decoding hex string: {}", e);
        }
    };
}

pub fn decode_http_bytes(bytes: Vec<u8>) -> String {
    let decoded_string = String::from_utf8_lossy(&bytes);
    decoded_string.to_string()
}

pub fn tick_n_blocks(pic: &PocketIc, times: u32) {
    for i in 0..times {
        pic.tick();
    }
}

// fn deserialize_state(bytes: &[u8]) -> RuntimeState {
//     let state: RuntimeState = serde::Deserialize(bytes).unwrap();
//     state
// }

// pub fn save_stable_memory(data: Vec<u8>) {
//     // Your Vec<u8> data

//     // Your Vec<u8> data
//     let state = deserialize_state(data.as_slice());
//     println!("--------------------------");
//     println!("{:?}", state.env.is_test_mode());
//     println!("--------------------------");

//     // Create a new file or truncate if it already exists
//     let mut file = File::create("example.bin").unwrap();

//     file.write_all(&data).unwrap();
// }
