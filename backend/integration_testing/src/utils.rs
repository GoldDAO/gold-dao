// will eventually hold common functions for integration testing
use candid::Principal;
use icrc_ledger_types::icrc1::account::Subaccount;
use rand::{ RngCore, thread_rng };

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
