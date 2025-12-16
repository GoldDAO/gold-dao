use crate::guards::caller_is_governance_principal;
use bity_ic_canister_tracing_macros::trace;
use ic_cdk::query;
pub use sns_rewards_api_canister::burn_goldao_reserve_pool_validate::{
    Args as BurnGoldaoReservePoolValidateArgs, Response as BurnGoldaoReservePoolValidateResponse,
};

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn burn_goldao_reserve_pool_validate(
    _args: BurnGoldaoReservePoolValidateArgs,
) -> BurnGoldaoReservePoolValidateResponse {
    Ok("No arguments to validate".to_string())
}
