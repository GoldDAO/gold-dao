use candid::CandidType;
use ic_stable_structures::Storable;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use types::TimestampMillis;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct TokenSwap {
    pub swap_id: u128,
    pub swap_client_id: u128,
    pub started: TimestampMillis,
    pub deposit_account: SwapSubtask<Account>,
    pub transfer: SwapSubtask<u64>, // Block Index
    pub notified_dex_at: SwapSubtask,
    pub amount_swapped: SwapSubtask<Result<u128, String>>,
    pub withdrawn_from_dex_at: SwapSubtask<u128>,
    pub success: Option<bool>,
    pub is_archived: bool,
}

use candid::{Decode, Encode};
use ic_stable_structures::storable::Bound;
use std::borrow::Cow;
const MAX_SWAP_INFO_BYTES_SIZE: u32 = 1000;

impl Storable for TokenSwap {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_SWAP_INFO_BYTES_SIZE,
        is_fixed_size: false,
    };
}

type SwapSubtask<T = ()> = Option<Result<T, String>>;

impl TokenSwap {
    pub fn new(swap_id: u128, swap_client_id: u128, now: TimestampMillis) -> TokenSwap {
        TokenSwap {
            swap_id,
            swap_client_id,
            started: now,
            deposit_account: None,
            transfer: None,
            notified_dex_at: None,
            amount_swapped: None,
            withdrawn_from_dex_at: None,
            success: None,
            is_archived: false,
        }
    }
}
