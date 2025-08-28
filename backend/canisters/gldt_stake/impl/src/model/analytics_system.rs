use crate::memory::get_daily_analytics_memory;
use crate::memory::VM;
use bity_ic_canister_time::DAY_IN_MS;
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
    // usd price of reward tokens + gldt - used for APY calculations
    pub token_usd_values: HashMap<TokenSymbol, f64>,
}

impl Default for AnalyticsSystem {
    fn default() -> Self {
        let now = timestamp_millis();
        Self {
            last_updated_timestamp: now,
            daily_analytics: init_daily_analytics_history(),
            token_usd_values: HashMap::new(),
        }
    }
}

pub fn init_daily_analytics_history() -> StableBTreeMap<TimestampMillis, DailyAnalytics, VM> {
    let memory = get_daily_analytics_memory();
    StableBTreeMap::init(memory)
}

impl AnalyticsSystem {
    pub fn get_last(&self) -> DailyAnalytics {
        self.daily_analytics
            .last_key_value()
            .map(|(_, analytics)| analytics.clone())
            .unwrap_or_default()
    }

    pub fn insert_daily_analytics(
        &mut self,
        rewards: HashMap<TokenSymbol, Nat>,
        total_staked: Nat,
        total_weighted_stake: Nat,
    ) {
        let now: u64 = timestamp_millis();
        let day_start = now - (now % DAY_IN_MS);

        let updated_analytics = if let Some(existing) = self.daily_analytics.get(&day_start) {
            // Clone existing record
            let mut analytics = existing.clone();

            // Merge rewards
            for (token, amount) in rewards {
                analytics
                    .rewards
                    .entry(token)
                    .and_modify(|v| *v += amount.clone())
                    .or_insert(amount);
            }

            // Recalculate APY
            analytics.apy = calculate_daily_apy(
                total_weighted_stake.clone(),
                analytics.rewards.clone(),
                &self.token_usd_values,
            );

            analytics.staked_gldt = total_staked.clone();
            analytics.weighted_stake = total_weighted_stake.clone();

            analytics
        } else {
            // First record for today
            DailyAnalytics {
                apy: calculate_daily_apy(
                    total_weighted_stake.clone(),
                    rewards.clone(),
                    &self.token_usd_values,
                ),
                staked_gldt: total_staked.clone(),
                weighted_stake: total_weighted_stake.clone(),
                rewards,
            }
        };

        // Insert back (overwrite or insert)
        self.daily_analytics.insert(day_start, updated_analytics);

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

    pub fn set_token_usd_values(&mut self, values: HashMap<TokenSymbol, f64>) {
        self.token_usd_values = values;
    }
}

use crate::calculate_apy;
use crate::calculate_daily_reward_per_token_in_usd;
use crate::calculate_weighted_stake_usd;
use crate::sum_usd_rewards;
use tracing::info;
fn calculate_daily_apy(
    total_weighted_stake: Nat,
    rewards: HashMap<TokenSymbol, Nat>,
    token_usd_values: &HashMap<TokenSymbol, f64>,
) -> f64 {
    let _span = tracing::info_span!("CALCULATE_DAILY_APY").entered();

    info!("start");
    info!(
        "total_weighted_stake = {:?}, rewards size = {}, token_usd_values = {:?}",
        total_weighted_stake,
        rewards.len(),
        token_usd_values
    );

    if !rewards.is_empty() {
        info!("rewards = {:?}", rewards);

        let daily_reward_per_token_usd =
            calculate_daily_reward_per_token_in_usd(rewards.clone(), 1, token_usd_values);

        let total_rewards_usd = sum_usd_rewards(daily_reward_per_token_usd.clone());
        let weighted_stake_usd =
            calculate_weighted_stake_usd(total_weighted_stake.clone(), token_usd_values);

        info!(
            "total_rewards_usd = {}, weighted_stake_usd = {}",
            total_rewards_usd, weighted_stake_usd
        );

        calculate_apy(total_rewards_usd, weighted_stake_usd)
    } else {
        info!("no rewards found");
        0.0
    }
}
