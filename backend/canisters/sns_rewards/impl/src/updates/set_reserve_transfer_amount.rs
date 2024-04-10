use std::collections::HashMap;

use crate::{ guards::caller_is_governance_principal, state::mutate_state };
use candid::{ CandidType, Nat };
use canister_tracing_macros::trace;
use ic_cdk::{ query, update };
use serde::{ Deserialize, Serialize };
use types::TokenSymbol;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum SetReserveTransferAmountResponse {
    Success,
    InternalError(String),
}
use SetReserveTransferAmountResponse::*;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SetReserveTransferAmountRequest {
    transfer_amounts: HashMap<TokenSymbol, Nat>,
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
pub async fn set_reserve_transfer_amounts(
    args: SetReserveTransferAmountRequest
) -> Result<SetReserveTransferAmountResponse, SetReserveTransferAmountResponse> {
    set_reserve_transfer_amounts_impl(args.transfer_amounts)
}

pub(crate) fn set_reserve_transfer_amounts_impl(
    transfer_amounts: HashMap<TokenSymbol, Nat>
) -> Result<SetReserveTransferAmountResponse, SetReserveTransferAmountResponse> {
    if transfer_amounts.len() < (1 as usize) {
        return Err(
            InternalError("Should contain at least 1 token symbol and amount to update".to_string())
        );
    }

    for (token_symbol, amount) in &transfer_amounts {
        // Check the amount is above 0.
        if amount == &Nat::from(0u64) {
            return Err(
                InternalError(
                    format!("ERROR : The amount for token : {:?} must be more than 0", token_symbol)
                )
            );
        }
    }

    mutate_state(|s| {
        s.data.daily_reserve_transfer = transfer_amounts;
    });
    Ok(Success)
}

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn set_reserve_transfer_amounts_validate(
    args: SetReserveTransferAmountRequest
) -> Result<String, String> {
    if args.transfer_amounts.len() < (1 as usize) {
        return Err("Should contain at least 1 token symbol and amount to update".to_string());
    }
    for (token_symbol, amount) in &args.transfer_amounts {
        // Check the amount is above 0.
        if amount == &Nat::from(0u64) {
            return Err(
                format!("ERROR : The amount for token : {:?} must be more than 0", token_symbol)
            );
        }
    }

    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}
