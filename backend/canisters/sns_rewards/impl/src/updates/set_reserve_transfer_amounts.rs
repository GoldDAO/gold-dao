use crate::{
    guards::caller_is_governance_principal,
    state::mutate_state,
    utils::validate_set_reserve_transfer_amounts_payload,
};
use canister_tracing_macros::trace;
use ic_cdk::update;
pub use sns_rewards_api_canister::{
    set_reserve_transfer_amounts::{
        Args as SetReserveTransferAmountsArgs,
        Response as SetReserveTransferAmountsResponse,
    },
    ReserveTokenAmounts,
};

#[update(guard = "caller_is_governance_principal")]
#[trace]
pub async fn set_reserve_transfer_amounts(
    args: SetReserveTransferAmountsArgs
) -> SetReserveTransferAmountsResponse {
    set_reserve_transfer_amounts_impl(args.transfer_amounts)
}

// this will overwrite the hashmap completely so any tokens not passed in will be removed.
pub(crate) fn set_reserve_transfer_amounts_impl(
    transfer_amounts: ReserveTokenAmounts
) -> SetReserveTransferAmountsResponse {
    match validate_set_reserve_transfer_amounts_payload(&transfer_amounts) {
        Ok(_) => {}
        Err(e) => {
            return SetReserveTransferAmountsResponse::InternalError(e);
        }
    }
    mutate_state(|s| {
        s.data.daily_reserve_transfer = transfer_amounts;
    });
    SetReserveTransferAmountsResponse::Success
}
