use crate::model::allocated_rewards_pool::calculate_total_weighted_stake;
use crate::model::analytics_system::calculate_daily_apy;
use crate::model::unallocated_rewards_pool::{UnallocatedRewards, UnallocatedRewardsPool};
use crate::state::{mutate_state, read_state};
use candid::Nat;
use futures::future::join_all;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use std::collections::{BTreeSet, HashMap};
use tracing::{error, info, warn};
use types::TokenSymbol;
use utils::numeric::{Percentage, ScaledArithmetic};

pub async fn process_rewards_impl() -> Result<(), BTreeSet<TokenSymbol>> {
    let _span = tracing::info_span!("PROCESS_REWARDS").entered();
    info!("start");

    let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());
    let unallocated_rewards_pool = read_state(|s| s.data.unallocated_rewards_pool.clone());
    let apy_limit_opt = read_state(|s| s.data.stake_system.apy_limit); // now Option<Percentage>

    let results = if let Some(apy_limit) = apy_limit_opt {
        batch_rewards_transfer_with_limit(apy_limit, reward_types, unallocated_rewards_pool).await
    } else {
        batch_rewards_transfer_unlimited(reward_types, unallocated_rewards_pool).await
    };
    let mut errors = BTreeSet::new();

    for (token_symbol, transfer_result) in results {
        match transfer_result {
            Ok(rewards_to_allocate) => {
                info!(
                    "transfer successful for token: {}, rewards to allocate: {}",
                    token_symbol, rewards_to_allocate
                );
            }
            Err(err) => match &err {
                GeneralError::CallError(e) | GeneralError::TransferError(e) => {
                    error!("transfer failed for token: {}, error: {}", token_symbol, e);
                    errors.insert(token_symbol);
                }
                _ => warn!(
                    "ignoring non-transfer error for token: {}, error: {:?}",
                    token_symbol, err
                ),
            },
        }
    }

    info!("finished");

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub async fn batch_rewards_transfer_with_limit(
    apy_limit: Percentage,
    reward_types: BTreeSet<TokenSymbol>,
    unallocated_rewards_pool: UnallocatedRewardsPool,
) -> Vec<(TokenSymbol, Result<Nat, GeneralError>)> {
    mutate_state(|s| {
        s.data.unallocated_rewards_pool.transition_to_transferring();
    });

    let mut transfer_futures = Vec::new();
    let mut token_symbols = Vec::new();

    let token_usd_values = read_state(|s| s.data.analytics_system.token_usd_values.clone());
    let stake_positions = read_state(|s| s.data.stake_system.get_reward_eligible_stake_positions());

    let total_weighted_stake = calculate_total_weighted_stake(&stake_positions);

    let mut rewards: HashMap<TokenSymbol, Nat> = HashMap::new();
    for reward_type in &reward_types {
        let token_ledger = reward_type.get_token_info().ledger_id;
        let pool_balance = match unallocated_rewards_pool.balance(token_ledger).await {
            Ok(bal) => bal,
            Err(e) => {
                error!("Error fetching balance for {:?}: {:?}", reward_type, e);
                Nat::from(0u64)
            }
        };
        rewards.insert(*reward_type, pool_balance);
    }

    let current_apy = calculate_daily_apy(
        total_weighted_stake.clone(),
        rewards.clone(),
        &token_usd_values,
    ); // in fractional way, e.g. 0.32 = 32%

    // --- Cancel if APY is zero or scaling_factor <= 0 ---
    if current_apy <= 0.0 {
        mutate_state(|s| s.data.unallocated_rewards_pool.transition_to_awaiting());
        return reward_types
            .into_iter()
            .map(|r| {
                (
                    r,
                    Err(GeneralError::ZeroAPY(
                        "Current APY is zero or invalid. Rewards processing cancelled.".to_string(),
                    )),
                )
            })
            .collect();
    }

    let scaling_factor = apy_limit.as_float() / current_apy;

    if scaling_factor <= 0.0 {
        mutate_state(|s| s.data.unallocated_rewards_pool.transition_to_awaiting());
        return reward_types
            .into_iter()
            .map(|r| {
                (
                    r,
                    Err(GeneralError::ZeroAPY(
                        "Scaling factor <= 0. No rewards to distribute.".to_string(),
                    )),
                )
            })
            .collect();
    }

    // --- Proceed normally if scaling_factor > 0 ---
    for reward_type in reward_types {
        let token_ledger = reward_type.get_token_info().ledger_id;
        let ledger_fee = reward_type.get_token_info().fee;

        let pool_balance = match unallocated_rewards_pool.balance(token_ledger).await {
            Ok(bal) => bal,
            Err(e) => {
                error!("Error fetching balance for {:?}: {:?}", reward_type, e);
                Nat::from(0u64)
            }
        };

        let scaled_amount = pool_balance
            .scale_e8s_mul_f64(scaling_factor)
            .scale_e8s_down();

        let apy_limit_amount = Some(scaled_amount);

        let future = unallocated_rewards_pool.transfer_part_of_rewards(
            token_ledger,
            Nat::from(ledger_fee),
            apy_limit_amount,
        );
        transfer_futures.push(future);
        token_symbols.push(reward_type);
    }

    let results = join_all(transfer_futures).await;

    mutate_state(|s| {
        s.data.unallocated_rewards_pool.transition_to_awaiting();
    });

    token_symbols.into_iter().zip(results.into_iter()).collect()
}

pub async fn batch_rewards_transfer_unlimited(
    reward_types: BTreeSet<TokenSymbol>,
    unallocated_rewards_pool: UnallocatedRewardsPool,
) -> Vec<(TokenSymbol, Result<Nat, GeneralError>)> {
    mutate_state(|s| {
        s.data.unallocated_rewards_pool.transition_to_transferring();
    });
    let mut transfer_futures = Vec::new();
    let mut token_symbols = Vec::new();

    for reward_type in reward_types {
        let token_ledger = reward_type.get_token_info().ledger_id;
        let ledger_fee = reward_type.get_token_info().fee;

        let future = unallocated_rewards_pool.transfer_part_of_rewards(
            token_ledger,
            Nat::from(ledger_fee),
            None,
        );
        transfer_futures.push(future);
        token_symbols.push(reward_type);
    }

    let results = join_all(transfer_futures).await;

    mutate_state(|s| {
        s.data.unallocated_rewards_pool.transition_to_awaiting();
    });

    token_symbols.into_iter().zip(results.into_iter()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Nat;

    const SCALE_FACTOR: u128 = 100_000_000_000_000; // 1e14 precision

    #[test]
    fn compare_float_vs_scaled_nat_multiplication() {
        // Example pool balance (e.g. 100_000_000 tokens)
        let pool_balance = Nat::from(123_456_000_000_000_u128);

        // Example scaling factor
        let scaling_factor_f64 = 0.31234542345091287203948756_f64;

        // --- (1) Float-based approach ---
        let scaled_amount_float = pool_balance
            .scale_e8s_mul_f64(scaling_factor_f64)
            .scale_e8s_down();

        // --- (2) Fixed-point scaled integer approach ---
        // Scale factor converted to integer
        let nat_as_u128 = u128::try_from(pool_balance.0.clone()).unwrap_or_default();
        let scaled_factor_int = (scaling_factor_f64 * SCALE_FACTOR as f64).round() as u128;

        // Multiply purely in integer domain, then scale down
        let scaled_amount_fixed = Nat::from((nat_as_u128 * scaled_factor_int) / SCALE_FACTOR);

        println!("pool_balance: {}", pool_balance);
        println!("scaling_factor_f64: {}", scaling_factor_f64);
        println!("scaled_factor_int: {}", scaled_factor_int);
        println!("scaled_amount_float: {}", scaled_amount_float);
        println!("scaled_amount_fixed: {}", scaled_amount_fixed);
    }

    #[test]
    fn compare_float_vs_scaled_nat_multiplication_roundup() {
        // Example pool balance (e.g. 100_000_000 tokens)
        let pool_balance = Nat::from(100_000_000_000_000_u128);

        // Example scaling factor
        let scaling_factor_f64 = 0.3123456789_f64;

        // --- (1) Float-based approach ---
        let scaled_amount_float = pool_balance
            .scale_e8s_mul_f64(scaling_factor_f64)
            .scale_e8s_down();

        // --- (2) Fixed-point scaled integer approach ---
        // Scale factor converted to integer
        let nat_as_u128 = u128::try_from(pool_balance.0.clone()).unwrap_or_default();
        let scaled_factor_int = (scaling_factor_f64 * SCALE_FACTOR as f64).round() as u128;

        // Multiply purely in integer domain, then scale down
        let scaled_amount_fixed = Nat::from((nat_as_u128 * scaled_factor_int) / SCALE_FACTOR);

        println!("pool_balance: {}", pool_balance);
        println!("scaling_factor_f64: {}", scaling_factor_f64);
        println!("scaled_factor_int: {}", scaled_factor_int);
        println!("scaled_amount_float: {}", scaled_amount_float);
        println!("scaled_amount_fixed: {}", scaled_amount_fixed);
    }
}
