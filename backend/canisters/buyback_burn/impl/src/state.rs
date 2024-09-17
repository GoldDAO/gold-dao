use crate::types::token_swaps::TokenSwaps;
use buyback_burn_api::get_config::Response as GetConfigResponse;
use buyback_burn_api::init::TokenAndPool;
use candid::{ CandidType, Principal };
use canister_state_macros::canister_state;
use ic_ledger_types::Tokens;
use serde::{ Deserialize, Serialize };
use std::time::Duration;
use types::BuildVersion;
use types::{ Cycles, TimestampMillis, TokenInfo };
use utils::env::{ CanisterEnv, Environment };
use utils::memory::MemorySize;
use crate::types::SwapClients;
use tracing::error;

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
            min_icp_burn_amount: self.data.burn_config.min_burn_amount,
        }
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            canister_info: CanisterInfo {
                test_mode: self.env.is_test_mode(),
                now: self.env.now(),
                version: self.env.version(),
                commit_hash: self.env.commit_hash().to_string(),
                memory_used: MemorySize::used(),
                cycles_balance: self.env.cycles_balance(),
            },
            authorized_principals: self.data.authorized_principals.to_vec(),
            gldgov_token_info: self.data.gldgov_token_info,
            burn_config: self.data.burn_config.clone(),
            // TODO: add more metrics
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub authorized_principals: Vec<Principal>,
    pub gldgov_token_info: TokenInfo,
    pub icp_swap_canister_id: Principal,
    pub swap_interval: Duration,
    pub swap_clients: SwapClients,
    pub burn_config: BurnConfig,
    pub token_swaps: TokenSwaps,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct BurnConfig {
    pub burn_rate: u8,
    pub min_burn_amount: Tokens,
    pub burn_interval: Duration,
}

impl BurnConfig {
    fn new(burn_rate: u8, min_burn_amount: Tokens, burn_interval_in_secs: u64) -> Self {
        BurnConfig {
            // Check if the burn rate is valid. Otherwise set 0
            burn_rate: if burn_rate > 100 {
                error!("Burn rate must be between 0 and 100");
                0
            } else {
                burn_rate
            },
            min_burn_amount,
            burn_interval: Duration::from_secs(burn_interval_in_secs),
        }
    }
}

impl Data {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        authorized_principals: Vec<Principal>,
        tokens: Vec<TokenAndPool>,
        gldgov_token_info: TokenInfo,
        swap_interval_in_secs: u64,
        icp_swap_canister_id: Principal,
        burn_rate: u8,
        min_burn_amount: Tokens,
        burn_interval_in_secs: u64
    ) -> Self {
        let mut swap_clients = SwapClients::init();

        for token in tokens.iter() {
            swap_clients.add_swap_client(token.token, gldgov_token_info, token.swap_pool_id);
        }

        Self {
            authorized_principals: authorized_principals.into_iter().collect(),
            gldgov_token_info,
            swap_interval: Duration::from_secs(swap_interval_in_secs),
            swap_clients,
            icp_swap_canister_id,
            burn_config: BurnConfig::new(burn_rate, min_burn_amount, burn_interval_in_secs),
            token_swaps: TokenSwaps::default(),
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub authorized_principals: Vec<Principal>,
    pub gldgov_token_info: TokenInfo,
    pub burn_config: BurnConfig,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub memory_used: MemorySize,
    pub cycles_balance: Cycles,
}
