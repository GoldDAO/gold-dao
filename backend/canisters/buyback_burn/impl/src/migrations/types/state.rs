use crate::types::token_swaps::TokenSwaps;
use candid::CandidType;
use candid::Principal;
use ic_ledger_types::Tokens;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use types::TokenInfo;
use utils::env::CanisterEnv;
use utils::numeric::Percentage;

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    pub env: CanisterEnv,
    pub data: DataV0,
}

#[derive(Serialize, Deserialize)]
pub struct DataV0 {
    pub authorized_principals: Vec<Principal>,
    pub gldgov_token_info: TokenInfo,
    pub icp_swap_canister_id: Principal,
    pub buyback_interval: Duration,
    pub swap_clients: SwapClientsV0,
    pub burn_config: BurnConfigV0,
    pub token_swaps: TokenSwaps,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct SwapClientsV0 {
    pub swap_clients: HashMap<u128, SwapClientEnumV0>,
}

#[derive(Serialize, Deserialize)]
pub struct BurnConfigV0 {
    pub burn_percentage: Percentage,
    pub min_burn_amount: Tokens,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum SwapClientEnumV0 {
    ICPSwapClient(ICPSwapClientV0),
}

use types::CanisterId;
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ICPSwapClientV0 {
    client_id: u128,
    this_canister_id: CanisterId,
    swap_canister_id: CanisterId,
    token0: TokenInfo,
    token1: TokenInfo,
    zero_for_one: bool,
}
