use candid::Nat;
use num_bigint::BigUint;
const SCALE_FACTOR: u64 = 100_000_000_000_000u64;
use candid::CandidType;
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::ops::Deref;
use tracing::error;

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

use thiserror::Error;
#[derive(Debug, Error)]
pub enum PercentageError {
    #[error("Percentage cannot be greater than 100: {0}")]
    PercentageOutOfBounds(u8),
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, CandidType, Serialize, Deserialize, Default,
)]
pub struct Percentage(u8);

impl Into<ICRC3Value> for Percentage {
    fn into(self) -> ICRC3Value {
        ICRC3Value::Nat(Nat::from(self.0))
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl Deref for Percentage {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use std::cmp::Ordering;
impl PartialEq<u8> for Percentage {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}
impl PartialOrd<u8> for Percentage {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}
impl PartialEq<Percentage> for u8 {
    fn eq(&self, other: &Percentage) -> bool {
        *self == other.0
    }
}
impl PartialOrd<Percentage> for u8 {
    fn partial_cmp(&self, other: &Percentage) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl Percentage {
    pub const MAX: Percentage = Percentage(100);

    pub fn new(value: u8) -> Result<Self, PercentageError> {
        if value > Self::MAX {
            Err(PercentageError::PercentageOutOfBounds(value))
        } else {
            Ok(Self(value))
        }
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub fn is_full(self) -> bool {
        self.0 == Self::MAX
    }

    // e.g. 0.32 = 32%
    pub fn as_fraction(self) -> f64 {
        self.0 as f64 / 100.0
    }

    // e.g. 32% = 32.0
    pub fn as_float(self) -> f64 {
        self.0 as f64
    }

    pub fn apply_to(&self, amount: &Nat) -> Nat {
        Nat::from((&amount.0 * BigUint::from(self.0)) / BigUint::from(100_u8))
    }
}

use std::ops::Mul;
impl Mul<f64> for Percentage {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        self.as_fraction() * rhs
    }
}

use std::ops::Div;
impl Div<f64> for Percentage {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        self.as_fraction() / rhs
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, CandidType, Serialize, Deserialize,
)]
pub struct Rate {
    /// Fixed-point value scaled by 1e8. e.g. 30% = 0.3 = 30_000_000 (scaled by 1e8)
    value: u64,
}

impl Rate {
    pub const SCALE: u128 = 100_000_000;

    pub const ZERO: Self = Self { value: 0 };
    pub const ONE: Self = Self {
        value: Self::SCALE as u64,
    }; // 100%

    /// Create from percent (e.g. 30 → 30%)
    pub fn from_percent(percent: u64) -> Self {
        Self {
            value: percent * (Self::SCALE as u64) / 100,
        }
    }

    /// Create directly from scaled value
    pub fn from_scaled(value: u64) -> Self {
        Self { value }
    }

    pub fn as_scaled(self) -> u64 {
        self.value
    }

    /// Apply rate to Nat amount
    pub fn apply_to(&self, amount: &Nat) -> Nat {
        Nat::from((&amount.0 * BigUint::from(self.value)) / BigUint::from(Self::SCALE))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_apply_bonus_multiplier() {
        let base_value = Nat(BigUint::from(100_000_000u64));
        let bonus_multiplier = 1.05;
        let expected_result = Nat::from(105_000_000u64);

        let result = base_value.scale_e8s_mul_f64(bonus_multiplier);
        println!("Result with bonus multiplier: {}", result);

        let readable_result = result.scale_e8s_down();
        assert_eq!(readable_result, expected_result)
    }

    #[test]
    fn test_percentage_new_valid() {
        for value in 0..=100 {
            let p = Percentage::new(value).unwrap();
            assert_eq!(p.value(), value);
        }
    }

    #[test]
    fn test_percentage_new_invalid_above_100() {
        let result = Percentage::new(101);
        assert!(matches!(
            result,
            Err(PercentageError::PercentageOutOfBounds(101))
        ));
    }

    #[test]
    fn test_percentage_is_full_true() {
        let full = Percentage::new(100).unwrap();
        assert!(full.is_full());
    }

    #[test]
    fn test_percentage_is_full_false() {
        let partial = Percentage::new(99).unwrap();
        assert!(!partial.is_full());
    }

    #[test]
    fn test_percentage_deref() {
        let p = Percentage::new(42).unwrap();
        assert_eq!(*p, 42u8);
    }

    #[test]
    fn test_percentage_apply_to_zero_amount() {
        let amount = Nat::from(0u128);
        let p = Percentage::new(42).unwrap();
        let result = p.apply_to(&amount);
        assert_eq!(result, Nat::from(0u128));
    }

    #[test]
    fn test_percentage_apply_to_full_amount() {
        let amount = Nat::from(123456789u128);
        let p = Percentage::new(100).unwrap();
        let result = p.apply_to(&amount);
        assert_eq!(result, amount);
    }

    #[test]
    fn test_percentage_apply_to_partial_amount() {
        let amount = Nat::from(1000u128);
        let p = Percentage::new(25).unwrap();
        let expected = Nat::from(250u128);
        assert_eq!(p.apply_to(&amount), expected);
    }

    #[test]
    fn test_percentage_apply_to_rounding_behavior() {
        let amount = Nat::from(3u128);
        let p = Percentage::new(33).unwrap(); // 3 * 0.33 = 0.99 => floor to 0
        let result = p.apply_to(&amount);
        assert_eq!(result, Nat::from(0u128));
    }

    #[test]
    fn test_invalid_percentage() {
        let p = Percentage::new(120);
        assert!(matches!(
            p,
            Err(PercentageError::PercentageOutOfBounds(120))
        ));
    }

    fn nat(value: u128) -> Nat {
        Nat::from(BigUint::from(value))
    }

    #[test]
    fn applies_full_rate_correctly() {
        let rate = Rate::ONE; // 100%
        let amount = nat(1_000_000_000);

        let result = rate.apply_to(&amount);

        assert_eq!(result, amount);
    }

    #[test]
    fn applies_zero_rate_correctly() {
        let rate = Rate::ZERO;
        let amount = nat(1_000_000_000);

        let result = rate.apply_to(&amount);

        assert_eq!(result, nat(0));
    }

    #[test]
    fn applies_fractional_percentage_precisely() {
        // 0.71428571428% ≈ 0.0071428571
        let rate = Rate::from_scaled(714_286);

        let amount = nat(1_000_000_000); // 10 ICP (e8s)
        let result = rate.apply_to(&amount);

        // Expected: 1_000_000_000 * 0.00714286 ≈ 7_142_860
        assert_eq!(result, nat(7_142_860));
    }

    #[test]
    fn rounding_is_deterministic() {
        let rate = Rate::from_scaled(333_333); // ~0.333333%
        let amount = nat(1);

        let result = rate.apply_to(&amount);

        // Integer math → rounds down
        assert_eq!(result, nat(0));
    }

    #[test]
    fn calculate_value() {
        let rate = Rate::from_scaled(392_857);
        let amount = nat(5_000_000_000);

        let result = rate.apply_to(&amount);
        println!("Result: {}", result);
    }

    #[test]
    fn large_amount_does_not_overflow() {
        let rate = Rate::from_percent(30); // 30%
        let amount = nat(u128::MAX / 2);

        let result = rate.apply_to(&amount);

        // Should not panic and must be <= amount
        assert!(result <= amount);
    }

    #[test]
    fn simple_check_rate_apply_to_nat() {
        let rate = Rate::from_percent(30); // 30%
        let amount = nat(1234);

        let result = rate.apply_to(&amount);

        // Should not panic and must be <= amount
        assert!(result == Nat::from(370u64));
    }

    #[test]
    fn weekly_rate_split_into_intervals() {
        // 30% weekly
        let weekly_rate = Rate::from_percent(30);

        // 7 days * 24h / 4h = 42 intervals
        let intervals = 42;

        let interval_rate = Rate::from_scaled(weekly_rate.as_scaled() / intervals);

        let amount = nat(1_000_000_000);
        let interval_amount = interval_rate.apply_to(&amount);

        // Applying interval rate 42 times ≈ weekly amount
        let mut total = BigUint::from(0u8);
        for _ in 0..intervals {
            total += interval_amount.0.clone();
        }

        let weekly_amount = weekly_rate.apply_to(&amount);

        // Allow small dust loss (integer division)
        assert!(total <= weekly_amount.0);
    }

    #[test]
    fn test_scale_e8s_down_rounding_and_zero() {
        let small_value = Nat::from(SCALE_FACTOR - 1);
        let big_value = Nat::from(SCALE_FACTOR * 5);

        let down_small = small_value.scale_e8s_down();
        let down_big = big_value.scale_e8s_down();

        assert_eq!(down_small, Nat::from(0u64)); // should floor down
        assert_eq!(down_big, Nat::from(5u64));
    }

    #[test]
    fn test_scaled_e8s_mul_basic() {
        let base = Nat::from(2u64);
        let factor = 5u64;
        let result = base.scaled_e8s_mul(factor);
        assert_eq!(result, Nat::from(10u64));
    }

    #[test]
    fn test_percentage_mul_and_div_with_f64() {
        let p = Percentage::new(50).unwrap();
        let x = 200.0;
        assert_eq!(p * x, 100.0);
        assert_eq!(p / 0.5, 1.0);
    }

    #[test]
    fn test_e8s_mul_basic() {
        let base = 2.0;
        let factor = Percentage::new(50).unwrap();
        assert_eq!(factor * base, 1.0);
    }

    #[test]
    fn test_e8s_div_basic() {
        let base = 2.0;
        let factor = Percentage::new(100).unwrap();
        assert_eq!(factor / base, 0.5);
    }

    #[test]
    fn test_partial_ord_and_eq_with_u8() {
        let p = Percentage::new(30).unwrap();
        assert!(p > 20u8);
        assert!(p < 40u8);
        assert_eq!(p, 30u8);
    }

    #[test]
    fn test_percentage_apply_to_large_nat() {
        let big_amount = Nat(BigUint::from(10u128.pow(20)));
        let p = Percentage::new(1).unwrap();
        let result = p.apply_to(&big_amount);
        // 1% of 10^20 = 10^18
        assert_eq!(result, Nat(BigUint::from(10u128.pow(18))));
    }

    #[test]
    fn test_combined_scaling_pipeline() {
        let base = Nat(BigUint::from(50_000_000u64)); // scaled 0.5
        let multiplier = 2.0;
        let percentage = Percentage::new(50).unwrap(); // 50%

        let scaled = base.scale_e8s_mul_f64(multiplier);
        let partial = percentage.apply_to(&scaled);
        let downscaled = partial.scale_e8s_down();

        println!("Combined scaling result: {}", downscaled);
        assert!(downscaled > Nat::from(0u64));
    }

    #[test]
    fn test_scaled_e8s_div_by_zero_panics() {
        let a = Nat::from(100u64);
        let b = Nat::from(0u64);
        let result = std::panic::catch_unwind(|| a.scaled_e8s_div(&b));
        assert!(result.is_err(), "Division by zero should panic");
    }
}
