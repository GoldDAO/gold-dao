use candid::CandidType;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use types::TimestampMillis;
use crate::memory::get_swap_history_memory;
use crate::memory::VM;
use crate::swap_clients::SwapConfig;
use crate::types::token_swap_status::SwapStatus;
use ic_stable_structures::StableBTreeMap;
use ic_stable_structures::Storable;
use icrc_ledger_types::icrc1::account::Account;
use tracing::error;

#[derive(Serialize, Deserialize)]
pub struct TokenSwaps {
    swaps: HashMap<u128, TokenSwap>,
    #[serde(skip, default = "init_map")]
    history: StableBTreeMap<u128, TokenSwap, VM>,
}

fn init_map() -> StableBTreeMap<u128, TokenSwap, VM> {
    let memory = get_swap_history_memory();
    StableBTreeMap::init(memory)
}

impl Default for TokenSwaps {
    fn default() -> Self {
        Self {
            swaps: HashMap::new(),
            history: init_map(),
        }
    }
}

// #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
// pub struct Args {
//     pub swap_id: u128,
//     // pub input_token: TokenInfo,
//     // pub output_token: TokenInfo,
//     // pub input_amount: u128,
//     // pub exchange_args: ExchangeArgs,
//     pub min_output_amount: u128,
//     pub pin: Option<String>,
// }

impl TokenSwaps {
    pub fn push_new(&mut self, args: SwapConfig, now: TimestampMillis) -> TokenSwap {
        let token_swap = TokenSwap::new(now);
        // FIXME: fix here the swap id
        self.upsert(token_swap.clone());
        token_swap
    }

    pub fn upsert(&mut self, swap: TokenSwap) {
        // FIXME: fix here the swap id
        self.swaps.insert(0, swap);
    }

    pub fn get(&self, swap_id: u128) -> Option<&TokenSwap> {
        self.swaps.get(&swap_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenSwap> {
        self.swaps.values()
    }

    pub fn get_swap_info(&self, swap_id: u128) -> Option<TokenSwap> {
        let swap_info_incomplete = self.swaps.get(&swap_id).cloned();
        let swap_info_completed = self.history.get(&swap_id);
        swap_info_incomplete.or(swap_info_completed)
    }

    pub fn archive_swap(&mut self, swap_id: u128) -> Result<(), ()> {
        let swap_info = self.swaps.get(&swap_id);
        match swap_info {
            Some(swap) => {
                let mut modified_swap = swap.clone();
                modified_swap.is_archived = true;
                self.history.insert(swap_id, modified_swap.clone());
                self.swaps.remove(&swap_id);
                Ok(())
            }
            None => {
                error!("Failed to archive {swap_id} because it doesn't exist in swap heap memory");
                Err(())
            }
        }
    }

    // pub total_amount_swapped: u64,
    // pub number_of_completed_swaps: u64,
    // pub number_of_attempted_swaps: u64,
    // pub number_of_failed_swaps: u64,
    // pub user_swaps: HashMap<Principal, UserSwap>,
    pub fn get_metrics(&self) {}
}

// #[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct TokenSwap {
    // pub args: SwapConfig,
    pub status: SwapStatus,
    pub started: TimestampMillis,
    pub deposit_account: SwapSubtask<Account>,
    pub transfer: SwapSubtask<u64>, // Block Index
    pub notified_dex_at: SwapSubtask,
    pub amount_swapped: SwapSubtask<Result<u128, String>>,
    pub withdrawn_from_dex_at: SwapSubtask<u128>,
    pub success: Option<bool>,
    pub is_archived: bool,
}

use candid::Decode;
use candid::Encode;
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
    pub fn new(now: TimestampMillis) -> TokenSwap {
        TokenSwap {
            // args,
            status: SwapStatus::Init,
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

// impl From<TokenSwap> for TokenSwapStatus {
//     fn from(value: TokenSwap) -> Self {
//         TokenSwapStatus {
//             started: value.started,
//             deposit_account: value.deposit_account.map(|a| a.value.map(|_| ())),
//             transfer: value.transfer.map(|t| t.value),
//             notify_dex: value.notified_dex_at.map(|t| t.value.map(|_| ())),
//             amount_swapped: value.amount_swapped.as_ref().map(|t| t.value.clone()),
//             withdraw_from_dex: value.withdrawn_from_dex_at.map(|t| t.value),
//             success: value.success.map(|t| t.value),
//         }
//     }
// }
