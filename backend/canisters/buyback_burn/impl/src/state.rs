use std::collections::HashMap;

use buyback_burn_canister::get_config::Response as GetConfigResponse;
use candid::{CandidType, Principal};
use canister_state_macros::canister_state;
use serde::{Deserialize, Serialize};
use types::CanisterId;
use types::{Cycles, Milliseconds, TimestampMillis};
use utils::env::CanisterEnv;
use utils::env::Environment;
use utils::memory::MemorySize;

canister_state!(State);

#[derive(Serialize, Deserialize)]
pub struct State {
    pub env: CanisterEnv,
    pub data: Data,
}

impl State {
    pub fn new(env: CanisterEnv, data: Data) -> State {
        State { env, data }
    }

    pub fn is_caller_governance_principal(&self) -> bool {
        self.data.authorized_principals.contains(&self.env.caller())
    }

    pub fn get_config(&self) -> GetConfigResponse {
        GetConfigResponse {
            min_burn_amount: self.data.proposal_config.min_burn_amount,
        }
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            canister_info: CanisterInfo {
                test_mode: self.env.is_test_mode(),
                now: self.env.now(),
                memory_used: MemorySize::used(),
                cycles_balance: self.env.cycles_balance(),
            },
            authorized_principals: self.data.authorized_principals.iter().copied().collect(),
            sns_governance_canister: self.data.proposal_config.sns_governance_canister,
            min_burn_amount: self.data.proposal_config.min_burn_amount,
        }
    }
}

use crate::model::token_swaps::TokenSwaps;
use crate::timer_job_types::TimerJob;
use candid::Nat;
use canister_timer_jobs::TimerJobs;
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub authorized_principals: Vec<Principal>,
    pub proposal_config: ProposalConfig,
    pub token_swaps: TokenSwaps,
    pub timer_jobs: TimerJobs<TimerJob>,
    pub daily_gldgov_burn_rate: Option<Nat>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProposalConfig {
    pub sns_governance_canister: CanisterId,
    pub min_burn_amount: u128,
    pub gldgov_ledger_canister_id: Principal,
}

impl Data {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        authorized_principals: Vec<Principal>,
        sns_governance_canister: CanisterId,
        gldgov_ledger_canister_id: CanisterId,
        min_burn_amount: u128,
        now: TimestampMillis,
    ) -> Data {
        Data {
            authorized_principals: authorized_principals.into_iter().collect(),
            proposal_config: ProposalConfig {
                sns_governance_canister,
                min_burn_amount: min_burn_amount,
                gldgov_ledger_canister_id,
            },
            token_swaps: TokenSwaps::default(),
            timer_jobs: TimerJobs::default(),
            daily_gldgov_burn_rate: Some(Nat::from(min_burn_amount)),
        }
    }
}

// NOTE: trying to make a swapping configuration
use types::ICPSwapTokenInfo;

// #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct SwapConfig {
    pub swap_id: u128,
    pub input_token: ICPSwapTokenInfo,
    pub output_token: ICPSwapTokenInfo,
    pub input_amount: u128,
    pub exchange_args: ICPSwapArgs,
    pub min_burn_amount: u128,
    pub min_output_amount: u128,
}

// #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ICPSwapArgs {
    pub swap_canister_id: CanisterId,
    pub zero_for_one: bool,
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
