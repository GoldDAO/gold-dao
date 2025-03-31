// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};

#[derive(CandidType, Deserialize)]
pub struct Timers {
    pub last_spawned_timestamp_seconds: Option<u64>,
    pub last_reset_timestamp_seconds: Option<u64>,
    pub requires_periodic_tasks: Option<bool>,
}

#[derive(CandidType, Deserialize)]
pub struct SnsRootCanister {
    pub dapp_canister_ids: Vec<Principal>,
    pub timers: Option<Timers>,
    pub testflight: bool,
    pub archive_canister_ids: Vec<Principal>,
    pub governance_canister_id: Option<Principal>,
    pub index_canister_id: Option<Principal>,
    pub swap_canister_id: Option<Principal>,
    pub ledger_canister_id: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterIdRecord {
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterStatusType {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "running")]
    Running,
}

#[derive(CandidType, Deserialize)]
pub enum LogVisibility {
    #[serde(rename = "controllers")]
    Controllers,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "allowed_viewers")]
    AllowedViewers(Vec<Principal>),
}

#[derive(CandidType, Deserialize)]
pub struct DefiniteCanisterSettings {
    pub freezing_threshold: Option<candid::Nat>,
    pub wasm_memory_threshold: Option<candid::Nat>,
    pub controllers: Vec<Principal>,
    pub reserved_cycles_limit: Option<candid::Nat>,
    pub log_visibility: Option<LogVisibility>,
    pub wasm_memory_limit: Option<candid::Nat>,
    pub memory_allocation: Option<candid::Nat>,
    pub compute_allocation: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub struct QueryStats {
    pub response_payload_bytes_total: Option<candid::Nat>,
    pub num_instructions_total: Option<candid::Nat>,
    pub num_calls_total: Option<candid::Nat>,
    pub request_payload_bytes_total: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterStatusResult {
    pub status: CanisterStatusType,
    pub memory_size: candid::Nat,
    pub cycles: candid::Nat,
    pub settings: DefiniteCanisterSettings,
    pub query_stats: Option<QueryStats>,
    pub idle_cycles_burned_per_day: Option<candid::Nat>,
    pub module_hash: Option<serde_bytes::ByteBuf>,
    pub reserved_cycles: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterInstallMode {
    #[serde(rename = "reinstall")]
    Reinstall,
    #[serde(rename = "upgrade")]
    Upgrade,
    #[serde(rename = "install")]
    Install,
}

#[derive(CandidType, Deserialize)]
pub struct ChunkedCanisterWasm {
    pub wasm_module_hash: serde_bytes::ByteBuf,
    pub chunk_hashes_list: Vec<serde_bytes::ByteBuf>,
    pub store_canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct ChangeCanisterRequest {
    pub arg: serde_bytes::ByteBuf,
    pub wasm_module: serde_bytes::ByteBuf,
    pub stop_before_installing: bool,
    pub mode: CanisterInstallMode,
    pub canister_id: Principal,
    pub chunked_canister_wasm: Option<ChunkedCanisterWasm>,
    pub memory_allocation: Option<candid::Nat>,
    pub compute_allocation: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub struct GetSnsCanistersSummaryRequest {
    pub update_canister_list: Option<bool>,
}

#[derive(CandidType, Deserialize)]
pub struct DefiniteCanisterSettingsArgs {
    pub freezing_threshold: candid::Nat,
    pub wasm_memory_threshold: Option<candid::Nat>,
    pub controllers: Vec<Principal>,
    pub wasm_memory_limit: Option<candid::Nat>,
    pub memory_allocation: candid::Nat,
    pub compute_allocation: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterStatusResultV2 {
    pub status: CanisterStatusType,
    pub memory_size: candid::Nat,
    pub cycles: candid::Nat,
    pub settings: DefiniteCanisterSettingsArgs,
    pub query_stats: Option<QueryStats>,
    pub idle_cycles_burned_per_day: candid::Nat,
    pub module_hash: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterSummary {
    pub status: Option<CanisterStatusResultV2>,
    pub canister_id: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct GetSnsCanistersSummaryResponse {
    pub root: Option<CanisterSummary>,
    pub swap: Option<CanisterSummary>,
    pub ledger: Option<CanisterSummary>,
    pub index: Option<CanisterSummary>,
    pub governance: Option<CanisterSummary>,
    pub dapps: Vec<CanisterSummary>,
    pub archives: Vec<CanisterSummary>,
}

#[derive(CandidType, Deserialize)]
pub struct GetTimersArg {}

#[derive(CandidType, Deserialize)]
pub struct GetTimersResponse {
    pub timers: Option<Timers>,
}

#[derive(CandidType, Deserialize)]
pub struct ListSnsCanistersArg {}

#[derive(CandidType, Deserialize)]
pub struct ListSnsCanistersResponse {
    pub root: Option<Principal>,
    pub swap: Option<Principal>,
    pub ledger: Option<Principal>,
    pub index: Option<Principal>,
    pub governance: Option<Principal>,
    pub dapps: Vec<Principal>,
    pub archives: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct ManageDappCanisterSettingsRequest {
    pub freezing_threshold: Option<u64>,
    pub wasm_memory_threshold: Option<u64>,
    pub canister_ids: Vec<Principal>,
    pub reserved_cycles_limit: Option<u64>,
    pub log_visibility: Option<i32>,
    pub wasm_memory_limit: Option<u64>,
    pub memory_allocation: Option<u64>,
    pub compute_allocation: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct ManageDappCanisterSettingsResponse {
    pub failure_reason: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterDappCanisterRequest {
    pub canister_id: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterDappCanisterRet {}

#[derive(CandidType, Deserialize)]
pub struct RegisterDappCanistersRequest {
    pub canister_ids: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterDappCanistersRet {}

#[derive(CandidType, Deserialize)]
pub struct ResetTimersArg {}

#[derive(CandidType, Deserialize)]
pub struct ResetTimersRet {}

#[derive(CandidType, Deserialize)]
pub struct SetDappControllersRequest {
    pub canister_ids: Option<RegisterDappCanistersRequest>,
    pub controller_principal_ids: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterCallError {
    pub code: Option<i32>,
    pub description: String,
}

#[derive(CandidType, Deserialize)]
pub struct FailedUpdate {
    pub err: Option<CanisterCallError>,
    pub dapp_canister_id: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct SetDappControllersResponse {
    pub failed_updates: Vec<FailedUpdate>,
}
