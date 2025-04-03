use crate::guards::caller_is_governance_principal;
use crate::state::mutate_state;
use crate::state::read_state;
pub use gldt_stake_api_canister::_set_token_usd_values::{
    Args as SetTokenUsdValuesArgs, Response as SetTokenUsdValuesResponse,
};
use ic_cdk::update;

#[update(guard = "caller_is_governance_principal")]
#[cfg(feature = "inttest")]
fn _set_token_usd_values(args: SetTokenUsdValuesArgs) -> SetTokenUsdValuesResponse {
    _set_token_usd_values_impl(args);
}

fn _set_token_usd_values_impl(args: SetTokenUsdValuesArgs) -> SetTokenUsdValuesResponse {
    mutate_state(|s| {
        s.data.stake_system.set_token_usd_values(args);
    });

    ()
}
