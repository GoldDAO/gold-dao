use crate::model::swap_configs::SwapConfigs;
use crate::model::swap_system::SwapSystem;
use candid::Principal;
use gldt_swap_common::nft::Nft;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use types::TimestampMillis;
use utils::env::CanisterEnv;

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    pub env: CanisterEnv,
    pub data: DataV0,
}

#[derive(Serialize, Deserialize)]
pub struct DataV0 {
    pub authorized_principals: Vec<Principal>,
    pub buyback_burn_canister: Option<Account>,
    pub nft_guards: BTreeSet<Nft>,
    pub swap_system: SwapSystem,
    pub swap_configs: SwapConfigs,
    pub is_remove_stale_swaps_cron_running: bool,
    pub is_archive_cron_running: bool,
    #[serde(default)]
    pub is_gldt_supply_balancer_running: bool,
    pub migration_finished: Option<TimestampMillis>,
    pub old_archive_canister: Principal,
}
