use bity_ic_icrc3::config::ICRC3Config;
use candid::{CandidType, Principal};
use gldt_swap_common::swap_canister_config::SwapCanisterConfig;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use bity_ic_types::BuildVersion;

#[derive(Deserialize, Serialize, CandidType, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub swap_configs: Vec<SwapCanisterConfig>,
    pub authorized_principals: Vec<Principal>,
    pub buyback_burn_canister: Option<Account>,
    pub gldt_swap_old_archive: Principal,
    pub icrc3_config: ICRC3Config,
}
