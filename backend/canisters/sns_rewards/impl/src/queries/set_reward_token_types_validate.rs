use crate::guards::caller_is_governance_principal;
use canister_tracing_macros::trace;
use ic_cdk::query;
pub use sns_rewards_api_canister::set_reward_token_types_validate::{
    Args as SetRewardTokenTypesValidateArgs, Response as SetRewardTokenTypesValidateResponse,
};
use types::{TokenInfo, TokenSymbol};

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn set_reward_token_types_validate(
    args: SetRewardTokenTypesValidateArgs,
) -> SetRewardTokenTypesValidateResponse {
    match validate_set_reward_token_types(&args.token_list) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }
    match serde_json::to_string_pretty(&args) {
        Ok(json) => Ok(json),
        Err(e) => Err(format!("invalid payload : {e:?}")),
    }
}

pub fn validate_set_reward_token_types(args: &[(TokenSymbol, TokenInfo)]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Should contain at least 1 token symbol and amount to update".to_string());
    }

    for (_token_symbol, token_info) in args {
        token_info.validate()?
    }
    Ok(())
}
