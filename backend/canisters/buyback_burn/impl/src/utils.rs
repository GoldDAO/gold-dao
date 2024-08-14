use candid::Nat;
use tracing::{ error, debug };

/// Calculates the burn amount based on the current balance and burn rate.
/// Returns the calculated amount or zero if there's an issue.
/// FIXME If the burn rate is incorrect -> cancel the job at all
pub fn calculate_burn_amount(amount_available: Nat, burn_rate: u8) -> u128 {
    let balance_u128: u128 = match amount_available.0.try_into() {
        Ok(val) => val,
        Err(_) => {
            error!("Failed to convert Nat to u128. Returning 0 as burn amount.");
            return 0;
        }
    };

    if burn_rate == 0 || burn_rate > 100 {
        error!("Invalid burn rate: {}. It must be between 1 and 100.", burn_rate);
        return 0;
    }

    // TODO: think of the sequence here
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
    fn test_calculate_burn_amount_valid_rate() {
        let amount_available = Nat::from(1000u128);
        let burn_rate = 10;
        let expected_burn_amount = 100u128; // 10% of 1000

        let burn_amount = calculate_burn_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, expected_burn_amount);
    }

    #[test]
    fn test_calculate_burn_amount_zero_rate() {
        let amount_available = Nat::from(1000u128);
        let burn_rate = 0;

        let burn_amount = calculate_burn_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, 0u128);
    }

    #[test]
    fn test_calculate_burn_amount_rate_over_100() {
        let amount_available = Nat::from(1000u128);
        let burn_rate = 110;

        let burn_amount = calculate_burn_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, 0u128);
    }

    #[test]
    fn test_calculate_burn_amount_100_percent_rate() {
        let amount_available = Nat::from(500u128);
        let burn_rate = 100;
        let expected_burn_amount = 500u128; // 100% of 500

        let burn_amount = calculate_burn_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, expected_burn_amount);
    }

    #[test]
    fn test_calculate_burn_amount_minimal_balance() {
        let amount_available = Nat::from(1u128); // Minimal balance
        let burn_rate = 50;
        let expected_burn_amount = 0u128; // 50% of 1 is 0.5, but result is truncated to 0

        let burn_amount = calculate_burn_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, expected_burn_amount);
    }

    #[test]
    fn test_calculate_burn_amount_large_balance() {
        let amount_available = Nat::from(1_000_000_000_000u128); // 1 trillion
        let burn_rate = 25;
        let expected_burn_amount = 250_000_000_000u128; // 25% of 1 trillion

        let burn_amount = calculate_burn_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, expected_burn_amount);
    }

    #[test]
    fn test_calculate_burn_amount_decimal_handling() {
        let amount_available = Nat::from(1234u128);
        let burn_rate = 33;
        let expected_burn_amount = 407u128; // 33% of 1234 is 407.22, rounded down

        let burn_amount = calculate_burn_amount(amount_available, burn_rate);
        assert_eq!(burn_amount, expected_burn_amount);
    }

    #[test]
    fn test_calculate_burn_amount_overflow() {
        let amount_available = Nat::from(u128::MAX);
        let burn_rate = 100;

        // This test is designed to ensure the function doesn't panic on overflow,
        // but in this context it should be handled safely.
        let burn_amount = calculate_burn_amount(amount_available, burn_rate);
        println!("burn_amount = {}", burn_amount);
    }
}
