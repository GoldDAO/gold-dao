use crate::{
    guards::caller_is_governance_principal, state::mutate_state,
    utils::validate_set_daily_goldao_burn_rate_payload,
};
use candid::Nat;
use canister_tracing_macros::trace;
use ic_cdk::update;
pub use sns_rewards_api_canister::{
    set_daily_goldao_burn_rate::{
        Args as SetDailyGOLDAOBurnRateArgs, Response as SetDailyGOLDAOBurnRateResponse,
    },
    ReserveTokenAmounts,
};

#[update(guard = "caller_is_governance_principal")]
#[trace]
pub async fn set_daily_goldao_burn_rate(
    amount: SetDailyGOLDAOBurnRateArgs,
) -> SetDailyGOLDAOBurnRateResponse {
    set_daily_goldao_burn_rate_impl(amount)
}

// this will overwrite the hashmap completely so any tokens not passed in will be removed.
pub(crate) fn set_daily_goldao_burn_rate_impl(amount: Nat) -> SetDailyGOLDAOBurnRateResponse {
    match validate_set_daily_goldao_burn_rate_payload(&amount) {
        Ok(_) => {}
        Err(e) => {
            return SetDailyGOLDAOBurnRateResponse::InternalError(e);
        }
    }
    mutate_state(|s| {
        s.data.daily_goldao_burn_rate = Some(amount);
    });
    SetDailyGOLDAOBurnRateResponse::Success
}
