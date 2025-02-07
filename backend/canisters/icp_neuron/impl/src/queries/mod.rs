use ic_cdk::query;

mod http_request;
pub mod list_neurons;
mod transform_http_response;

pub use list_neurons::*;

#[query(hidden = true)]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../../../api/can.did").to_string()
}
