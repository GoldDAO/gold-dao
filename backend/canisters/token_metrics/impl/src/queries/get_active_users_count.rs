use crate::state::read_state;
use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_active_users_count::Response as GetActiveUsersCountResponse;

#[query]
fn get_active_users_count() -> GetActiveUsersCountResponse {
    read_state(|state| state.data.active_users.clone())
}
