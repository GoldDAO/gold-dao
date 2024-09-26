use gldt_swap_common::{
    archive::format_archive_canisters,
    nft::NftCanisterConf,
    swap::{ ArchiveStatus, ServiceDownReason, ServiceStatus },
};
use icrc_ledger_types::icrc1::account::Account;
use serde::{ Deserialize, Serialize };
use candid::{ CandidType, Nat, Principal };
use canister_state_macros::canister_state;
use types::{ BuildVersion, TimestampMillis };
use utils::{ env::{ CanisterEnv, Environment }, memory::MemorySize };

use crate::model::swaps::Swaps;

canister_state!(RuntimeState);

#[derive(Default, Serialize, Deserialize)]
pub struct RuntimeState {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
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
            total_historic_swaps: format!("{:?}", self.get_total_historic_swaps()),
            total_stuck_swaps: self.get_total_stuck_swaps(),
            archive_canisters: format_archive_canisters(self.data.swaps.get_archive_canisters()),
            service_status: self.data.service_status.clone(),
            required_ogy_threshold: format!("{:?}", self.get_required_ogy_for_canister()),
            ogy_balance: format!("{:?}", self.data.ogy_balance.clone()),
            nft_fee_accounts: format_nft_canister_configs(self.data.gldnft_canisters.clone()),
        }
    }

    pub fn is_caller_is_nft_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.gldnft_canisters.iter().any(|(principal, _, _)| *principal == caller)
    }

    pub fn is_caller_authorized(&self) -> bool {
        let caller = self.env.caller();
        self.data.authorized_principals.contains(&caller)
    }

    fn get_number_active_swaps(&self) -> usize {
        read_state(|s| s.data.swaps.get_active_swaps().len())
    }

    fn get_total_historic_swaps(&self) -> Nat {
        read_state(|s| s.data.swaps.get_history_total())
    }

    fn get_total_stuck_swaps(&self) -> usize {
        read_state(|s| s.data.swaps.get_stuck_swaps().len())
    }

    pub fn set_archive_status(&mut self, status: ArchiveStatus) {
        self.data.archive_status = status;
    }

    pub fn get_archive_stauts(&self) -> ArchiveStatus {
        self.data.archive_status.clone()
    }

    pub fn set_service_status(&mut self, status: ServiceStatus) {
        self.data.service_status = status;
    }

    pub fn get_required_ogy_for_1000_swaps(&self) -> Nat {
        self.data.base_ogy_swap_fee.clone() * Nat::from(1000u64)
    }

    pub fn get_required_ogy_for_canister(&self) -> Nat {
        let nft_canisters = Nat::from(read_state(|s| s.data.gldnft_canisters.len()));
        self.get_required_ogy_for_1000_swaps() * (nft_canisters + Nat::from(1u64))
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub total_active_swaps: usize,
    pub total_historic_swaps: String,
    pub total_stuck_swaps: usize,
    pub archive_canisters: String,
    pub service_status: ServiceStatus,
    pub required_ogy_threshold: String,
    pub ogy_balance: String,
    pub nft_fee_accounts: String,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    pub cycles_balance_in_tc: f64,
    pub version: BuildVersion,
    pub commit_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    /// SNS governance canister
    pub gldt_ledger_id: Principal,
    pub ogy_ledger_id: Principal,
    pub swaps: Swaps,
    pub gldnft_canisters: Vec<(Principal, NftCanisterConf, Option<FeeAccount>)>,
    pub authorized_principals: Vec<Principal>,
    pub is_remove_stale_swaps_cron_running: bool,
    pub is_archive_cron_running: bool,
    pub max_canister_archive_threshold: Nat,
    pub should_upgrade_archives: bool,
    pub ogy_balance: Nat,
    pub archive_status: ArchiveStatus,
    pub service_status: ServiceStatus,
    pub base_ogy_swap_fee: Nat,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            gldt_ledger_id: Principal::anonymous(),
            swaps: Swaps::default(),
            gldnft_canisters: vec![],
            ogy_ledger_id: Principal::anonymous(),
            authorized_principals: vec![],
            is_remove_stale_swaps_cron_running: false,
            is_archive_cron_running: false,
            max_canister_archive_threshold: Nat::from(370 * 1024 * 1024 * (1024 as u128)), // 370GB
            should_upgrade_archives: false,
            ogy_balance: Nat::from(0u64),
            archive_status: ArchiveStatus::Initializing,
            service_status: ServiceStatus::Down(ServiceDownReason::Initializing),
            base_ogy_swap_fee: Nat::from(1_000_000_000u64), // default of 10 OGY
        }
    }
}

pub fn format_nft_canister_configs(
    configs: Vec<(Principal, NftCanisterConf, Option<Account>)>
) -> String {
    let confs: Vec<String> = configs
        .iter()
        .map(|(canister_id, weight, fee_account)| {
            let w = weight.grams;
            let mut owner: String = "".to_string();
            let mut sub: String = "".to_string();
            if let Some(acc) = fee_account {
                owner = acc.owner.to_text();
                if let Some(sub_account) = &acc.subaccount {
                    sub = subaccount_to_hex(sub_account);
                }
            }

            format!(
                "{{canister id : {canister_id}, weight : {w}, fee account : {{ : owner : {owner:?}, sub_account : {sub} }} }}"
            )
        })
        .collect();

    format!("[{}]", confs.join(","))
}

fn subaccount_to_hex(subaccount: &[u8; 32]) -> String {
    subaccount
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}
