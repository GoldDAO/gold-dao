use ic_cdk::query;

mod http_request;
pub mod list_ogy_neurons;

// NOTE: why do we need this?
#[query(hidden = true)]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../../../api/can.did").to_string()
}
