use canister_time::{ run_now_then_interval, MINUTE_IN_MS };
use std::time::Duration;
use ic_cdk::api::management_canister::http_request::{
    http_request,
    CanisterHttpRequestArgument,
    HttpMethod,
    TransformContext,
};
use types::Milliseconds;
use crate::types::gold_price_types::YumiApiResponse;
use crate::state::mutate_state;
use tracing::{ info, error };
use ic_cdk::api::time;
use time::OffsetDateTime;

const REFRESH_GOLD_PRICE_INTERVAL: Milliseconds = 10 * MINUTE_IN_MS;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(REFRESH_GOLD_PRICE_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

fn timestamp_to_date_string(nanoseconds: u64) -> (String, String) {
    info!("get timestamp_to_date_string.");
    let datetime = OffsetDateTime::from_unix_timestamp_nanos(nanoseconds as i128).unwrap_or_else(|_|
        panic!("Invalid timestamp")
    );

    let year: i32 = datetime.year();
    let month: u8 = datetime.month().into();
    let day: u8 = datetime.day();

    (
        format!("{:04}-{:02}-{:02}", year - 1, month, day),
        format!("{:04}-{:02}-{:02}", year, month, day),
    )
}

async fn run_async() {
    info!("Run gold price update.");

    let (last_year, today) = timestamp_to_date_string(time());

    let url: &str = &format!(
        "https://api2.yumi.io/gold/tradePrice?symbols=XAU&start_at={last_year}&end_at={today}"
    ).to_string();

    let request_headers = vec![];

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: Some(
            TransformContext::from_name("transform_http_response".to_string(), Vec::new())
        ),
        headers: request_headers,
    };

    let cycles: u128 = 230_949_972_000;

    match http_request(request, cycles).await {
        Ok((response,)) => {
            let message = match String::from_utf8(response.body) {
                Ok(val) => { val }
                Err(e) => {
                    error!("Transformed response is not UTF-8 encoded : {e:?}");
                    return ();
                }
            };

            match serde_json::from_str::<YumiApiResponse>(message.as_str()) {
                Ok(v) => {
                    match v.data.last() {
                        Some(val) => {
                            let xau_price: f64 = (1 as f64) / val.price;

                            mutate_state(|state| {
                                // convert price in Once to price in Gram
                                state.data.gold_price = xau_price / 31.1;
                            });
                        }
                        _ => {
                            error!("Error while getting last elem of returned array.");
                            return ();
                        }
                    };
                }
                Err(err) => {
                    error!("The http_request resulted into error : {err:?}");
                    return ();
                }
            };
        }
        Err((r, m)) => {
            error!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
            return ();
        }
    }
    info!("Finished gold price update.");
}
