use crate::gldt_swap_suite::{init, CanisterIds, PrincipalIds, TestEnv};
use crate::utils::tick_n_blocks;
use candid::Principal;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use canister_time::HOUR_IN_MS;
    use serde_bytes::ByteBuf;
    use serde_json::{from_slice, Value};
    use types::HttpRequest;

    use crate::client::gldt_swap::http_request;

    use super::*;
    #[test]
    pub fn metrics_endpoint_is_valid_json() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids: CanisterIds { gldt_swap, .. },
            principal_ids: PrincipalIds { .. },
        } = env;
        pic.advance_time(Duration::from_millis(HOUR_IN_MS));
        tick_n_blocks(pic, 5);

        let res = http_request(
            pic,
            Principal::anonymous(),
            gldt_swap,
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
}
