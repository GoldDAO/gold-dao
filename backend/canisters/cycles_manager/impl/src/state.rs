use crate::model::canisters::{CanisterMetrics, Canisters};
use candid::{CandidType, Principal};
use canister_state_macros::canister_state;
// use ic_ledger_types::BlockIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
// use types::{BuildVersion, Timestamped};
use types::{CanisterId, Cycles, Milliseconds, TimestampMillis};
use utils::env::CanisterEnv;
use utils::env::Environment;

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
            now: self.env.now(),
            cycles_balance: self.env.cycles_balance(),
            // wasm_version: WASM_VERSION.with_borrow(|v| **v),
            // git_commit_id: utils::git::git_commit_id().to_string(),
            authorized_principals: self.data.authorized_principals.iter().copied().collect(),
            canisters: self.data.canisters.metrics(),
            sns_root_canister: self.data.sns_root_canister,
            max_top_up_amount: self.data.max_top_up_amount,
            min_interval: self.data.min_interval,
            min_cycles_balance: self.data.min_cycles_balance,
        }
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
    pub rng_seed: [u8; 32],
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
            rng_seed: [0; 32],
        }
    }
}

#[derive(CandidType, Serialize, Debug)]
pub struct Metrics {
    pub now: TimestampMillis,
    pub cycles_balance: Cycles,
    // pub wasm_version: BuildVersion,
    // pub git_commit_id: String,
    pub authorized_principals: Vec<Principal>,
    pub canisters: Vec<CanisterMetrics>,
    pub sns_root_canister: Option<CanisterId>,
    pub max_top_up_amount: Cycles,
    pub min_interval: Milliseconds,
    pub min_cycles_balance: Cycles,
}
