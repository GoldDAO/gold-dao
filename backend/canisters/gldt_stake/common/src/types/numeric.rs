use candid::Nat;
use num_bigint::BigUint;
const SCALE_FACTOR: u64 = 100_000_000_000_000u64;

pub trait ScaledArithmetic {
    fn scaled_e8s_div(&self, factor: &Self) -> Self;
    fn scale_e8s_down(&self) -> Self;
    fn scale_e8s_mul_f64(&self, bonus_multiplier: f64) -> Self;
    fn scaled_e8s_mul(&self, factor: u64) -> Self;
}

impl ScaledArithmetic for Nat {
    fn scale_e8s_down(&self) -> Self {
        if self >= &SCALE_FACTOR {
            Nat(&self.0 / BigUint::from(SCALE_FACTOR))
        } else {
            Nat(BigUint::from(0u64))
        }
    }

    fn scaled_e8s_div(&self, other: &Self) -> Self {
        Nat((&self.0 * BigUint::from(SCALE_FACTOR)) / &other.0)
    }

    fn scaled_e8s_mul(&self, factor: u64) -> Self {
        Nat(&self.0 * BigUint::from(factor))
    }

    fn scale_e8s_mul_f64(&self, bonus_multiplier: f64) -> Self {
        let scaled_bonus = (bonus_multiplier * SCALE_FACTOR as f64) as u64; // scale up multiplier
        self.scaled_e8s_mul(scaled_bonus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaled_e8s_div_proportion() {
        let user_tokens = Nat(BigUint::from(100_000_000u64)); // User has 1 token (1 ICP)
        let total_stake = Nat(BigUint::from(10_000_000_000u64)); // Total stake is 100 tokens (100 ICP)
        let reward_tokens = Nat(BigUint::from(100_000_000_000u64)); // 1000 Reward tokens
        let expected_rewards = Nat::from(1_000_000_000u64);
        // Calculate the percentage
        let percentage_scaled = user_tokens.scaled_e8s_div(&total_stake);

        // Validate the results
        assert_eq!(percentage_scaled, Nat::from(1_000_000_000_000u64)); // Scaled percentage (1% in e8s)

        let user_rewards = (reward_tokens * percentage_scaled).scale_e8s_down();
        assert_eq!(user_rewards, expected_rewards)
    }

    #[test]
    fn test_scaled_e8s_div_proportion_smaller_percentage() {
        let user_tokens = Nat(BigUint::from(100_000_000u64)); // User has 1 ICP
        let total_stake = Nat(BigUint::from(10_000_000_000_000_000u64)); // Total stake is 100_000_000 ICP
                                                                         // percentage of stake pool is = 0.000001%
        let reward_tokens = Nat(BigUint::from(100_000_000_000u64)); // 1000 Reward tokens
        let expected_rewards = Nat::from(1_000u64);
        // Calculate the percentage
        let percentage_scaled = user_tokens.scaled_e8s_div(&total_stake);

        let user_rewards = (reward_tokens * percentage_scaled).scale_e8s_down();
        assert_eq!(user_rewards, expected_rewards)
    }

    #[test]
    fn test_apply_bonus_multiplier() {
        let base_value = Nat(BigUint::from(100_000_000u64)); // Example value (e8s)

        let bonus_multiplier = 1.05; // 5% bonus
        let expected_result = Nat::from(105_000_000u64);

        // Apply the bonus
        let result = base_value.scale_e8s_mul_f64(bonus_multiplier);
        println!("Result with bonus multiplier: {}", result);

        let readable_result = result.scale_e8s_down();
        assert_eq!(readable_result, expected_result)
    }
}
