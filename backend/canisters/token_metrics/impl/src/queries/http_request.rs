use crate::state::read_state;
use http_request::{ build_json_response, extract_route, Route, encode_logs };
use ic_cdk_macros::query;
use types::{ HttpRequest, HttpResponse, TimestampMillis };
use std::collections::HashMap;
use tracing::info;

#[query(hidden = true)]
fn http_request(request: HttpRequest) -> HttpResponse {
    fn get_logs_impl(since: Option<TimestampMillis>) -> HttpResponse {
        encode_logs(canister_logger::export_logs(), since.unwrap_or(0))
    }

    fn get_traces_impl(since: Option<TimestampMillis>) -> HttpResponse {
        encode_logs(canister_logger::export_traces(), since.unwrap_or(0))
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

    match extract_route(&request.url) {
        Route::Logs(since) => get_logs_impl(since),
        Route::Traces(since) => get_traces_impl(since),
        Route::Other(path, _) if path == "gold_nft_metrics" => get_gold_nft_metrics(),
        _ => HttpResponse::not_found(),
    }
}
