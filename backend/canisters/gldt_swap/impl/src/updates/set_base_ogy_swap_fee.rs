use crate::{guards::caller_is_authorized, state::mutate_state};
use candid::Nat;
use canister_tracing_macros::trace;
pub use gldt_swap_api_canister::set_base_ogy_swap_fee::{
    Args as SetBaseOgySwapFeeArgs, Response as SetBaseOgySwapFeeResponse,
};
use ic_cdk::update;

#[update(guard = "caller_is_authorized", hidden = true)]
#[trace]
pub async fn set_base_ogy_swap_fee(amount: SetBaseOgySwapFeeArgs) -> SetBaseOgySwapFeeResponse {
    match validate_set_base_ogy_swap_fee_payload(&amount) {
        Ok(_) => {
            mutate_state(|s| {
                s.data.base_ogy_swap_fee = amount;
            });
            SetBaseOgySwapFeeResponse::Success
        }
        Err(e) => {
            return SetBaseOgySwapFeeResponse::InternalError(e);
        }
    }
}

pub fn validate_set_base_ogy_swap_fee_payload(amount: &Nat) -> Result<(), String> {
    if amount == &Nat::from(0u64) {
        return Err(format!(
            "ERROR : The base fee for a single swap must be more than 0"
        ));
    }
    Ok(())
}
