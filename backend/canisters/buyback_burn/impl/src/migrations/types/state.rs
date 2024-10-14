use serde::{ Deserialize, Serialize };
use crate::state::BurnConfig;

use crate::types::token_swaps::TokenSwaps;
use candid::Principal;
use std::time::Duration;
use types::TokenInfo;
use utils::env::CanisterEnv;
use crate::types::SwapClients;

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub authorized_principals: Vec<Principal>,
    pub gldgov_token_info: TokenInfo,
    pub icp_swap_canister_id: Principal,
    pub buyback_burn_interval: Duration,
    pub swap_clients: SwapClients,
    pub burn_config: BurnConfig,
    pub token_swaps: TokenSwaps,
}
