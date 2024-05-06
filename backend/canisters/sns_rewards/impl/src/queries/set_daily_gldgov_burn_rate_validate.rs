use crate::{
    guards::caller_is_governance_principal,
    utils::validate_set_daily_gldgov_burn_rate_payload,
};
use canister_tracing_macros::trace;
use ic_cdk::query;

pub use sns_rewards_api_canister::set_daily_gldgov_burn_rate_validate::{
    Args as SetDailyGLDGovBurnRateValidateArgs,
    Response as SetDailyGLDGovBurnRateValidateResponse,
};

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
pub async fn set_daily_gldgov_burn_rate_validate(
    amount: SetDailyGLDGovBurnRateValidateArgs
) -> SetDailyGLDGovBurnRateValidateResponse {
    match validate_set_daily_gldgov_burn_rate_payload(&amount) {
        Ok(_) => {}
        Err(e) => {
            return SetDailyGLDGovBurnRateValidateResponse::Err(e);
        }
    }
    match serde_json::to_string_pretty(&amount) {
        Ok(json) => SetDailyGLDGovBurnRateValidateResponse::Ok(json),
        Err(e) => SetDailyGLDGovBurnRateValidateResponse::Err(format!("invalid payload : {e:?}")),
    }
}
