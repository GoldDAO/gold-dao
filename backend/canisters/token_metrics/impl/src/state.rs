use candid::Principal;
use utils::{
    consts::{ ICP_LEDGER_CANISTER_ID, NNS_GOVERNANCE_CANISTER_ID, SNS_GOVERNANCE_CANISTER_ID },
    env::{ CanisterEnv, Environment },
    memory::MemorySize,
};
use canister_state_macros::canister_state;
use serde::{ Serialize, Deserialize };

canister_state!(RuntimeState);

#[derive(Serialize, Deserialize)]
pub struct RuntimeState {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: CanisterEnv, data: Data) -> Self {
        Self { env, data }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub gold_price: f64,
    pub gold_nft_canisters: Vec<(Principal, u64)>,
    pub total_gold_grams: u64,
}

impl Data {
    pub fn new(gold_nft_canisters: Vec<(Principal, u64)>) -> Self {
        Self {
            gold_price: 0.0,
            gold_nft_canisters: gold_nft_canisters,
            total_gold_grams: 0,
        }
    }
}
