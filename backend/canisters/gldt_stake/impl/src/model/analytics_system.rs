use crate::memory::get_daily_analytics_memory;
use crate::memory::VM;
use bity_ic_canister_time::DAY_IN_MS;
use candid::Nat;
use bity_ic_canister_time::timestamp_millis;
use gldt_stake_common::daily_analytics::DailyAnalytics;
use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};
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
            ic_cdk::println!(
                "Found existing daily analytics for day_start {}: {:?}",
                day_start,
                existing
            );

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

            ic_cdk::println!("Recalculated APY: {}", analytics.apy);

            analytics.staked_gldt = total_staked.clone();
            analytics.weighted_stake = total_weighted_stake.clone();

            analytics
        } else {
            // First record for today
            let apy = DailyAnalytics {
                apy: calculate_daily_apy(
                    total_weighted_stake.clone(),
                    rewards.clone(),
                    &self.token_usd_values,
                ),
                staked_gldt: total_staked.clone(),
                weighted_stake: total_weighted_stake.clone(),
                rewards,
            };
            ic_cdk::println!(
                "No existing record for day_start {}. Creating new: {:?}",
                day_start,
                apy
            );
            apy
        };

        ic_cdk::println!(
            "Inserting/Updating daily analytics for day_start {}: {:?}",
            day_start,
            updated_analytics
        );

        // Insert back (overwrite or insert)
        self.daily_analytics.insert(day_start, updated_analytics);

        self.last_updated_timestamp = now;
    }

    pub fn get_analytics_rev(
        &self,
        starting_day: TimestampMillis,
        limit: Option<usize>,
    ) -> Vec<(TimestampMillis, DailyAnalytics)> {
        self.daily_analytics
            .iter()
            .rev()
            .enumerate()
            .filter(|(day, _)| *day as u64 >= starting_day)
            .take(limit.unwrap_or(usize::MAX))
            .map(|(_, (timestamp, analytics))| (timestamp, analytics))
            .collect()
    }

    pub fn get_apys_rev(
        &self,
        starting_day: TimestampMillis,
        limit: Option<usize>,
    ) -> Vec<(TimestampMillis, f64)> {
        self.daily_analytics
            .iter()
            .rev()
            .enumerate()
            .filter(|(day, _)| *day as u64 >= starting_day)
            .take(limit.unwrap_or(usize::MAX))
            .map(|(_, (timestamp, analytics))| (timestamp, analytics.apy))
            .collect()
    }

    pub fn get_staked_gldt(
        &self,
        starting_day: TimestampMillis,
        limit: Option<usize>,
    ) -> Vec<(TimestampMillis, Nat)> {
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
    ) -> Vec<(TimestampMillis, Nat)> {
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
    ) -> Vec<(TimestampMillis, HashMap<TokenSymbol, Nat>)> {
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

use crate::{
    calculate_apy, calculate_daily_reward_per_token_in_usd, calculate_weighted_stake_usd,
    sum_usd_rewards,
};
use tracing::info;
pub fn calculate_daily_apy(
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
    ic_cdk::println!(
        "total_weighted_stake = {:?}, rewards size = {}, token_usd_values = {:?}",
        total_weighted_stake,
        rewards.len(),
        token_usd_values
    );

    if !rewards.is_empty() {
        info!("rewards = {:?}", rewards);

        let daily_reward_per_token_usd =
            calculate_daily_reward_per_token_in_usd(rewards.clone(), 1, token_usd_values);
        ic_cdk::println!(
            "daily_reward_per_token_usd = {:?}",
            daily_reward_per_token_usd
        );

        let total_rewards_usd = sum_usd_rewards(daily_reward_per_token_usd.clone());
        ic_cdk::println!("total_rewards_usd = {}", total_rewards_usd);
        let weighted_stake_usd =
            calculate_weighted_stake_usd(total_weighted_stake.clone(), token_usd_values);
        ic_cdk::println!("weighted_stake_usd = {}", weighted_stake_usd);
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

#[cfg(test)]
mod tests {

    use super::*;

    impl AnalyticsSystem {
        /// Construct a mock system with `num_days` entries.
        /// - APY increases by `0.01` each day.
        /// - Weighted stake increases by `100` each day.
        /// - Timestamp increments by `DAY_IN_MS`.
        pub fn mock(num_days: usize, start_ts: TimestampMillis) -> Self {
            let mut system = AnalyticsSystem::default();
            for i in 0..num_days {
                let ts = start_ts + i as u64 * DAY_IN_MS;
                system.daily_analytics.insert(
                    ts,
                    DailyAnalytics {
                        apy: 0.01 * (i as f64 + 1.0),
                        staked_gldt: Nat::from((i + 1) as u64 * 10),
                        weighted_stake: Nat::from((i + 1) as u64 * 100),
                        rewards: HashMap::new(),
                    },
                );
            }
            system
        }
    }

    #[test]
    fn test_get_apys() {
        let system = AnalyticsSystem::mock(10, 1756339200000);

        // Case 1: No limit, starting from 0 → should return all in reverse (latest first)
        let apys = system.get_apys_rev(0, None);
        assert_eq!(
            apys,
            Vec::from([
                (1757116800000, 0.10),
                (1757030400000, 0.09),
                (1756944000000, 0.08),
                (1756857600000, 0.07),
                (1756771200000, 0.06),
                (1756684800000, 0.05),
                (1756598400000, 0.04),
                (1756512000000, 0.03),
                (1756425600000, 0.02),
                (1756339200000, 0.01),
            ])
        );

        // Case 2: Limit = 2 → should return the last 2 (3000 and 2000)
        let apys = system.get_apys_rev(0, Some(2));
        assert_eq!(
            apys,
            Vec::from([(1757116800000, 0.10), (1757030400000, 0.09),])
        );

        // Case 3: Starting day = 2500 → should only return 3000
        let apys = system.get_apys_rev(0, Some(1));
        assert_eq!(apys, Vec::from([(1757116800000, 0.10)]));

        // Case 4: Starting day > latest → should return empty
        let apys = system.get_apys_rev(4000, None);
        assert!(apys.is_empty());
    }

    #[test]
    fn test_get_analytics_rev() {
        let system = AnalyticsSystem::mock(10, 1756339200000);

        // Case 1: No limit, starting from 0 → should return all
        let analytics = system.get_analytics_rev(0, None);
        let keys: Vec<u64> = analytics.iter().map(|(ts, _)| *ts).collect();
        assert_eq!(
            keys,
            vec![
                1757116800000,
                1757030400000,
                1756944000000,
                1756857600000,
                1756771200000,
                1756684800000,
                1756598400000,
                1756512000000,
                1756425600000,
                1756339200000,
            ]
        );

        assert_eq!(analytics[9].1.apy, 0.01); // earliest
        assert_eq!(analytics[5].1.apy, 0.05);
        assert_eq!(analytics[0].1.apy, 0.10); // latest
        assert_eq!(analytics[0].1.weighted_stake, Nat::from(1000u64));
        println!("analytics {:?}", analytics);

        // Case 2: Limit = 2 → should return the last 2 (latest first)
        let analytics = system.get_analytics_rev(0, Some(2));
        assert_eq!(analytics.len(), 2);
        assert_eq!(analytics[1].1.apy, 0.09);
        assert_eq!(analytics[0].1.apy, 0.10);
        println!("analytics {:?}", analytics);

        // Case 3: Limit = 1 → should only return the very latest
        let analytics = system.get_analytics_rev(0, Some(1));
        assert_eq!(analytics.len(), 1);
        assert_eq!(analytics[0].1.apy, 0.10);

        // Case 4: Too big starting day
        let analytics = system.get_analytics_rev(20, None);
        assert!(analytics.is_empty());
    }

    #[test]
    fn test_empty_rewards_returns_zero() {
        let rewards: HashMap<TokenSymbol, Nat> = HashMap::new();
        let prices: HashMap<TokenSymbol, f64> = HashMap::new();
        let apy = calculate_daily_apy(Nat::from(100u64), rewards, &prices);
        assert_eq!(apy, 0.0);
    }

    #[test]
    fn test_single_token_simple_case() {
        let mut rewards = HashMap::new();
        let mut prices = HashMap::new();

        let gldt = TokenSymbol::GLDT;
        let goldao = TokenSymbol::GOLDAO;
        rewards.insert(goldao.clone(), Nat::from(5_000_000_000_u64));
        prices.insert(gldt.clone(), 1.12940407616124);
        prices.insert(goldao.clone(), 0.01839565874165977);

        let apy = calculate_daily_apy(Nat::from(178_585_860_745_u64), rewards, &prices);
        assert_eq!(apy, 16.644923021770992);
    }

    #[test]
    fn test_all_rewards_calculation() {
        let mut rewards = HashMap::new();
        let mut prices = HashMap::new();

        // token_usd_values for all tokens
        prices.insert(TokenSymbol::OGY, 0.0020897925827264275);
        prices.insert(TokenSymbol::WTN, 0.12425300335520276);
        prices.insert(TokenSymbol::GLDT, 1.12940407616124);
        prices.insert(TokenSymbol::ICP, 4.903953021499477);
        prices.insert(TokenSymbol::GOLDAO, 0.01839565874165977);

        // realistic rewards (exclude GLDT as per business logic)
        rewards.insert(TokenSymbol::OGY, Nat::from(10_000_000_000u64)); // 20897925.8273 USD
        rewards.insert(TokenSymbol::WTN, Nat::from(2_500_000u64)); // 310632.508388 USD
        rewards.insert(TokenSymbol::GOLDAO, Nat::from(5_000_000_000u64)); // 91978293.7083 USD
        rewards.insert(TokenSymbol::ICP, Nat::from(1_000_000u64)); // 4903953.0215 USD

        // weighted stake
        let total_weighted_stake = Nat::from(178_585_860_745u64); // 201695599070 USD

        let apy = calculate_daily_apy(total_weighted_stake, rewards, &prices);

        // Total rewards - 118090805.065 USD
        // daily rewards 0.00058549024
        // APY - 0.2137039376 * 100
        assert_eq!(apy, 21.370393824951332);
    }

    #[test]
    fn test_zero_stake_returns_zero() {
        let mut rewards = HashMap::new();
        let mut prices = HashMap::new();

        let gldt = TokenSymbol::GLDT;
        rewards.insert(gldt.clone(), Nat::from(10u64));
        prices.insert(gldt.clone(), 2.0);

        let apy = calculate_daily_apy(Nat::from(0u64), rewards, &prices);

        assert_eq!(apy, 0.0);
    }
}
