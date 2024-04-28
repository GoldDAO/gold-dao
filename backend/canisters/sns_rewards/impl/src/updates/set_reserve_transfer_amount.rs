use std::collections::HashMap;

use crate::{
    guards::caller_is_governance_principal,
    state::mutate_state,
    utils::validate_set_reserve_transfer_amounts_payload,
};
use candid::Nat;
use canister_tracing_macros::trace;
use ic_cdk::{ query, update };
use sns_rewards_api_canister::set_reserve_transfer_amounts::{ Args, Response };
use types::TokenSymbol;

#[trace]
#[update(guard = "caller_is_governance_principal")]
pub async fn set_reserve_transfer_amounts(args: Args) -> Response {
    set_reserve_transfer_amounts_impl(args.transfer_amounts)
}

// this will overwrite the hashmap completely so any tokens not passed in will be removed.
pub(crate) fn set_reserve_transfer_amounts_impl(
    transfer_amounts: HashMap<TokenSymbol, Nat>
) -> Response {
    match validate_set_reserve_transfer_amounts_payload(&transfer_amounts) {
        Ok(_) => {}
        Err(e) => {
            return Response::InternalError(e);
        }
    }
    mutate_state(|s| {
        s.data.daily_reserve_transfer = transfer_amounts;
    });
    Response::Success
}
