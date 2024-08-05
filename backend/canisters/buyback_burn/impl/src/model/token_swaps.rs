use candid::CandidType;
use ic_ledger_types::{BlockIndex, Subaccount};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::TimestampMillis;
// use user_canister::token_swap_status::TokenSwapStatus;
use crate::memory::get_swap_history_memory;
use crate::memory::VM;
use crate::model::token_swap_status::TokenSwapStatus;
use crate::state::SwapConfig;
use ic_stable_structures::StableBTreeMap;
use ic_stable_structures::Storable;
use icrc_ledger_types::icrc1::account::Account;

#[derive(Serialize, Deserialize)]
pub struct TokenSwaps {
    swaps: HashMap<u128, TokenSwap>,
    // #[serde(skip, default = "init_map")]
    // history: HashMap<u128, TokenSwap>,
    #[serde(skip, default = "init_map")]
    history: StableBTreeMap<BlockIndex, TokenSwap, VM>,
}

fn init_map() -> StableBTreeMap<BlockIndex, TokenSwap, VM> {
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
        let token_swap = TokenSwap::new(args, now);
        self.upsert(token_swap.clone());
        token_swap
    }

    pub fn upsert(&mut self, swap: TokenSwap) {
        self.swaps.insert(swap.args.swap_id, swap);
    }

    pub fn get(&self, swap_id: u128) -> Option<&TokenSwap> {
        self.swaps.get(&swap_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenSwap> {
        self.swaps.values()
    }
}

// #[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct TokenSwap {
    pub args: SwapConfig,
    pub started: TimestampMillis,
    pub deposit_account: SwapSubtask<Account>,
    pub transfer: SwapSubtask<u64>, // Block Index
    pub notified_dex_at: SwapSubtask,
    pub amount_swapped: SwapSubtask<Result<u128, String>>,
    pub withdrawn_from_dex_at: SwapSubtask<u128>,
    pub success: Option<bool>,
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
    pub fn new(args: SwapConfig, now: TimestampMillis) -> TokenSwap {
        TokenSwap {
            args,
            started: now,
            deposit_account: None,
            transfer: None,
            notified_dex_at: None,
            amount_swapped: None,
            withdrawn_from_dex_at: None,
            success: None,
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
