use crate::state::read_state;
use candid::{Nat, Principal};
use ic_xrc_types::{Asset, AssetClass, GetExchangeRateRequest, GetExchangeRateResult};
use icrc_ledger_client_cdk::{CdkRuntime, ICRC1Client};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{TransferArg, TransferError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use std::fmt;

/// Represents an error from a management canister call
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallError {
    method: String,
    reason: Reason,
}

impl CallError {
    /// Returns the name of the method that resulted in this error.
    pub fn method(&self) -> &str {
        &self.method
    }

    /// Returns the failure reason.
    pub fn reason(&self) -> &Reason {
        &self.reason
    }
}

impl fmt::Display for CallError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "management call '{}' failed: {}",
            self.method, self.reason
        )
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
/// The reason for the management call failure.
pub enum Reason {
    /// The canister does not have enough cycles to submit the request.
    OutOfCycles,
    /// The call failed with an error.
    CanisterError(String),
    /// The management canister rejected the signature request (not enough
    /// cycles, the ECDSA subnet is overloaded, etc.).
    Rejected(String),
    /// The call failed with a transient error. Retrying may help.
    TransientInternalError(String),
    /// The call failed with a non-transient error. Retrying will not help.
    InternalError(String),
}

impl fmt::Display for Reason {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfCycles => write!(fmt, "the canister is out of cycles"),
            Self::CanisterError(msg) => write!(fmt, "canister error: {}", msg),
            Self::Rejected(msg) => {
                write!(fmt, "the management canister rejected the call: {}", msg)
            }
            Reason::TransientInternalError(msg) => write!(fmt, "transient internal error: {}", msg),
            Reason::InternalError(msg) => write!(fmt, "internal error: {}", msg),
        }
    }
}

/// Query the XRC canister to retrieve the last GOLD/USD price.
/// https://github.com/dfinity/exchange-rate-canister
pub async fn fetch_gold_price() -> Result<GetExchangeRateResult, String> {
    const XRC_CALL_COST_CYCLES: u64 = 10_000_000_000;

    // 1 GLDT = 0.01g of gold
    // 1 PAXG = 31.1034768g of gold

    let gold = Asset {
        symbol: "PAXG".to_string(),
        class: AssetClass::Cryptocurrency,
    };
    let usd = Asset {
        symbol: "USD".to_string(),
        class: AssetClass::FiatCurrency,
    };

    let args = GetExchangeRateRequest {
        base_asset: gold,
        quote_asset: usd,
        timestamp: None,
    };

    let xrc_principal = read_state(|s| s.xrc_id);

    let res_xrc: Result<(GetExchangeRateResult,), (i32, String)> =
        ic_cdk::api::call::call_with_payment(
            xrc_principal,
            "get_exchange_rate",
            (args,),
            XRC_CALL_COST_CYCLES,
        )
        .await
        .map_err(|(code, msg)| (code as i32, msg));
    match res_xrc {
        Ok((xr,)) => Ok(xr),
        Err((code, msg)) => Err(format!(
            "Error while calling XRC canister ({}): {:?}",
            code, msg
        )),
    }
}

pub async fn transfer(
    to: impl Into<Account>,
    amount: Nat,
    fee: Option<Nat>,
    ledger_canister_id: Principal,
) -> Result<u64, TransferError> {
    let client = ICRC1Client {
        runtime: CdkRuntime,
        ledger_canister_id,
    };
    let block_index = client
        .transfer(TransferArg {
            from_subaccount: None,
            to: to.into(),
            fee,
            created_at_time: None,
            memo: None,
            amount,
        })
        .await
        .map_err(|e| TransferError::GenericError {
            error_code: (Nat::from(e.0 as u32)),
            message: (e.1),
        })??;
    Ok(block_index.0.try_into().unwrap())
}

pub async fn transfer_from(
    from: impl Into<Account>,
    to: impl Into<Account>,
    amount: Nat,
    fee: Option<Nat>,
    ledger_canister_id: Principal,
) -> Result<u64, TransferFromError> {
    let client = ICRC1Client {
        runtime: CdkRuntime,
        ledger_canister_id,
    };
    let block_index = client
        .transfer_from(TransferFromArgs {
            spender_subaccount: None,
            from: from.into(),
            to: to.into(),
            amount,
            fee,
            created_at_time: None,
            memo: None,
        })
        .await
        .map_err(|e| TransferFromError::GenericError {
            error_code: (Nat::from(e.0 as u32)),
            message: (e.1),
        })??;
    Ok(block_index.0.try_into().unwrap())
}
