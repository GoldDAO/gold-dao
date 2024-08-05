use candid::{ CandidType, Nat, Principal };
use canister_state_macros::canister_state;
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;
use std::collections::BTreeMap;
use types::{ CanisterId, TimestampMillis };
use utils::{
    consts::{ SNS_GOVERNANCE_CANISTER_ID, SNS_LEDGER_CANISTER_ID },
    env::{ CanisterEnv, Environment },
    memory::MemorySize,
};

canister_state!(RuntimeState);

#[derive(Serialize, Deserialize)]
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
            },
            sync_info: self.data.sync_info,
            sns_governance_canister: self.data.sns_governance_canister,
            sns_ledger_canister: self.data.sns_ledger_canister,
        }
    }
}
#[derive(CandidType, Deserialize, Serialize, Clone, Copy, Default)]
pub struct SyncInfo {
    pub last_synced_start: TimestampMillis,
    pub last_synced_end: TimestampMillis,
    pub last_synced_number_of_neurons: usize,
    pub last_synced_transaction: usize,
}
#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub sns_governance_canister: Principal,
    pub sns_ledger_canister: Principal,
    pub sync_info: SyncInfo,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    pub cycles_balance_in_tc: f64,
}
#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct GovernanceStats {
    pub total_staked: Nat,
    pub total_locked: Nat,
    pub total_unlocked: Nat,
    pub total_rewards: Nat,
}
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub gold_price: f64,
    pub gold_nft_canisters: Vec<(Principal, u128)>,
    pub total_gold_grams: u128,
    pub all_gov_stats: GovernanceStats,
    /// SNS governance cansiter
    pub sns_governance_canister: Principal,
    /// SNS ledger canister
    pub sns_ledger_canister: Principal,
    /// Information about governance neurons sync
    pub sync_info: SyncInfo,
    /// Stores governance stats by principal
    pub principal_gov_stats: BTreeMap<Principal, GovernanceStats>,
    /// Token supply data, such as total supply and circulating supply
    pub supply_data: TokenSupplyData,
    /// Stores the mapping of each principal to its neurons
    pub principal_neurons: BTreeMap<Principal, Vec<NeuronId>>,
    // The cansiter id of the sns rewards cansiter
    pub sns_rewards_canister_id: CanisterId,
}

#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct PrincipalBalance {
    pub governance: GovernanceStats,
    pub ledger: u64,
}
#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct TokenSupplyData {
    pub total_supply: Nat,
    pub circulating_supply: Nat,
}
impl Data {
    pub fn new(
        gold_nft_canisters: Vec<(Principal, u128)>,
        sns_rewards_canister_id: CanisterId
    ) -> Self {
        Self {
            sns_governance_canister: SNS_GOVERNANCE_CANISTER_ID,
            sns_ledger_canister: SNS_LEDGER_CANISTER_ID,
            sns_rewards_canister_id,
            gold_price: 0.0,
            gold_nft_canisters: gold_nft_canisters,
            total_gold_grams: 0,
            principal_neurons: BTreeMap::new(),
            principal_gov_stats: BTreeMap::new(),
            all_gov_stats: GovernanceStats::default(),
            supply_data: TokenSupplyData::default(),
            sync_info: SyncInfo::default(),
        }
    }
}
