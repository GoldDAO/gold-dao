use crate::{ guards::caller_is_governance_principal, state::mutate_state };
use candid::CandidType;
use canister_tracing_macros::trace;
use ic_cdk::{ update };
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

// #[update(guard = "caller_is_governance_principal")]
#[update()]
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
