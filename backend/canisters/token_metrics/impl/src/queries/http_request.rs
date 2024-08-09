use http_request::{ build_json_response, encode_logs, extract_route, Route };
use ic_cdk_macros::query;
use types::{ HttpRequest, HttpResponse, TimestampMillis };

use crate::state::{ read_state, RuntimeState };

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

    fn get_total_supply(state: &RuntimeState) -> HttpResponse {
        let result = state.data.supply_data.total_supply.clone();
        let result_u64: u64 = result.0.try_into().unwrap_or(0);
        let result_float: f64 = (result_u64 as f64) / 1e8;
        build_json_response(&result_float)
    }

    fn get_circulating_supply(state: &RuntimeState) -> HttpResponse {
        let result = state.data.supply_data.circulating_supply.clone();
        let result_u64: u64 = result.0.try_into().unwrap_or(0);
        let result_float: f64 = (result_u64 as f64) / 1e8;
        build_json_response(&result_float)
    }

    match extract_route(&request.url) {
        Route::Logs(since) => get_logs_impl(since),
        Route::Traces(since) => get_traces_impl(since),
        Route::Metrics => read_state(get_metrics_impl),
        Route::Other(path, _) => {
            if path == "total-supply" {
                return read_state(get_total_supply);
            }
            if path == "circulating-supply" {
                return read_state(get_circulating_supply);
            } else {
                HttpResponse::not_found()
            }
        }
        _ => HttpResponse::not_found(),
    }
}
