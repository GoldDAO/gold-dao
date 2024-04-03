use crate::{ guards::caller_is_governance_principal, state::mutate_state };
use candid::CandidType;
use canister_tracing_macros::trace;
use ic_cdk::{ query, update };
use serde::{ Deserialize, Serialize };
use types::{ TokenInfo, TokenSymbol };

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum SetRewardTokenTypesResponse {
    Success,
    InternalError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SetRewardTokenTypesRequest {
    token_list: Vec<(String, TokenInfo)>,
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
pub async fn set_reward_token_types(
    args: SetRewardTokenTypesRequest
) -> SetRewardTokenTypesResponse {
    match set_reward_token_types_impl(args.token_list) {
        Ok(_) => SetRewardTokenTypesResponse::Success,
        Err(err) => SetRewardTokenTypesResponse::InternalError(err),
    }
}

pub(crate) fn set_reward_token_types_impl(
    token_list: Vec<(String, TokenInfo)>
) -> Result<(), String> {
    mutate_state(
        |s| -> Result<(), String> {
            for (token_string, token_info) in token_list {
                if let Ok(valid_token) = TokenSymbol::parse(&token_string) {
                    s.data.tokens.insert(valid_token, token_info);
                } else {
                    return Err(format!("Invalid token string passed : {}", token_string));
                }
            }
            Ok(())
        }
    )
}

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn set_reward_token_types_validate(
    args: SetRewardTokenTypesRequest
) -> Result<String, String> {
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

    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}
