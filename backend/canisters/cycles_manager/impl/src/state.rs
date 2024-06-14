use crate::model::canisters::Canisters;
use candid::{CandidType, Principal};
use canister_state_macros::canister_state;
use cycles_manager_api_canister::get_canisters_summary::CanisterMetrics;
use cycles_manager_api_canister::get_config::Response as GetConfigResponse;
use ic_ledger_types::BlockIndex;
use ic_ledger_types::Tokens;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use types::{CanisterId, Cycles, TimestampMillis};
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

    pub fn get_top_up_config(&self) -> GetConfigResponse {
        GetConfigResponse {
            max_top_up_amount: self.data.top_up_config.max_top_up_amount,
            min_cycles_balance: self.data.top_up_config.min_cycles_balance,
        }
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            canister_info: CanisterInfo {
                now: self.env.now(),
                test_mode: self.env.is_test_mode(),
                memory_used: MemorySize::used(),
                cycles_balance: self.env.cycles_balance(),
            },
            authorized_principals: self.data.authorized_principals.iter().copied().collect(),
            canisters: self.data.canisters.metrics(),
            sns_root_canister: self.data.top_up_config.sns_root_canister,
            max_top_up_amount: self.data.top_up_config.max_top_up_amount,
            min_cycles_balance: self.data.top_up_config.min_cycles_balance,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub authorized_principals: HashSet<Principal>,
    pub canisters: Canisters,
    pub top_up_config: TopUpConfig,
    pub burn_config: BurnConfig,
}

impl Data {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        authorized_principals: Vec<Principal>,
        canisters: Vec<CanisterId>,
        sns_root_canister: CanisterId,
        max_top_up_amount: Cycles,
        min_cycles_balance: Cycles,
        icp_burn_amount: Tokens,
        ledger_canister: CanisterId,
        cycles_minting_canister: CanisterId,
        now: TimestampMillis,
    ) -> Data {
        Data {
            authorized_principals: authorized_principals.into_iter().collect(),
            canisters: Canisters::new(canisters, now),
            top_up_config: TopUpConfig {
                sns_root_canister,
                max_top_up_amount,
                min_cycles_balance,
            },
            burn_config: BurnConfig {
                icp_burn_amount,
                ledger_canister,
                cycles_minting_canister,
                cycles_top_up_pending_notification: None,
            },
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub authorized_principals: Vec<Principal>,
    pub canisters: Vec<CanisterMetrics>,
    pub sns_root_canister: CanisterId,
    pub max_top_up_amount: Cycles,
    pub min_cycles_balance: Cycles,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    pub cycles_balance: Cycles,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct TopUpConfig {
    pub sns_root_canister: CanisterId,
    pub max_top_up_amount: Cycles,
    pub min_cycles_balance: Cycles,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct BurnConfig {
    pub icp_burn_amount: Tokens,
    pub ledger_canister: CanisterId,
    pub cycles_minting_canister: CanisterId,
    pub cycles_top_up_pending_notification: Option<BlockIndex>,
}
