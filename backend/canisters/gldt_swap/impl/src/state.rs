use crate::model::swap_configs::SwapConfigs;
use crate::model::swap_system::SwapSystem;
use bity_ic_canister_state_macros::canister_state;
use bity_ic_icrc3::transaction::TransactionType;
use bity_ic_types::BuildVersion;
use candid::{CandidType, Nat, Principal};
use gldt_swap_common::nft::Nft;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use tracing::debug;
use types::TimestampMillis;
use utils::{
    env::{CanisterEnv, Environment},
    memory::MemorySize,
};

canister_state!(RuntimeState);
use bity_ic_icrc3_macros::icrc3_state;
icrc3_state!();

#[derive(Default, Serialize, Deserialize)]
pub struct RuntimeState {
    pub env: CanisterEnv,
    pub data: Data,
}

pub type FeeAccount = Account;

impl RuntimeState {
    pub fn new(env: CanisterEnv, data: Data) -> Self {
        Self { env, data }
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            canister_info: CanisterInfo {
                now: self.env.now(),
                test_mode: self.env.is_test_mode(),
                memory_used: MemorySize::used(),
                cycles_balance_in_tc: self.env.cycles_balance_in_tc(),
                version: self.env.version(),
                commit_hash: self.env.commit_hash().to_string(),
            },
            total_active_swaps: self.get_number_active_swaps(),
            total_stuck_swaps: self.get_total_stuck_swaps(),
            authorized_principals: self.data.authorized_principals.clone(),
            is_gldt_supply_balancer_running: self.data.is_gldt_supply_balancer_running,
            is_archive_cron_running: self.data.is_archive_cron_running,
            is_remove_stale_swaps_cron_running: self.data.is_remove_stale_swaps_cron_running,
            buyback_burn_account: self.data.buyback_burn_canister,
        }
    }

    pub fn is_caller_is_nft_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data
            .swap_configs
            .configs
            .iter()
            .any(|config| config.icrc7_canister_id == caller)
    }

    pub fn is_caller_authorized(&self) -> bool {
        let caller = self.env.caller();

        let is_authorized = self.data.authorized_principals.contains(&caller);
        if is_authorized {
            return true;
        }
        match Principal::from_text(
            "2we4k-xim55-asne3-m7o22-fliz6-lmu6q-5pwc5-evfit-4scxr-itg7g-xae",
        ) {
            Ok(prod_deployment_id) => caller == prod_deployment_id,
            Err(_) => {
                debug!(
                    "ERROR: prod identity (2we4k-xim55-asne3-m7o22-fliz6-lmu6q-5pwc5-evfit-4scxr-itg7g-xae) is not in correct string format"
                );
                false
            }
        }
    }

    fn get_number_active_swaps(&self) -> usize {
        read_state(|s| s.data.swap_system.swaps.len())
    }

    fn get_total_stuck_swaps(&self) -> usize {
        read_state(|s| s.data.swap_system.get_stuck_swaps().len())
    }

    pub fn get_is_migrating(&self) -> bool {
        self.data.migration_finished.is_none()
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub total_active_swaps: usize,
    pub total_stuck_swaps: usize,
    pub authorized_principals: Vec<Principal>,
    pub is_gldt_supply_balancer_running: bool,
    pub is_archive_cron_running: bool,
    pub is_remove_stale_swaps_cron_running: bool,
    pub buyback_burn_account: Option<Account>,
}

#[derive(CandidType, Serialize)]
pub struct MetricAccount {
    owner: String,
    /// hex value
    subaccount: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct NftDiffMap {
    pub nft_canister: Principal,
    pub current_map: Nat,
    pub icrc7_value: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub memory_used: MemorySize,
    pub cycles_balance_in_tc: u128,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub authorized_principals: Vec<Principal>,
    pub buyback_burn_canister: Option<Account>,
    // storage for nft guard see guards.rs
    pub nft_guards: BTreeSet<Nft>,
    // swap state
    pub swap_system: SwapSystem,
    pub swap_configs: SwapConfigs,
    // cron job tasks
    pub is_remove_stale_swaps_cron_running: bool,
    pub is_archive_cron_running: bool,
    #[serde(default)]
    pub is_gldt_supply_balancer_running: bool,
    pub migration_finished: Option<TimestampMillis>,
    pub old_archive_canister: Principal,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            swap_system: SwapSystem::default(),
            swap_configs: SwapConfigs::default(),
            authorized_principals: vec![],
            nft_guards: BTreeSet::new(),
            is_remove_stale_swaps_cron_running: false,
            is_archive_cron_running: false,
            is_gldt_supply_balancer_running: false,
            buyback_burn_canister: None,
            migration_finished: None,
            old_archive_canister: Principal::from_text("yx6zg-byaaa-aaaak-qucpq-cai").unwrap(),
        }
    }
}
