use crate::memory::get_daily_analytics_memory;
use crate::memory::VM;
use candid::Nat;
use canister_time::timestamp_millis;
use gldt_stake_common::daily_analytics::DailyAnalytics;
use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use types::TimestampMillis;
use types::TokenSymbol;

#[derive(Serialize, Deserialize)]
pub struct AnalyticsSystem {
    pub last_updated_timestamp: TimestampMillis,
    #[serde(skip, default = "init_daily_analytics_history")]
    pub daily_analytics: StableBTreeMap<TimestampMillis, DailyAnalytics, VM>,
}

impl Default for AnalyticsSystem {
    fn default() -> Self {
        let now = timestamp_millis();
        Self {
            last_updated_timestamp: now,
            daily_analytics: init_daily_analytics_history(),
        }
    }
}

pub fn init_daily_analytics_history() -> StableBTreeMap<TimestampMillis, DailyAnalytics, VM> {
    let memory = get_daily_analytics_memory();
    StableBTreeMap::init(memory)
}

impl AnalyticsSystem {
    pub fn insert_daily_analytics(
        &mut self,
        apy: f64,
        staked_gldt: Nat,
        weighted_stake: Nat,
        rewards: HashMap<TokenSymbol, Nat>,
    ) {
        let now = timestamp_millis();

        let analytics = DailyAnalytics {
            apy,
            staked_gldt,
            weighted_stake,
            rewards,
        };

        self.daily_analytics.insert(now, analytics);
        self.last_updated_timestamp = now;
    }

    pub fn get_analytics(
        &self,
        starting_day: TimestampMillis,
        limit: Option<usize>,
    ) -> BTreeMap<TimestampMillis, DailyAnalytics> {
        self.daily_analytics
            .range(starting_day..)
            .take(limit.unwrap_or(usize::MAX))
            .map(|(ts, analytics)| (ts, analytics.clone()))
            .collect()
    }

    pub fn get_apys(
        &self,
        starting_day: TimestampMillis,
        limit: Option<usize>,
    ) -> BTreeMap<TimestampMillis, f64> {
        self.daily_analytics
            .range(starting_day..)
            .take(limit.unwrap_or(usize::MAX))
            .map(|(ts, analytics)| (ts, analytics.apy))
            .collect()
    }

    pub fn get_staked_gldt(
        &self,
        starting_day: TimestampMillis,
        limit: Option<usize>,
    ) -> BTreeMap<TimestampMillis, Nat> {
        self.daily_analytics
            .range(starting_day..)
            .take(limit.unwrap_or(usize::MAX))
            .map(|(ts, analytics)| (ts, analytics.staked_gldt.clone()))
            .collect()
    }

    pub fn get_weighted_gldt_staked(
        &self,
        starting_day: TimestampMillis,
        limit: Option<usize>,
    ) -> BTreeMap<TimestampMillis, Nat> {
        self.daily_analytics
            .range(starting_day..)
            .take(limit.unwrap_or(usize::MAX))
            .map(|(ts, analytics)| (ts, analytics.weighted_stake.clone()))
            .collect()
    }

    pub fn get_rewards(
        &self,
        starting_day: TimestampMillis,
        limit: Option<usize>,
    ) -> BTreeMap<TimestampMillis, HashMap<TokenSymbol, Nat>> {
        self.daily_analytics
            .range(starting_day..)
            .take(limit.unwrap_or(usize::MAX))
            .map(|(ts, analytics)| (ts, analytics.rewards.clone()))
            .collect()
    }
}
