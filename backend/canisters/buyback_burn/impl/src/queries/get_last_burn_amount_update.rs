use crate::guards::caller_is_governance_principal;
use crate::state::read_state;
use buyback_burn_api::get_last_burn_amount_update::Response as GetLastBurnAmountUpdateResponse;
use ic_cdk_macros::query;

#[query(guard = "caller_is_governance_principal", hidden = true)]
fn get_last_burn_amount_update() -> GetLastBurnAmountUpdateResponse {
    read_state(|state| state.data.last_burn_amount_update)
}
