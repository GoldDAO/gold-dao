use crate::types::token_swaps::TokenSwaps;
use crate::types::ExchangeJobs;
use candid::Principal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::collections::HashMap;
use utils::env::CanisterEnv;

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    pub env: CanisterEnv,
    pub data: DataV0,
}

#[derive(Serialize, Deserialize)]
pub struct DataV0 {
    pub authorized_principals: Vec<Principal>,
    pub token_swaps: TokenSwapsV0,

    // NOTE: the main ICP Swap canister
    pub icp_swap_canister_id: Principal,
    pub exchange_jobs: ExchangeJobs,

    // storage for swap guard see guards.rs
    pub exchange_job_guards: BTreeSet<u128>,
}

// #[derive(CandidType, Serialize, Deserialize, Clone)]
// pub struct SwapClientsV0 {
//     pub swap_clients: HashMap<u128, SwapClientEnumV0>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct BurnConfigV0 {
//     pub burn_percentage: Percentage,
//     pub min_burn_amount: Tokens,
// }

// #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
// pub enum SwapClientEnumV0 {
//     ICPSwapClient(ICPSwapClientV0),
// }

// use types::CanisterId;
// #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
// pub struct ICPSwapClientV0 {
//     client_id: u128,
//     this_canister_id: CanisterId,
//     swap_canister_id: CanisterId,
//     token0: TokenInfo,
//     token1: TokenInfo,
//     zero_for_one: bool,
// }

use crate::memory::VM;
use crate::types::TokenSwap;
use ic_stable_structures::StableBTreeMap;
#[derive(Serialize, Deserialize)]
pub struct TokenSwapsV0 {
    swaps: HashMap<u128, TokenSwap>,
    #[serde(skip, default = "init_map")]
    history: StableBTreeMap<u128, TokenSwap, VM>,
}

use crate::memory::get_swap_history_memory;
fn init_map() -> StableBTreeMap<u128, TokenSwap, VM> {
    let memory = get_swap_history_memory();
    StableBTreeMap::init(memory)
}

impl From<TokenSwapsV0> for TokenSwaps {
    fn from(old: TokenSwapsV0) -> Self {
        let history = init_map();

        // Calculate the next ID based on current state to prevent collisions
        let active_max = old.swaps.keys().max().cloned().unwrap_or(0);
        let history_max = history.iter().map(|(k, _)| k).max().unwrap_or(0);

        // Next ID should be 1 higher than the highest ID ever seen
        let next_id = std::cmp::max(active_max, history_max) + 1;

        Self {
            next_id,
            swaps: old.swaps,
            history,
        }
    }
}
