use crate::state::read_state;
pub use gldt_swap_index_api_canister::status::Response as StatusResponse;
use ic_cdk::query;

#[query]
pub fn status() -> StatusResponse {
    let last_block_id = read_state(|state| state.data.last_block_id);

    StatusResponse { last_block_id }
}
