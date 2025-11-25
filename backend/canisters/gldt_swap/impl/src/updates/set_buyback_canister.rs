use crate::{guards::caller_is_authorized, state::mutate_state};
use bity_ic_canister_tracing_macros::trace;
use candid::Principal;
pub use gldt_swap_api_canister::set_buyback_canister::{
    Args as SetBuyBackCanisterArgs, Response as SetBuyBackCanisterResponse,
};
use ic_cdk::{query, update};
use icrc_ledger_types::icrc1::account::Account;

#[query(guard = "caller_is_authorized", hidden = true)]
#[trace]
async fn set_buyback_canister_validate(args: SetBuyBackCanisterArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_authorized")]
#[trace]
pub async fn set_buyback_canister(account: SetBuyBackCanisterArgs) -> SetBuyBackCanisterResponse {
    match validate_set_buyback_canister_payload(&account) {
        Ok(_) => {
            mutate_state(|s| {
                s.data.buyback_burn_canister = account;
            });
            SetBuyBackCanisterResponse::Success
        }
        Err(e) => SetBuyBackCanisterResponse::InternalError(e),
    }
}

pub fn validate_set_buyback_canister_payload(some_account: &Option<Account>) -> Result<(), String> {
    match some_account {
        Some(account) => {
            if account.owner == Principal::anonymous() {
                return Err(format!("ERROR : account.owner can't be annoymous"));
            }
            Ok(())
        }
        None => Ok(()),
    }
}
