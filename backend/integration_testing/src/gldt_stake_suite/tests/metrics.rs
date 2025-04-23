use crate::gldt_stake_suite::setup::{default_test_setup, setup::GldtStakeTestEnv};
use candid::Principal;
use serde_bytes::ByteBuf;
use serde_json::{from_slice, Value};
use types::HttpRequest;

use crate::client::gldt_swap::http_request;

#[test]
pub fn gldt_stake_metrics_endpoint_is_valid_json() {
    let mut test_env = default_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic_borrowed = &pic.borrow();

    let res = http_request(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &(HttpRequest {
            method: "GET".to_string(),
            url: "/metrics".to_string(),
            headers: vec![],
            body: ByteBuf::new(),
        }),
    );
    assert_eq!(res.status_code, 200);

    match from_slice::<Value>(&res.body) {
        Ok(json_value) => println!("Valid JSON: {}", json_value),
        Err(e) => {
            panic!("JSON from metrics endpoint is not valid JSON {e:?}");
        }
    }
}
