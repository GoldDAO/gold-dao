use crate::logs::{DEBUG, INFO};
use crate::numeric::DisplayAmount;
use crate::state::{mutate_state, read_state};
use candid::Nat;
use candid::{CandidType, Principal};
use gldt_swap_common::gldt::GLDT_TX_FEE;
use ic_canister_log::log;
use icrc_ledger_types::icrc1::account::Account;
use minicbor::{Decode, Encode};
use serde::Deserialize;
use std::fmt;

pub type TransferId = u64;

#[derive(CandidType, Clone, Debug, PartialEq, Eq, Decode, Encode, Deserialize, Copy)]
#[allow(non_camel_case_types)]
pub enum Unit {
    #[n(0)]
    USDG = 0,
    #[n(1)]
    GLDT = 1,
}

impl Unit {
    pub fn ledger_id(&self) -> Principal {
        match self {
            Unit::USDG => read_state(|s| s.usdg_ledger_id),
            Unit::GLDT => read_state(|s| s.gldt_ledger_id),
        }
    }

    pub fn fee(&self) -> u64 {
        match self {
            Unit::GLDT => GLDT_TX_FEE,
            Unit::USDG => 0, // The transfer fee should be 0 as this canister is the minter.
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let unit_str = match *self {
            Unit::GLDT => "GLDT",
            Unit::USDG => "USDG",
        };
        write!(f, "{}", unit_str)
    }
}

#[derive(CandidType, Clone, Debug, PartialEq, Eq, Encode, Decode, Deserialize)]
pub struct PendingTransfer {
    #[n(0)]
    pub transfer_id: TransferId,
    #[n(1)]
    pub amount: u64,
    #[cbor(n(2), with = "crate::cbor::account")]
    pub receiver: Account,
    #[n(3)]
    pub unit: Unit,
}

pub async fn process_pending_transfer() -> u64 {
    let mut error_count = 0;

    let pending_transfers: Vec<PendingTransfer> = read_state(|s| {
        s.pending_transfers
            .values()
            .cloned()
            .collect::<Vec<PendingTransfer>>()
    });

    for transfer in pending_transfers {
        let (ledger_id, fee) = (transfer.unit.ledger_id(), transfer.unit.fee());
        match crate::management::transfer(
            transfer.receiver,
            transfer
                .amount
                .checked_sub(fee)
                .expect("bug: transfer should always be greater than the fee.")
                .into(),
            Some(Nat::from(fee)),
            ledger_id,
        )
        .await
        {
            Ok(_block_index) => {
                log!(
                    INFO,
                    "[process_pending_transfer] successfully transfered: {} {} to {}, transfer id: {}",
                    DisplayAmount(transfer.amount),
                    transfer.unit,
                    transfer.receiver,
                    transfer.transfer_id
                );
                mutate_state(|s| {
                    // process_event(
                    //     s,
                    //     EventType::TransferExecuted {
                    //         transfer_id: transfer.transfer_id,
                    //         block_index,
                    //     },
                    // )
                    s.record_process_pending_transfer(transfer.transfer_id);
                });
            }
            Err(error) => {
                log!(
                    DEBUG,
                    "[process_pending_transfer] failed to transfer margin: {}, with error: {}",
                    transfer.amount,
                    error
                );
                error_count += 1;
            }
        }
    }
    error_count
}
