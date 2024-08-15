use std::time::Duration;
use candid::{ CandidType, Principal };
use serde::{ Deserialize, Serialize };
use buyback_burn_canister::get_config::Response as GetConfigResponse;
use canister_state_macros::canister_state;
use ic_ledger_types::Tokens;
use utils::env::{ CanisterEnv, Environment };
use utils::memory::MemorySize;
use types::{ CanisterId, Cycles, TimestampMillis, TokenInfo };
use crate::types::{ ExchangeConfig, SwapClinets, SwapConfig, icpswap::ICPSwapConfig };
use crate::types::token_swaps::TokenSwaps;
use canister_timer_jobs::TimerJobs;
use crate::timer_job_types::TimerJob;

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

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub authorized_principals: Vec<Principal>,
    pub gldgov_ledger_canister_id: CanisterId,
    pub swap_interval: Duration,
    pub swap_clients: SwapClinets,
    pub burn_config: BurnConfig,
    pub token_swaps: TokenSwaps,
    pub timer_jobs: TimerJobs<TimerJob>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BurnConfig {
    pub burn_address: CanisterId,
    pub burn_rate: u8,
    pub min_burn_amount: Tokens,
    pub burn_interval: Duration,
}

impl BurnConfig {
    fn validate_burn_rate(&self) -> bool {
        // the burn rate should be between 1 and 100
        self.burn_rate > 100 || self.burn_rate == 0
    }
}

impl Data {
    pub fn new(
        authorized_principals: Vec<Principal>,
        tokens: Vec<TokenInfo>,
        gldgov_ledger_canister_id: CanisterId,
        swap_interval_in_secs: u64,
        sns_governance_canister_id: Principal,
        burn_rate: u8,
        min_burn_amount: Tokens,
        burn_interval_in_secs: u64
    ) -> Data {
        let mut swap_clients = SwapClinets::init();
        // TODO: add other tokens support
        swap_clients.add_swap_client(SwapConfig {
            swap_client_id: 0,
            input_token: TokenInfo::icp(),
            output_token: TokenInfo::gldgov(),
            exchange_config: ExchangeConfig::ICPSwap(ICPSwapConfig::default()),
        });
        // NOTE: here we add all other tokens except of
        for (id, token) in tokens.iter().enumerate() {
            swap_clients.add_swap_client(SwapConfig {
                swap_client_id: (id as u128) + 1,
                input_token: TokenInfo::icp(),
                output_token: *token,
                exchange_config: ExchangeConfig::ICPSwap(ICPSwapConfig::default()),
            });
        }

        Data {
            authorized_principals: authorized_principals.into_iter().collect(),
            gldgov_ledger_canister_id,
            swap_interval: Duration::from_secs(swap_interval_in_secs),
            // icpswap_client: ,
            swap_clients,
            burn_config: BurnConfig {
                burn_address: sns_governance_canister_id,
                burn_rate,
                min_burn_amount,
                burn_interval: Duration::from_secs(burn_interval_in_secs),
            },
            token_swaps: TokenSwaps::default(),
            timer_jobs: TimerJobs::default(),
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub authorized_principals: Vec<Principal>,
    pub sns_governance_canister: CanisterId,
    pub min_burn_amount: u128,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    pub cycles_balance: Cycles,
}
