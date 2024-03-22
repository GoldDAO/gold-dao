use canister_time::{ run_now_then_interval, MINUTE_IN_MS };
use std::time::Duration;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};
use types::Milliseconds;
use crate::types::gold_price_types::DataPoint;
use crate::state::mutate_state;

const REFRESH_GOLD_PRICE_INTERVAL: Milliseconds = 10 * MINUTE_IN_MS;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(REFRESH_GOLD_PRICE_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

async fn run_async() {
    const URL : &str = "https://forex-data-feed.swissquote.com/public-quotes/bboquotes/instrument/XAU/USD";

    let request_headers = vec![];

    let request = CanisterHttpRequestArgument {
        url: URL.to_string(),
        method: HttpMethod::GET,
        body: None,               
        max_response_bytes: None, 
        transform: None,
        headers: request_headers,
    };

    let cycles : u128 = 230_949_972_000;

    match http_request(request, cycles).await {
        Ok((response,)) => {
            // TODO handle expect error
            let message = String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");

            match serde_json::from_str::<Vec<DataPoint>>(message.as_str()) {
                Ok(v) => {
                    if v.len() > 0 {
                        let spread_profile_prices = v[0].spreadProfilePrices.clone();
                        if spread_profile_prices.len() > 0 {
                            mutate_state(|state| {
                                // convert price in Once to price in Gram
                                state.data.gold_price = spread_profile_prices[0].ask / 31.1;
                            });
                        }
                    }
                }
                Err(err) => {            
                    let message: String = format!("The http_request resulted into error : {err:?}");
                    ic_cdk::api::print(message);
                    return ();
                }
            };
        }
        Err((r, m)) => {
            let message = format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
            ic_cdk::api::print(message);
            return ();
        }
    };
}
