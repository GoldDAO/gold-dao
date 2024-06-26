use crate::guards::caller_is_governance_principal;
use canister_tracing_macros::trace;
use ic_cdk::query;
pub use sns_rewards_api_canister::set_reward_token_types_validate::{
    Args as SetRewardTokenTypesValidateArgs,
    Response as SetRewardTokenTypesValidateResponse,
};
use types::TokenSymbol;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn set_reward_token_types_validate(
    args: SetRewardTokenTypesValidateArgs
) -> SetRewardTokenTypesValidateResponse {
    for (token_string, _) in &args.token_list {
        // Check token is in approved list and or return early if fail
        let parsed_token_result = TokenSymbol::parse(token_string);
        match parsed_token_result {
            Ok(_) => {}
            Err(e) => {
                let err_message = format!("Error parsing token {token_string}. error : {:?}", e);
                return Err(err_message);
            }
        }
    }

    match serde_json::to_string_pretty(&args) {
        Ok(json) => Ok(json),
        Err(e) => Err(format!("invalid payload : {e:?}")),
    }
}
