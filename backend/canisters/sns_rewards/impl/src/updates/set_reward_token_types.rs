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
async fn set_reward_token_types_validation(
    args: SetRewardTokenTypesRequest
) -> Result<String, String> {
    for (token_string, token_info) in &args.token_list {
        // Check token is in approved list and or return early if fail
        let parsed_token_result = TokenSymbol::parse(&token_string);
        match parsed_token_result {
            Ok(_) => {}
            Err(e) => {
                let err_message = format!("Error parsing token {token_string}. error : {:?}", e);
                return Err(err_message);
            }
        }

        // Not sure we if need this check since a valid principal is required in the args
        // if token_info.ledger_id {
        //     return Err(format!("ledger field may not be empty for token {}", token_string));
        // }

        if token_info.decimals <= 0 {
            return Err(format!("decimals for token {} may not be negative or 0", token_string));
        }

        if token_info.fee <= 0 {
            return Err(format!("fee for token {} may not be negative or 0", token_string));
        }

        // TODO - more verification ideas
        // we can verify the ledger is working and the symbols match by calling the ledger method - `icrc1_symbol`
        // we can also verify the fee and decimals match
    }

    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}
