use std::collections::HashMap;

use crate::{ guards::caller_is_governance_principal, state::mutate_state };
use candid::{ CandidType, Nat };
use canister_tracing_macros::trace;
use ic_cdk::{ query, update };
use serde::{ Deserialize, Serialize };
use types::TokenSymbol;

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SetReserveTransferAmountResponse {
    Success,
    InternalError(String),
}
use SetReserveTransferAmountResponse::*;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SetReserveTransferAmountRequest {
    pub transfer_amounts: HashMap<TokenSymbol, Nat>,
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
pub async fn set_reserve_transfer_amounts(
    args: SetReserveTransferAmountRequest
) -> Result<SetReserveTransferAmountResponse, SetReserveTransferAmountResponse> {
    set_reserve_transfer_amounts_impl(args.transfer_amounts)
}

// this will overwrite the hashmap completely so any tokens not passed in will be removed.
pub(crate) fn set_reserve_transfer_amounts_impl(
    transfer_amounts: HashMap<TokenSymbol, Nat>
) -> Result<SetReserveTransferAmountResponse, SetReserveTransferAmountResponse> {
    match validate_payload(&transfer_amounts) {
        Ok(_) => {}
        Err(e) => {
            return Err(InternalError(e));
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
    validate_payload(&args.transfer_amounts)?;
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

pub fn validate_payload(args: &HashMap<TokenSymbol, Nat>) -> Result<(), String> {
    if args.len() < (1 as usize) {
        return Err("Should contain at least 1 token symbol and amount to update".to_string());
    }

    for (token_symbol, amount) in args {
        // Check the amount is above 0.
        if amount == &Nat::from(0u64) {
            return Err(
                format!("ERROR : The amount for token : {:?} must be more than 0", token_symbol)
            );
        }
    }
    Ok(())
}
