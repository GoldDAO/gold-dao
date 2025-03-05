use crate::{guards::caller_is_authorized, state::mutate_state};
use candid::Principal;
use canister_tracing_macros::trace;
pub use gldt_swap_api_canister::set_buy_back_canister::{
    Args as SetBuyBackCanisterArgs, Response as SetBuyBackCanisterResponse,
};
use ic_cdk::update;
use icrc_ledger_types::icrc1::account::Account;

#[update(guard = "caller_is_authorized", hidden = true)]
#[trace]
pub async fn set_buy_back_canister(account: SetBuyBackCanisterArgs) -> SetBuyBackCanisterResponse {
    match validate_set_buy_back_canister_payload(&account) {
        Ok(_) => {
            mutate_state(|s| {
                s.data.buy_back_burn_canister = account;
            });
            SetBuyBackCanisterResponse::Success
        }
        Err(e) => {
            return SetBuyBackCanisterResponse::InternalError(e);
        }
    }
}

pub fn validate_set_buy_back_canister_payload(
    some_account: &Option<Account>,
) -> Result<(), String> {
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
