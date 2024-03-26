use crate::state::{ read_state };
use http_request::{ build_json_response, extract_route, Route };
use ic_cdk_macros::query;
use types::{ HttpRequest, HttpResponse };
use std::collections::HashMap;

#[query(hidden = true)]
fn http_request(request: HttpRequest) -> HttpResponse {
    fn get_gold_nft_metrics() -> HttpResponse {
        let total_gold_grams = read_state(|s| s.data.total_gold_grams.clone());
        let gold_price = read_state(|s| s.data.gold_price.clone());

        let tvl = (total_gold_grams as f64) * gold_price;

        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("total_gold_grams".to_string(), total_gold_grams.to_string());
        data.insert("gold_price".to_string(), gold_price.to_string());
        data.insert("tvl".to_string(), tvl.to_string());

        build_json_response(&data)
    }

    match extract_route(&request.url) {
        Route::Other(path, _) if path == "gold_nft_metrics" => get_gold_nft_metrics(),
        _ => HttpResponse::not_found(),
    }
}
