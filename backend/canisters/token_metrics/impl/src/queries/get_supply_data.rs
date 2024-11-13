use crate::state::read_state;
use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_supply_data::Response as GetSupplyDataResponse;

#[query]
fn get_supply_data() -> GetSupplyDataResponse {
    read_state(|state| state.data.supply_data.clone())
}
