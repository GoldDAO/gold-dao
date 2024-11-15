use candid::{CandidType, Decode, Encode, Nat};
use icrc_ledger_types::icrc1::account::Account;
use std::{borrow::Cow, ops::Add};

use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use super_stats_v3_api::account_tree::{HistoryData, Overview as LedgerOverview};
use utils::consts::E8S_PER_ICP;

#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct TokenSupplyData {
    pub total_supply: Nat,
    pub circulating_supply: Nat,
}

impl Storable for TokenSupplyData {
    const BOUND: Bound = Bound::Bounded {
        max_size: 100,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct PrincipalBalance {
    pub governance: GovernanceStats,
    pub ledger: u64,
}

impl Storable for PrincipalBalance {
    const BOUND: Bound = Bound::Bounded {
        max_size: 256,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}
#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct WalletEntry(pub Account, pub WalletOverview);

impl Storable for WalletEntry {
    const BOUND: Bound = Bound::Bounded {
        max_size: 400,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}
#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct GovHistoryEntry(pub u64, pub HistoryData);
impl Storable for GovHistoryEntry {
    const BOUND: Bound = Bound::Bounded {
        max_size: 56,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct WalletOverview {
    pub ledger: LedgerOverview,
    pub governance: GovernanceStats,
    pub total: u64,
}

impl Storable for WalletOverview {
    const BOUND: Bound = Bound::Bounded {
        max_size: 400,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct GovernanceStats {
    pub total_staked: Nat,
    pub total_locked: Nat,
    pub total_unlocked: Nat,
    pub total_rewards: Nat,
}
impl Storable for GovernanceStats {
    const BOUND: Bound = Bound::Bounded {
        max_size: 100,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
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
impl Storable for VotingHistoryCalculations {
    const BOUND: Bound = Bound::Bounded {
        max_size: 40,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
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

impl Storable for DailyVotingMetrics {
    const BOUND: Bound = Bound::Bounded {
        max_size: 32,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
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
