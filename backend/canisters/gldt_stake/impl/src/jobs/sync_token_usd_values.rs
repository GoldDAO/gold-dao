use bity_ic_canister_time::{run_now_then_interval, HOUR_IN_MS};
use std::{collections::HashMap, time::Duration};
use tracing::{debug, error, info, trace};
use types::TokenSymbol;

use ic_cdk::{
    api::canister_self,
    management_canister::{
        http_request, HttpHeader, HttpMethod, HttpRequestArgs, HttpRequestResult, TransformArgs,
        TransformContext, TransformFunc,
    },
};

use crate::state::{mutate_state, read_state};

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(HOUR_IN_MS), sync_token_usd_values_job);
}

pub fn sync_token_usd_values_job() {
    ic_cdk::futures::spawn(sync_token_usd_values_impl())
}

async fn sync_token_usd_values_impl() {
    let _span = tracing::info_span!("SYNC_TOKEN_USD_VALUES").entered();

    info!("start");

    let mut tokens = read_state(|s| s.data.stake_system.reward_types.clone());
    trace!("loaded reward_types: {:?}", tokens);

    tokens.insert(TokenSymbol::GLDT);
    trace!(
        "added GLDT token to token set. Tokens to process: {:?}",
        tokens
    );

    let mut new_price_map: HashMap<TokenSymbol, f64> = HashMap::new();
    let current_price_map = read_state(|s| s.data.analytics_system.token_usd_values.clone());
    trace!("current token_usd_values: {:?}", current_price_map);

    for token_symbol in tokens {
        let ledger_id = token_symbol.get_token_info().ledger_id;
        info!(
            "fetching USD value for token: {:?}, ledger_id: {:?}",
            token_symbol, ledger_id
        );

        // Setup the URL and its query parameters
        let url = format!(
            "https://api.gldt.org/v1/tokens/{}/price/latest",
            token_symbol.symbol()
        );

        let request = HttpRequestArgs {
            url: url.to_string(),
            method: HttpMethod::GET,
            body: None,               //optional for request
            max_response_bytes: None, //optional for request
            transform: Some(TransformContext {
                function: TransformFunc::new(canister_self(), "transform".to_string()),
                context: vec![],
            }),
            headers: vec![],
        };

        //3. MAKE HTTPS REQUEST AND WAIT FOR RESPONSE

        //Note: in Rust, `http_request()` already sends the cycles needed
        //so no need for explicit Cycles.add() as in Motoko
        match http_request(&request).await {
            //4. DECODE AND RETURN THE RESPONSE

            //See: https://docs.rs/ic-cdk/latest/ic_cdk/management_canister/struct.HttpRequestResult.html
            Ok(response) => {
                let str_body = String::from_utf8(response.body)
                    .expect("Transformed response is not UTF-8 encoded.");
                match str_body.trim().parse::<f64>() {
                    Ok(price) => {
                        new_price_map.insert(token_symbol.clone(), price);
                        info!("Parsed price for {:?}: {}", token_symbol, price);
                    }
                    Err(e) => {
                        error!(
                            "Failed to parse price for {:?}: {} (body: {:?})",
                            token_symbol, e, str_body
                        );
                    }
                }
            }
            Err(e) => {
                error!(
                    "error fetching {:?} USD value: {:?}. Falling back to current or 0.",
                    token_symbol, e
                );
                let current_token_price = current_price_map.get(&token_symbol).unwrap_or(&0f64);
                new_price_map.insert(token_symbol, *current_token_price);
                debug!(
                    "fallback price used for {:?}: {}",
                    token_symbol, current_token_price
                );
            }
        }
    }

    ic_cdk::println!("new token_usd_values to be saved: {:?}", new_price_map);
    mutate_state(|s| s.data.analytics_system.set_token_usd_values(new_price_map));

    info!("finished");
}

// Strips all data that is not needed from the original response.
// Read more here https://internetcomputer.org/docs/references/ic-interface-spec#ic-http_request
#[ic_cdk::query(hidden = true)]
fn transform(raw: TransformArgs) -> HttpRequestResult {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];

    let mut res = HttpRequestResult {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
        ..Default::default()
    };

    if res.status == 200u8 {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::debug_print(format!(
            "Received an error from price source: err = {:?}",
            raw
        ));
    }
    res
}
