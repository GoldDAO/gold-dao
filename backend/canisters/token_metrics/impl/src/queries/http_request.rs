use crate::state::{ read_state, RuntimeState };
use http_request::{ build_json_response, encode_logs, extract_route, Route };
use ic_cdk_macros::query;
use types::{ HttpRequest, HttpResponse, TimestampMillis };

#[query(hidden = true)]
fn http_request(request: HttpRequest) -> HttpResponse {
    fn get_gold_tvl() -> HttpResponse {
        let total_gold_grams = read_state(|s| s.data.total_gold_grams.clone());
        let gold_price = read_state(|s| s.data.gold_price.clone());

        let tvl = total_gold_grams as f64 * gold_price;

        build_json_response(&tvl)
    }

    match extract_route(&request.url) {
        Route::Other(path, _) if path == "tvl" => get_gold_tvl(),
        _ => HttpResponse::not_found(),
    }
}
