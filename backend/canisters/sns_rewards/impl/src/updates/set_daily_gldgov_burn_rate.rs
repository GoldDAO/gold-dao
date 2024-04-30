use crate::{
    guards::caller_is_governance_principal,
    state::mutate_state,
    utils::validate_set_daily_gldgov_burn_rate_payload,
};
use candid::Nat;
use canister_tracing_macros::trace;
use ic_cdk::update;
pub use sns_rewards_api_canister::{
    set_daily_gldgov_burn_rate::{
        Args as SetDailyGLDGovBurnRateArgs,
        Response as SetDailyGLDGovBurnRateResponse,
    },
    ReserveTokenAmounts,
};

#[update(guard = "caller_is_governance_principal")]
#[trace]
pub async fn set_daily_gldgov_burn_rate(
    amount: SetDailyGLDGovBurnRateArgs
) -> SetDailyGLDGovBurnRateResponse {
    set_daily_gldgov_burn_rate_impl(amount)
}

// this will overwrite the hashmap completely so any tokens not passed in will be removed.
pub(crate) fn set_daily_gldgov_burn_rate_impl(amount: Nat) -> SetDailyGLDGovBurnRateResponse {
    match validate_set_daily_gldgov_burn_rate_payload(&amount) {
        Ok(_) => {}
        Err(e) => {
            return SetDailyGLDGovBurnRateResponse::InternalError(e);
        }
    }
    mutate_state(|s| {
        s.data.daily_gldgov_burn_rate = Some(amount);
    });
    SetDailyGLDGovBurnRateResponse::Success
}
