use serde::{ Deserialize, Serialize };
use candid::{ CandidType, Principal };
use canister_state_macros::canister_state;
use types::{ BuildVersion, TimestampMillis };
use utils::{ env::{ CanisterEnv, Environment }, memory::MemorySize };

canister_state!(RuntimeState);

#[derive(Default, Serialize, Deserialize)]
pub struct RuntimeState {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: Data,
}

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
            gld_dashbaord_maintenance_mode: self.data.gld_dashbaord_maintenance_mode,
            authorized_principals: self.data.authorized_principals.clone(),
        }
    }

    pub fn is_caller_authorized(&self) -> bool {
        let caller = self.env.caller();
        self.data.authorized_principals.contains(&caller)
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub gld_dashbaord_maintenance_mode: bool,
    pub authorized_principals: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub memory_used: MemorySize,
    pub cycles_balance_in_tc: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    /// Decides if the GLD Dashboard should be put into maintenance mode ( useful for when the backend and frontend need to update )
    pub gld_dashbaord_maintenance_mode: bool,
    pub authorized_principals: Vec<Principal>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            gld_dashbaord_maintenance_mode: false,
            authorized_principals: vec![],
        }
    }
}
