use ic_cdk_macros::query;

use crate::state::read_state;

pub use sns_rewards_api_canister::get_reserve_transfer_amounts::Response as GetReserveTransferAmountsResponse;

#[query(hidden = true)]
fn get_reserve_transfer_amounts() -> GetReserveTransferAmountsResponse {
    read_state(|state| { state.data.daily_reserve_transfer.clone() })
}
