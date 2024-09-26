use candid::Nat;
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;
use tracing::{ debug, error };

pub const RETRY_DELAY: Duration = Duration::from_secs(5 * 60); // each 5 minutes

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

pub async fn retry_with_attempts<F, Fut>(
    max_attempts: u8,
    _delay_duration: Duration,
    f: F
)
    -> Result<(), String>
    where F: FnMut() -> Fut + 'static, Fut: std::future::Future<Output = Result<(), String>>
{
    // Run code with delay
    fn recursive<
        F: FnMut() -> Fut + 'static,
        Fut: std::future::Future<Output = Result<(), String>>
    >(mut f: F, attempt: u8, max_attempts: u8) {
        ic_cdk_timers::set_timer(Duration::ZERO, move || {
            ic_cdk::spawn(async move {
                if f().await.is_ok() {
                } else if attempt < max_attempts {
                    recursive(f, attempt + 1, max_attempts);
                } else {
                    error!("Failed to execute the action after {} attempts", max_attempts);
                }
            });
        });
    }

    recursive(f, 0, max_attempts);

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
