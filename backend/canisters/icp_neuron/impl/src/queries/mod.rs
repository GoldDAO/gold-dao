use ic_cdk::query;

mod http_request;
mod transform_http_response;

#[query(hidden = true)]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../../../api/can.did").to_string()
}
