use crate::model::canisters::{CanisterMetrics, Canisters};
use candid::{CandidType, Principal};
use canister_state_macros::canister_state;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use types::{CanisterId, Cycles, Milliseconds, TimestampMillis};
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
            sns_root_canister: self.data.sns_root_canister,
            max_top_up_amount: self.data.max_top_up_amount,
            min_interval: self.data.min_interval,
            min_cycles_balance: self.data.min_cycles_balance,
        }
        // Metrics {
        //     canister_info: CanisterInfo {
        //         now: self.env.now(),
        //         test_mode: self.env.is_test_mode(),
        //         memory_used: MemorySize::used(),
        //         cycles_balance_in_tc: self.env.cycles_balance_in_tc(),
        //     },
        //     sns_governance_canister: self.data.sns_governance_canister,
        //     number_of_neurons: self.data.neuron_maturity.len(),
        //     sync_info: self.data.sync_info,
        //     authorized_principals: self.data.authorized_principals.clone(),
        //     daily_reserve_transfer: self.data.daily_reserve_transfer
        //         .iter()
        //         .map(|(token, val)| format!("{:?} - {}", token, val))
        //         .collect(),
        //     last_daily_reserve_transfer_time: self.data.last_daily_reserve_transfer_time,
        //     last_daily_gldgov_burn_time: self.data.last_daily_gldgov_burn.clone(),
        //     daily_gldgov_burn_amount: self.data.daily_gldgov_burn_rate.clone(),
        // }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub test_mode: bool,
    pub authorized_principals: HashSet<Principal>,
    pub canisters: Canisters,
    pub sns_root_canister: Option<CanisterId>,
    pub max_top_up_amount: Cycles,
    pub min_interval: Milliseconds,
    pub min_cycles_balance: Cycles,
}

impl Data {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        test_mode: bool,
        authorized_principals: Vec<Principal>,
        canisters: Vec<CanisterId>,
        sns_root_canister: Option<CanisterId>,
        max_top_up_amount: Cycles,
        min_interval: Milliseconds,
        min_cycles_balance: Cycles,
        now: TimestampMillis,
    ) -> Data {
        Data {
            test_mode,
            authorized_principals: authorized_principals.into_iter().collect(),
            canisters: Canisters::new(canisters, now),
            sns_root_canister,
            max_top_up_amount,
            min_interval,
            min_cycles_balance,
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub authorized_principals: Vec<Principal>,
    pub canisters: Vec<CanisterMetrics>,
    pub sns_root_canister: Option<CanisterId>,
    pub max_top_up_amount: Cycles,
    pub min_interval: Milliseconds,
    pub min_cycles_balance: Cycles,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    // pub cycles_balance_in_tc: f64,
    pub cycles_balance: Cycles,
}
