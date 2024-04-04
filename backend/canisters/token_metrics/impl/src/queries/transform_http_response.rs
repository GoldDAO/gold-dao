use ic_cdk::api::management_canister::http_request::{ HttpResponse, TransformArgs };
use ic_cdk::query;

#[query(hidden = true)]
fn transform_http_response(args: TransformArgs) -> HttpResponse {
    let mut response = args.response;
    response.headers.clear();
    response
}
