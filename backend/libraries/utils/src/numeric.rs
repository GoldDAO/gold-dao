use candid::Nat;
use num_bigint::BigUint;
const SCALE_FACTOR: u64 = 100_000_000_000_000u64;
use candid::CandidType;
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

    pub fn apply_to(&self, amount: &Nat) -> Nat {
        Nat::from((&amount.0 * BigUint::from(self.0)) / BigUint::from(100_u8))
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
}
