use std::ops::Add;

use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};
use super_stats_v3_api::account_tree::Overview as LedgerOverview;
use utils::consts::E8S_PER_ICP;

#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct TokenSupplyData {
    pub total_supply: Nat,
    pub circulating_supply: Nat,
}

#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct PrincipalBalance {
    pub governance: GovernanceStats,
    pub ledger: u64,
}
#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct WalletOverview {
    pub ledger: LedgerOverview,
    pub governance: GovernanceStats,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct GovernanceStats {
    pub total_staked: Nat,
    pub total_locked: Nat,
    pub total_unlocked: Nat,
    pub total_rewards: Nat,
}
#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct LockedNeuronsAmount {
    pub one_year: u64,
    pub two_years: u64,
    pub three_years: u64,
    pub four_years: u64,
    pub five_years: u64,
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ProposalsMetricsCalculations {
    pub cumulative_voting_participation: f64,
    pub cumulative_voting_power: u64,
    pub valid_tally_count: u64,
}
#[derive(Serialize, Deserialize, Clone, Default, CandidType, Debug)]
pub struct VotingHistoryCalculations {
    pub cumulative_voting_participation: f64,
    pub valid_tally_count: u64,
}

#[derive(Serialize, Deserialize, Clone, CandidType, Debug)]
pub struct ProposalsMetrics {
    pub total_proposals: u64,
    pub daily_voting_rewards: u64,
    pub reward_base_current_year: u64,
    pub total_voting_power: u64,
    pub average_voting_power: u64,
    pub average_voting_participation: u64,
}
impl Default for ProposalsMetrics {
    fn default() -> Self {
        ProposalsMetrics {
            total_proposals: 0,
            // This needs a new value for GoldGov
            reward_base_current_year: 250_000_000 * E8S_PER_ICP,
            daily_voting_rewards: (250_000_000 / 365) * E8S_PER_ICP,
            total_voting_power: 0,
            average_voting_power: 0,
            average_voting_participation: 0u64,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct DailyVotingMetrics {
    pub org_voting_power: u64,
    pub total_voting_power: u64,
    pub voting_participation: u64,
}
impl Add for GovernanceStats {
    type Output = GovernanceStats;

    fn add(self, other: Self) -> Self::Output {
        GovernanceStats {
            total_staked: self.total_staked + other.total_staked,
            total_locked: self.total_locked + other.total_locked,
            total_unlocked: self.total_unlocked + other.total_unlocked,
            total_rewards: self.total_rewards + other.total_rewards,
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct GetHoldersArgs {
    pub offset: u64,
    pub limit: u64,
    pub merge_accounts_to_principals: bool,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Default)]
pub struct ActiveUsers {
    pub active_accounts_count: usize,
    pub active_principals_count: usize,
}
