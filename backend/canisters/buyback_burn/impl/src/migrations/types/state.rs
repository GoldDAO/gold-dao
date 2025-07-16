use crate::types::token_swaps::TokenSwaps;
use crate::types::SwapClients;
use candid::Principal;
use ic_ledger_types::Tokens;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use types::TokenInfo;
use utils::env::CanisterEnv;

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: DataV0,
}

#[derive(Serialize, Deserialize)]
pub struct DataV0 {
    pub authorized_principals: Vec<Principal>,
    pub gldgov_token_info: TokenInfo,
    pub icp_swap_canister_id: Principal,
    pub buyback_burn_interval: Duration,
    pub swap_clients: SwapClients,
    pub burn_config: BurnConfigV0,
    pub token_swaps: TokenSwaps,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BurnConfigV0 {
    pub burn_rate: u8,
    pub min_burn_amount: Tokens,
}
