use bity_ic_canister_time::{run_now_then_interval, HOUR_IN_MS};
use ic_cdk::{
    api::canister_self,
    management_canister::{
        http_request, HttpHeader, HttpMethod, HttpRequestArgs, HttpRequestResult, TransformArgs,
        TransformContext, TransformFunc,
    },
};
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};
use tracing::{debug, error, info, trace};
use types::TokenSymbol;

#[derive(Debug, Deserialize)]
struct PriceEntry {
    #[serde(rename = "date")]
    _date: String,
    price: f64,
}

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
    ic_cdk::println!("start");

    let mut tokens = read_state(|s| s.data.stake_system.reward_types.clone());
    trace!("loaded reward_types: {:?}", tokens);
    ic_cdk::println!("loaded reward_types: {:?}", tokens);

    tokens.insert(TokenSymbol::GLDT);
    trace!(
        "added GLDT token to token set. Tokens to process: {:?}",
        tokens
    );
    ic_cdk::println!(
        "added GLDT token to token set. Tokens to process: {:?}",
        tokens
    );

    let mut new_price_map: HashMap<TokenSymbol, f64> = HashMap::new();
    let current_price_map = read_state(|s| s.data.analytics_system.token_usd_values.clone());
    trace!("current token_usd_values: {:?}", current_price_map);
    ic_cdk::println!("current token_usd_values: {:?}", current_price_map);

    for token_symbol in tokens {
        let ledger_id = token_symbol.get_prod_token_info().ledger_id;
        info!(
            "fetching USD history for token: {:?}, ledger_id: {:?}",
            token_symbol, ledger_id
        );
        ic_cdk::println!(
            "fetching USD history for token: {:?}, ledger_id: {:?}",
            token_symbol,
            ledger_id
        );

        let url = format!(
            "https://api.gldt.org/v1/tokens/{}/price/history",
            token_symbol.symbol()
        );

        let request = HttpRequestArgs {
            url: url.to_string(),
            method: HttpMethod::GET,
            body: None,
            max_response_bytes: Some(1024 * 32), // allow larger history payload
            transform: Some(TransformContext {
                function: TransformFunc::new(canister_self(), "transform".to_string()),
                context: vec![],
            }),
            headers: vec![],
        };

        match http_request(&request).await {
            Ok(response) => {
                let str_body = String::from_utf8(response.body)
                    .expect("Transformed response is not UTF-8 encoded.");

                match serde_json::from_str::<Vec<PriceEntry>>(&str_body) {
                    Ok(mut entries) if !entries.is_empty() => {
                        if entries.len() > 7 {
                            entries = entries.split_off(entries.len() - 7);
                        }

                        let prices: Vec<f64> = entries.iter().map(|e| e.price).collect();

                        if let Some(median_value) = median(prices) {
                            new_price_map.insert(token_symbol, median_value);
                            info!(
                                "Computed 7-day median price for {:?}: {}",
                                token_symbol, median_value
                            );
                            ic_cdk::println!(
                                "Computed 7-day median price for {:?}: {}",
                                token_symbol,
                                median_value
                            );
                        } else {
                            error!("Failed to compute median for {:?}", token_symbol);
                            ic_cdk::println!("Failed to compute median for {:?}", token_symbol);
                        }
                    }
                    Ok(_) => {
                        error!("No price entries found for {:?}", token_symbol);
                        ic_cdk::println!("No price entries found for {:?}", token_symbol);
                    }
                    Err(e) => {
                        error!(
                            "Failed to parse history for {:?}: {} (body: {:?})",
                            token_symbol, e, str_body
                        );
                        ic_cdk::println!(
                            "Failed to parse history for {:?}: {} (body: {:?})",
                            token_symbol,
                            e,
                            str_body
                        );
                    }
                }
            }
            Err(e) => {
                error!(
                    "error fetching {:?} USD history: {:?}. Falling back to current or 0.",
                    token_symbol, e
                );
                ic_cdk::println!(
                    "error fetching {:?} USD history: {:?}. Falling back to current or 0.",
                    token_symbol,
                    e
                );

                let current_token_price = current_price_map.get(&token_symbol).unwrap_or(&0f64);
                new_price_map.insert(token_symbol, *current_token_price);

                debug!(
                    "fallback price used for {:?}: {}",
                    token_symbol, current_token_price
                );
                ic_cdk::println!(
                    "fallback price used for {:?}: {}",
                    token_symbol,
                    current_token_price
                );
            }
        }
    }

    mutate_state(|s| s.data.analytics_system.set_token_usd_values(new_price_map));

    info!("finished");
    ic_cdk::println!("finished");
}

// --- Median helper ---
fn median(mut values: Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = values.len() / 2;
    if values.len() % 2 == 0 {
        Some((values[mid - 1] + values[mid]) / 2.0)
    } else {
        Some(values[mid])
    }
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
