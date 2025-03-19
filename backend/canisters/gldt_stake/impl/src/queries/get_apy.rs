use std::collections::HashMap;

use candid::Nat;
use canister_time::{timestamp_millis, WEEK_IN_MS};
pub use gldt_stake_api_canister::get_apy::{Args as GetApyArgs, Response as GetApyResponse};
use ic_cdk::query;
use tracing::info;
use types::TimestampMillis;

use crate::state::read_state;

#[query]
fn get_apy(_: GetApyArgs) -> GetApyResponse {
    let (total_weighted_stake, genesis_datetime, total_rewards_history_per_token, token_usd_values) =
        read_state(|s| {
            let stake_system = &s.data.stake_system;
            let reward_system = &s.data.reward_system;
            (
                stake_system.cached_total_weighted_stake.clone(),
                stake_system.genesis_datetime,
                reward_system.reward_history.clone(),
                stake_system.token_usd_values.clone(),
            )
        });
    get_apy_impl(
        total_weighted_stake,
        genesis_datetime,
        total_rewards_history_per_token,
        token_usd_values,
    )
}

fn get_apy_impl(
    total_weighted_stake: Nat,
    genesis_datetime: TimestampMillis,
    total_rewards_history_per_token: HashMap<String, Nat>,
    token_usd_values: HashMap<String, f64>,
) -> GetApyResponse {
    let weeks_since_genesis = calculate_weeks_since_genesis(genesis_datetime);
    let total_rewards_usd = sum_usd_rewards(calculate_weekly_reward_per_token_in_usd(
        total_rewards_history_per_token,
        weeks_since_genesis,
        &token_usd_values,
    ));

    let weighted_stake_usd = calculate_weighted_stake_usd(total_weighted_stake, &token_usd_values);

    calculate_apy(total_rewards_usd, weighted_stake_usd)
}

fn calculate_apy(total_weekly_rewards_as_usd: f64, total_weighted_stake_as_usd: f64) -> f64 {
    if total_weighted_stake_as_usd == 0.0 || total_weekly_rewards_as_usd == 0.0 {
        info!(
            "APY calculation skipped: total_weighted_stake_as_usd = {}, total_weekly_rewards_as_usd = {}",
            total_weighted_stake_as_usd,
            total_weekly_rewards_as_usd
        );
        return 0.0;
    }

    (total_weekly_rewards_as_usd / total_weighted_stake_as_usd) * 52.0 * 100.0
}

fn calculate_weeks_since_genesis(genesis_datetime: TimestampMillis) -> u64 {
    let current_time = timestamp_millis();
    if current_time <= genesis_datetime {
        return 0;
    }
    (current_time - genesis_datetime) / WEEK_IN_MS
}

fn calculate_weekly_reward_per_token_in_usd(
    total_token_rewards: HashMap<String, Nat>,
    num_weeks: u64,
    token_usd_values: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let mut weekly_rewards_per_token = HashMap::new();

    for (token, rewards) in total_token_rewards {
        if rewards > Nat::from(0u64) && num_weeks > 0 {
            let weekly_rewards = rewards.0 / Nat::from(num_weeks).0;
            let usd_value = token_usd_values.get(&token).unwrap_or(&0.0);
            let weekly_rewards_usd = convert_to_usd(Nat::from(weekly_rewards), *usd_value);
            weekly_rewards_per_token.insert(token, weekly_rewards_usd);
        } else {
            weekly_rewards_per_token.insert(token, 0.0);
        }
    }
    weekly_rewards_per_token
}

fn sum_usd_rewards(rewards: HashMap<String, f64>) -> f64 {
    rewards.into_iter().fold(0.0, |acc, (_, usd)| acc + usd)
}

fn convert_to_usd(tokens: Nat, usd_price: f64) -> f64 {
    const E8S: f64 = 100_000_000.0;

    let tokens_u128: u128 = tokens.0.try_into().unwrap_or_else(|_| {
        info!("Invalid tokens value for conversion");
        0
    });

    if tokens_u128 == 0 || usd_price == 0.0 {
        info!(
            "Invalid conversion inputs: tokens = {}, usd_price = {}",
            tokens_u128, usd_price
        );
        return 0.0;
    }

    let normalized_tokens = tokens_u128 as f64 / E8S;

    normalized_tokens * usd_price
}

fn calculate_weighted_stake_usd(tokens: Nat, token_usd_values: &HashMap<String, f64>) -> f64 {
    let gldt_price = token_usd_values.get("GLDT");
    match gldt_price {
        Some(usd_price) => convert_to_usd(tokens, usd_price.clone()),
        None => 0.0,
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use candid::Nat;
    use time::{Duration, OffsetDateTime};

    use super::get_apy_impl;

    #[test]
    fn test_get_apy_impl() {
        // --------------------------------------
        //        basic happy path test
        // --------------------------------------

        let mut token_prices_usd = HashMap::new();
        token_prices_usd.insert("GOLDAO".to_string(), 1.0);
        token_prices_usd.insert("OGY".to_string(), 1.0);
        token_prices_usd.insert("ICP".to_string(), 1.0);
        token_prices_usd.insert("GLDT".to_string(), 10.0);

        let one_week_ago = OffsetDateTime::now_utc()
            .checked_sub(Duration::days(8))
            .unwrap()
            .unix_timestamp() as u64
            * 1000;

        let mut rewards: HashMap<String, Nat> = HashMap::new();
        rewards.insert("GOLDAO".to_string(), Nat::from(400u64));
        rewards.insert("OGY".to_string(), Nat::from(400u64));
        rewards.insert("ICP".to_string(), Nat::from(400u64));

        let cached_gldt_staked = Nat::from(100_00u64);
        // state
        // - with token pricing
        // - 1 weeks passed
        // - with rewards built up

        // total value of GLDT = 1000 USD
        // total value of rewards = 400 + 400 + 400 = 1200 USD

        // (1200 USD / 100_000 USD) * 52.0 * 100.0 = 62.4;
        assert_eq!(
            get_apy_impl(cached_gldt_staked, one_week_ago, rewards, token_prices_usd),
            62.4
        )

        // --------------------------------------
        //        no weeks have passed
        // --------------------------------------

        // --------------------------------------
        //        no rewards have been allocated
        // --------------------------------------

        // --------------------------------------
        //        pricing of all tokens is 0
        // --------------------------------------
    }
}
