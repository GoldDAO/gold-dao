use buyback_burn_api::swap_config::SwapConfig;
use candid::Nat;
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;
use tracing::{ debug, error };
use crate::types::ICPSwapClient;

pub const RETRY_DELAY: Duration = Duration::from_secs(5 * 60); // each 5 minutes

use buyback_burn_api::swap_config::ExchangeConfig;
use crate::types::SwapClient;
pub fn build_swap_client(config: SwapConfig) -> Box<dyn SwapClient> {
    let input_token = config.input_token;
    let output_token = config.output_token;

    match config.exchange_config {
        ExchangeConfig::ICPSwap(icpswap) => {
            let (token0, token1) = if icpswap.zero_for_one {
                (input_token, output_token)
            } else {
                (output_token, input_token)
            };

            Box::new(
                ICPSwapClient::new(
                    config.swap_client_id,
                    ic_cdk::api::id(),
                    icpswap.swap_canister_id,
                    token0,
                    token1,
                    icpswap.zero_for_one
                )
            )
        }
    }
}

pub async fn get_token_balance(ledger_id: Principal) -> Result<Nat, String> {
    icrc_ledger_canister_c2c_client
        ::icrc1_balance_of(
            ledger_id,
            &(Account {
                owner: ic_cdk::api::id(),
                subaccount: None,
            })
        ).await
        .map_err(|e| format!("Failed to fetch token balance: {:?}", e))
}

// TODO: think on how to add delay here
pub async fn retry_with_attempts<F, Fut>(
    max_attempts: u8,
    _delay_duration: Duration,
    mut f: F
)
    -> Result<(), String>
    where F: FnMut() -> Fut, Fut: std::future::Future<Output = Result<(), String>>
{
    for attempt in 1..=max_attempts {
        match f().await {
            Ok(_) => {
                return Ok(());
            }
            Err(err) => {
                error!("Attempt {}: Error - {:?}", attempt, err);
                if attempt == max_attempts {
                    return Err(err);
                }
            }
        }
    }
    Ok(())
}

/// Calculates the burn amount based on the current balance and burn rate.
/// Returns the calculated amount or zero if there's an issue.
pub fn calculate_percentage_of_amount(amount_available: Nat, burn_rate: u8) -> u128 {
    let balance_u128: u128 = match amount_available.0.try_into() {
        Ok(val) => val,
        Err(_) => {
            error!("Failed to convert Nat to u128. Returning 0 as burn amount.");
            return 0;
        }
    };

    if burn_rate > 100 {
        error!("Burn rate couldn't be more that 100. Returning 0 as burn amount.");
        return 0;
    }

    let amount_to_burn = balance_u128.saturating_mul(burn_rate as u128) / 100;

    debug!(
        "Calculated burn amount: {} tokens ({}% of {} e8s).",
        amount_to_burn,
        burn_rate,
        balance_u128
    );

    amount_to_burn
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Nat;

    #[test]
    fn test_calculate_percentage_of_amount_valid_rate() {
        let amount_available = Nat::from(1000u128);
        let burn_rate = 10;
        let expected_burn_amount = 100u128; // 10% of 1000

        let burn_amount = calculate_percentage_of_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, expected_burn_amount);
    }

    #[test]
    fn test_calculate_percentage_of_amount_zero_rate() {
        let amount_available = Nat::from(1000u128);
        let burn_rate = 0;

        let burn_amount = calculate_percentage_of_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, 0u128);
    }

    #[test]
    fn test_calculate_percentage() {
        let amount_available = Nat::from(50000000u128);
        let burn_rate = 33;

        let burn_amount = calculate_percentage_of_amount(amount_available, burn_rate);
        println!("Burn amount: {}", burn_amount);
        // assert_eq!(burn_amount, 0u128);
    }

    #[test]
    fn test_calculate_percentage_of_amount_rate_over_100() {
        let amount_available = Nat::from(1000u128);
        let burn_rate = 110;

        let burn_amount = calculate_percentage_of_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, 0u128);
    }

    #[test]
    fn test_calculate_percentage_of_amount_100_percent_rate() {
        let amount_available = Nat::from(500u128);
        let burn_rate = 100;
        let expected_burn_amount = 500u128; // 100% of 500

        let burn_amount = calculate_percentage_of_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, expected_burn_amount);
    }

    #[test]
    fn test_calculate_percentage_of_amount_minimal_balance() {
        let amount_available = Nat::from(1u128); // Minimal balance
        let burn_rate = 50;
        let expected_burn_amount = 0u128; // 50% of 1 is 0.5, but result is truncated to 0

        let burn_amount = calculate_percentage_of_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, expected_burn_amount);
    }

    #[test]
    fn test_calculate_percentage_of_amount_large_balance() {
        let amount_available = Nat::from(1_000_000_000_000u128); // 1 trillion
        let burn_rate = 25;
        let expected_burn_amount = 250_000_000_000u128; // 25% of 1 trillion

        let burn_amount = calculate_percentage_of_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, expected_burn_amount);
    }

    #[test]
    fn test_calculate_percentage_of_amount_decimal_handling() {
        let amount_available = Nat::from(1234u128);
        let burn_rate = 33;
        let expected_burn_amount = 407u128; // 33% of 1234 is 407.22, rounded down

        let burn_amount = calculate_percentage_of_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, expected_burn_amount);
    }

    #[test]
    fn test_calculate_percentage_of_amount_overflow() {
        let amount_available = Nat::from(u128::MAX);
        let burn_rate = 100;

        // This test is designed to ensure the function doesn't panic on overflow,
        // but in this context it should be handled safely.
        let burn_amount = calculate_percentage_of_amount(amount_available, burn_rate);
        println!("burn_amount = {}", burn_amount);
    }
}
