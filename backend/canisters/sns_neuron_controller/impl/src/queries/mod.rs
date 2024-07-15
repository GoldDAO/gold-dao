use ic_cdk::query;

mod current_available_rewards;
mod http_request;
pub mod list_ogy_neurons;

// NOTE: why do we need this?
/// This makes this Candid service self-describing, so that for example Candid UI, but also other
/// tools, can seamlessly integrate with it. The concrete interface (method name etc.) is
/// provisional, but works.
#[query(hidden = true)]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../../../api/can.did").to_string()
}
