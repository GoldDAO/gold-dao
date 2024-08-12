use crate::types::token_swaps::TokenSwaps;
use crate::utils::build_icpswap_client;
use buyback_burn_canister::get_config::Response as GetConfigResponse;
use candid::{ CandidType, Principal };
use canister_state_macros::canister_state;
use ic_ledger_types::Tokens;
use icpswap_client::ICPSwapClient;
use serde::{ Deserialize, Serialize };
use types::CanisterId;
use types::TokenInfo;
use types::{ Cycles, TimestampMillis };
use utils::env::CanisterEnv;
use utils::env::Environment;
use utils::memory::MemorySize;
use crate::token_swap::SwapClient;

canister_state!(RuntimeState);

#[derive(Serialize, Deserialize)]
pub struct RuntimeState {
    pub env: CanisterEnv,
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: CanisterEnv, data: Data) -> Self {
        RuntimeState { env, data }
    }

    pub fn is_caller_governance_principal(&self) -> bool {
        self.data.authorized_principals.contains(&self.env.caller())
    }

    pub fn get_config(&self) -> GetConfigResponse {
        GetConfigResponse {
            burn_rate: self.data.burn_config.burn_rate,
            min_icp_burn_amount: self.data.burn_config.min_icp_burn_amount,
        }
    }

    // pub fn metrics(&self) -> Metrics {
    //     Metrics {
    //         canister_info: CanisterInfo {
    //             test_mode: self.env.is_test_mode(),
    //             now: self.env.now(),
    //             memory_used: MemorySize::used(),
    //             cycles_balance: self.env.cycles_balance(),
    //         },
    //         authorized_principals: self.data.authorized_principals.iter().copied().collect(),
    //         sns_governance_canister: self.data.proposal_config.sns_governance_canister,
    //         min_burn_amount: self.data.proposal_config.min_burn_amount,
    //     }
    // }
}

use types::TokenSymbol;
use std::collections::HashMap;
use std::time::Duration;
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub authorized_principals: Vec<Principal>,
    pub gldgov_ledger_canister_id: CanisterId,
    // pub tokens: HashMap<TokenSymbol, TokenInfo>,
    // pub icpswap_clients: Vec<CanisterId>,
    pub icpswap_client: Box<dyn SwapClient>,
    pub burn_config: BurnConfig,
    pub token_swaps: TokenSwaps,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BurnConfig {
    pub burn_address: CanisterId,
    pub burn_rate: u8,
    pub min_icp_burn_amount: Tokens,
    pub burn_interval: Duration,
}

impl Data {
    pub fn new(
        authorized_principals: Vec<Principal>,
        tokens: HashMap<TokenSymbol, TokenInfo>,
        gldgov_ledger_canister_id: CanisterId,
        sns_governance_canister_id: Principal,
        burn_rate: u8,
        min_icp_burn_amount: Tokens,
        burn_interval_in_secs: u64,
        this_canister_id: Principal
    ) -> Data {
        Data {
            authorized_principals: authorized_principals.into_iter().collect(),
            gldgov_ledger_canister_id,
            // tokens,
            icpswap_client: build_icpswap_client(&SwapConfig::default(), this_canister_id),
            burn_config: BurnConfig {
                burn_address: sns_governance_canister_id,
                burn_rate,
                min_icp_burn_amount,
                burn_interval: Duration::from_secs(burn_interval_in_secs),
            },
            token_swaps: TokenSwaps::default(),
        }
    }
}

// #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct SwapConfig {
    pub swap_client_id: u128,
    pub input_token: TokenInfo,
    pub output_token: TokenInfo,
    pub swap_canister_id: Principal,
    pub zero_for_one: bool,
}

impl Default for SwapConfig {
    fn default() -> Self {
        Self {
            swap_client_id: 0,
            input_token: TokenInfo::icp(),
            output_token: TokenInfo::gldgov(),
            swap_canister_id: Principal::from_text("7eikv-2iaaa-aaaag-qdgwa-cai").unwrap(),
            zero_for_one: true,
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub authorized_principals: Vec<Principal>,
    pub sns_governance_canister: CanisterId,
    // pub cycles_minting_canister: CanisterId,
    pub min_burn_amount: u128,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    pub cycles_balance: Cycles,
}
