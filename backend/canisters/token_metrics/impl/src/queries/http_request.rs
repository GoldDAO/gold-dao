use crate::state::{read_state, RuntimeState};
use http_request::{build_json_response, encode_logs, extract_route, Route};
use ic_cdk_macros::query;
use std::collections::HashMap;
use tracing::info;
use types::{HttpRequest, HttpResponse, TimestampMillis};

#[query(hidden = true)]
fn http_request(request: HttpRequest) -> HttpResponse {
    fn get_logs_impl(since: Option<TimestampMillis>) -> HttpResponse {
        encode_logs(canister_logger::export_logs(), since.unwrap_or(0))
    }

    fn get_traces_impl(since: Option<TimestampMillis>) -> HttpResponse {
        encode_logs(canister_logger::export_traces(), since.unwrap_or(0))
    }

    fn get_metrics_impl(state: &RuntimeState) -> HttpResponse {
        build_json_response(&state.metrics())
    }

    fn get_gold_nft_metrics() -> HttpResponse {
        let total_gold_grams = read_state(|s| s.data.total_gold_grams.clone());
        info!("total_gold_grams : {total_gold_grams:?}");
        let gold_price = read_state(|s| s.data.gold_price.clone());
        info!("gold_price : {gold_price:?}");

        let tvl = (total_gold_grams as f64) * gold_price;
        info!("tvl : {tvl:?}");

        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("total_gold_grams".to_string(), total_gold_grams.to_string());
        data.insert("gold_price".to_string(), gold_price.to_string());
        data.insert("tvl".to_string(), tvl.to_string());

        info!("get_gold_nft_metrics return : {data:?}");

        build_json_response(&data)
    }

    fn get_total_supply(state: &RuntimeState) -> HttpResponse {
        build_json_response(&state.data.supply_data.total_supply)
    }

    fn get_circulating_supply(state: &RuntimeState) -> HttpResponse {
        build_json_response(&state.data.supply_data.circulating_supply)
    }

    match extract_route(&request.url) {
        Route::Logs(since) => get_logs_impl(since),
        Route::Traces(since) => get_traces_impl(since),
        Route::RawTotalSupply => read_state(get_total_supply),
        Route::RawCirculatingSupply => read_state(get_circulating_supply),
        Route::Metrics => read_state(get_metrics_impl),
        Route::Other(path, _) if path == "gold_nft_metrics" => get_gold_nft_metrics(),
        _ => HttpResponse::not_found(),
    }
}
