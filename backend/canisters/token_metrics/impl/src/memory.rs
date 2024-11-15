use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap as StableBTreeMap, DefaultMemoryImpl, Vec as StableVec,
};
use sns_governance_canister::types::{Account, VecNeurons};
use token_metrics_api::token_data::{
    DailyVotingMetrics, GovHistoryEntry, GovernanceStats, PrincipalBalance,
    VotingHistoryCalculations, WalletEntry, WalletOverview,
};

const UPGRADES: MemoryId = MemoryId::new(0);
pub const PRINCIPAL_GOV_STATS_MEMORY: MemoryId = MemoryId::new(1);
pub const PRINCIPAL_NEURONS_MEMORY: MemoryId = MemoryId::new(2);
pub const BALANCE_LIST_MEMORY: MemoryId = MemoryId::new(3);
pub const WALLET_LIST_MEMORY: MemoryId = MemoryId::new(4);
pub const DAILY_VOTING_METRICS_MEMORY: MemoryId = MemoryId::new(5);
pub const VOTING_PARTICIPATION_HISTORY_MEMORY: MemoryId = MemoryId::new(6);
pub const VOTING_PARTICIPATION_HISTORY_CALCULATIONS_MEMORY: MemoryId = MemoryId::new(7);
pub const MERGED_WALLET_LIST_MEMORY: MemoryId = MemoryId::new(8);
pub const GOV_STAKE_HISTORY_MEMORY: MemoryId = MemoryId::new(9);

pub type VM = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: MemoryManager<DefaultMemoryImpl> = MemoryManager::init(
        DefaultMemoryImpl::default()
    );
}

pub fn get_upgrades_memory() -> VM {
    get_memory(UPGRADES)
}

fn get_memory(id: MemoryId) -> VM {
    MEMORY_MANAGER.with(|m| m.get(id))
}

pub fn init_pricipal_gov_stats() -> StableBTreeMap<Principal, GovernanceStats, VM> {
    let memory = get_memory(PRINCIPAL_GOV_STATS_MEMORY);
    StableBTreeMap::init(memory)
}
pub fn init_principal_neurons() -> StableBTreeMap<Principal, VecNeurons, VM> {
    let memory = get_memory(PRINCIPAL_NEURONS_MEMORY);
    StableBTreeMap::init(memory)
}
pub fn init_balance_list() -> StableBTreeMap<Principal, PrincipalBalance, VM> {
    let memory = get_memory(BALANCE_LIST_MEMORY);
    StableBTreeMap::init(memory)
}
pub fn init_wallet_list() -> StableVec<WalletEntry, VM> {
    let memory = get_memory(WALLET_LIST_MEMORY);
    StableVec::init(memory).unwrap()
}
pub fn init_daily_voting_metrics() -> StableBTreeMap<u64, DailyVotingMetrics, VM> {
    let memory = get_memory(DAILY_VOTING_METRICS_MEMORY);
    StableBTreeMap::init(memory)
}
pub fn init_voting_participation_history() -> StableBTreeMap<u64, u64, VM> {
    let memory = get_memory(VOTING_PARTICIPATION_HISTORY_MEMORY);
    StableBTreeMap::init(memory)
}
pub fn init_voting_participation_history_calculations(
) -> StableBTreeMap<u64, VotingHistoryCalculations, VM> {
    let memory = get_memory(VOTING_PARTICIPATION_HISTORY_CALCULATIONS_MEMORY);
    StableBTreeMap::init(memory)
}
pub fn init_merged_wallet_list() -> StableVec<WalletEntry, VM> {
    let memory = get_memory(MERGED_WALLET_LIST_MEMORY);
    StableVec::init(memory).unwrap()
}
pub fn init_gov_stake_history() -> StableVec<GovHistoryEntry, VM> {
    let memory = get_memory(GOV_STAKE_HISTORY_MEMORY);
    StableVec::init(memory).unwrap()
}
