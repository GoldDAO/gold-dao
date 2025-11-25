use std::collections::HashMap;

use bity_ic_canister_time::DAY_IN_MS;
use candid::Nat;
use bity_ic_canister_time::timestamp_millis;
pub use gldt_stake_api_canister::get_apy_overall::{
    Args as GetApyArgs, Response as GetApyResponse,
};
use ic_cdk::query;
use tracing::info;
use types::TimestampMillis;
use types::TokenSymbol;

use crate::state::read_state;

#[query]
fn get_apy_overall(_: GetApyArgs) -> GetApyResponse {
    let (daily_weighted_stake, daily_rewards, token_usd_values) = read_state(|s| {
        let analytics_system = &s.data.analytics_system;
        (
            analytics_system.get_weighted_gldt_staked(0, None),
            analytics_system.get_rewards(0, None),
            analytics_system.token_usd_values.clone(),
        )
    });

    get_apy_impl(daily_weighted_stake, daily_rewards, token_usd_values)
}

fn get_apy_impl(
    daily_weighted_stake: Vec<(TimestampMillis, Nat)>,
    daily_rewards: Vec<(TimestampMillis, HashMap<TokenSymbol, Nat>)>,
    token_usd_values: HashMap<TokenSymbol, f64>,
) -> GetApyResponse {
    let total_weighted_stake = daily_weighted_stake
        .iter()
        .fold(Nat::from(0u64), |acc, (_, n)| acc + n.clone());

    let total_rewards_per_token = calculate_total_rewards_per_token(daily_rewards);
    let total_rewards_usd = sum_usd_rewards(convert_rewards_to_usd(
        total_rewards_per_token,
        &token_usd_values,
    ));

    let weighted_stake_usd = calculate_weighted_stake_usd(total_weighted_stake, &token_usd_values);

    calculate_apy(total_rewards_usd, weighted_stake_usd)
}

fn calculate_total_rewards_per_token(
    daily_rewards: Vec<(TimestampMillis, HashMap<TokenSymbol, Nat>)>,
) -> HashMap<TokenSymbol, Nat> {
    let mut total_rewards: HashMap<TokenSymbol, Nat> = HashMap::new();

    for (_ts, rewards_per_timestamp) in daily_rewards.iter() {
        for (token, amount) in rewards_per_timestamp {
            total_rewards
                .entry(*token)
                .and_modify(|existing| *existing += amount.clone())
                .or_insert(amount.clone());
        }
    }

    total_rewards
}

fn convert_rewards_to_usd(
    daily_rewards: HashMap<TokenSymbol, Nat>,
    token_usd_values: &HashMap<TokenSymbol, f64>,
) -> HashMap<TokenSymbol, f64> {
    let mut usd_rewards = HashMap::new();

    for (token, rewards) in daily_rewards {
        if rewards > 0_u64 {
            let usd_value = token_usd_values.get(&token).unwrap_or(&0.0);
            let daily_rewards_usd = convert_to_usd(rewards, *usd_value);
            usd_rewards.insert(token, daily_rewards_usd);
        } else {
            usd_rewards.insert(token, 0.0);
        }
    }

    usd_rewards
}

pub fn calculate_apy(total_daily_rewards_as_usd: f64, total_weighted_stake_as_usd: f64) -> f64 {
    if total_weighted_stake_as_usd == 0.0 || total_daily_rewards_as_usd == 0.0 {
        info!(
            "APY calculation skipped: total_weighted_stake_as_usd = {}, total_daily_rewards_as_usd = {}",
            total_weighted_stake_as_usd,
            total_daily_rewards_as_usd
        );
        return 0.0;
    }
    ic_cdk::println!(
        "Calculating APY: total_daily_rewards_as_usd = {}, total_weighted_stake_as_usd = {}",
        total_daily_rewards_as_usd,
        total_weighted_stake_as_usd
    );

    let apy = (total_daily_rewards_as_usd / total_weighted_stake_as_usd) * 365.0 * 100.0;
    ic_cdk::println!("Calculated APY: {}", apy);
    apy
}

pub fn calculate_days_since_genesis(genesis_datetime: TimestampMillis) -> u64 {
    let current_time = timestamp_millis();
    if current_time <= genesis_datetime {
        return 0;
    }
    (current_time - genesis_datetime) / DAY_IN_MS
}

pub fn calculate_daily_reward_per_token_in_usd(
    total_token_rewards: HashMap<TokenSymbol, Nat>,
    num_days: u64,
    token_usd_values: &HashMap<TokenSymbol, f64>,
) -> HashMap<TokenSymbol, f64> {
    let mut daily_rewards_per_token = HashMap::new();

    for (token, rewards) in total_token_rewards {
        if rewards > 0_u64 && num_days > 0 {
            // Calculate daily rewards by dividing by the number of days
            let daily_rewards = rewards.0 / Nat::from(num_days).0;
            let usd_value = token_usd_values.get(&token).unwrap_or(&0.0);
            let daily_rewards_usd = convert_to_usd(Nat::from(daily_rewards), *usd_value);
            daily_rewards_per_token.insert(token, daily_rewards_usd);
        } else {
            daily_rewards_per_token.insert(token, 0.0);
        }
    }
    daily_rewards_per_token
}

pub fn sum_usd_rewards(rewards: HashMap<TokenSymbol, f64>) -> f64 {
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

pub fn calculate_weighted_stake_usd(
    tokens: Nat,
    token_usd_values: &HashMap<TokenSymbol, f64>,
) -> f64 {
    let gldt_price = token_usd_values.get(&TokenSymbol::GLDT);
    match gldt_price {
        Some(usd_price) => convert_to_usd(tokens, *usd_price),
        None => 0.0,
    }
}

#[cfg(test)]
mod tests {

    use candid::Nat;
    use std::collections::HashMap;
    use time::{Duration, OffsetDateTime};
    use types::{TimestampMillis, TokenSymbol};

    use super::get_apy_impl;

    #[test]
    fn test_get_apy_impl() {
        // --------------------------------------
        //        basic happy path test
        // --------------------------------------
        let one_day_ago = OffsetDateTime::now_utc()
            .checked_sub(Duration::days(8))
            .unwrap()
            .unix_timestamp() as u64
            * 1000;

        let mut token_prices_usd = HashMap::new();
        token_prices_usd.insert(TokenSymbol::GOLDAO, 1.0);
        token_prices_usd.insert(TokenSymbol::OGY, 1.0);
        token_prices_usd.insert(TokenSymbol::ICP, 1.0);
        token_prices_usd.insert(TokenSymbol::GLDT, 10.0);

        let mut daily_rewards: Vec<(TimestampMillis, HashMap<TokenSymbol, Nat>)> = Vec::new();
        let mut rewards: HashMap<TokenSymbol, Nat> = HashMap::new();
        rewards.insert(TokenSymbol::GOLDAO, Nat::from(400u64));
        rewards.insert(TokenSymbol::OGY, Nat::from(400u64));
        rewards.insert(TokenSymbol::ICP, Nat::from(400u64));
        daily_rewards.push((one_day_ago, rewards));

        let mut dayly_weighted_stake = Vec::new();
        dayly_weighted_stake.push((one_day_ago, Nat::from(100_00u64)));
        // state
        // - with token pricing
        // - 1 days passed
        // - with rewards built up

        // total value of GLDT = 1000 USD
        // total value of rewards = 400 + 400 + 400 = 1200 USD

        // (1200 USD / 100_000 USD) * 365.0 * 100.0 = 438.4;
        assert_eq!(
            get_apy_impl(
                dayly_weighted_stake.clone(),
                daily_rewards.clone(),
                token_prices_usd.clone()
            ),
            438.0
        );

        // second day with no rewards, we expect that the apy should be half
        let now = OffsetDateTime::now_utc().unix_timestamp() as u64 * 1000;

        let mut rewards: HashMap<TokenSymbol, Nat> = HashMap::new();
        rewards.insert(TokenSymbol::GOLDAO, Nat::from(0u64));
        rewards.insert(TokenSymbol::OGY, Nat::from(0u64));
        rewards.insert(TokenSymbol::ICP, Nat::from(0u64));
        daily_rewards.push((now, rewards));

        dayly_weighted_stake.push((now, Nat::from(100_00u64)));

        assert_eq!(
            get_apy_impl(dayly_weighted_stake, daily_rewards, token_prices_usd),
            219.0
        );
    }
}
