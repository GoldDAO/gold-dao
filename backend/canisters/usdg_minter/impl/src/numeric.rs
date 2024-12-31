use crate::E8S;
use candid::CandidType;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::iter::Sum;
use std::marker::PhantomData;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
pub struct CheckedAmountOf<Unit>(pub u64, PhantomData<Unit>);

impl<Unit> Serialize for CheckedAmountOf<Unit> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de, Unit> Deserialize<'de> for CheckedAmountOf<Unit> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        u64::deserialize(deserializer).map(Self::from_e8s)
    }
}

impl<Unit> CandidType for CheckedAmountOf<Unit> {
    fn _ty() -> candid::types::Type {
        u64::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        self.0.idl_serialize(serializer)
    }
}

impl<C, Unit> minicbor::Encode<C> for CheckedAmountOf<Unit> {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.u64(self.0)?;
        Ok(())
    }
}

impl<'b, C, Unit> minicbor::Decode<'b, C> for CheckedAmountOf<Unit> {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        match d.u64() {
            Ok(n) => Ok(Self::from_e8s(n)),
            Err(e) => Err(e),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Ord, PartialOrd, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum USDGUnit {}

#[derive(PartialEq, Eq, Debug, Ord, PartialOrd, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum GLDTUnit {}

#[derive(PartialEq, Eq, Debug, Ord, PartialOrd, Clone, Copy)]
pub enum GoldPriceUnit {}

#[derive(PartialEq, Eq, Debug, Ord, PartialOrd, Clone, Copy)]
pub enum FactorUnit {}

pub type USDG = CheckedAmountOf<USDGUnit>;
pub type GLDT = CheckedAmountOf<GLDTUnit>;
pub type Factor = CheckedAmountOf<FactorUnit>;
pub type GoldPrice = CheckedAmountOf<GoldPriceUnit>;

impl<Unit> CheckedAmountOf<Unit> {
    pub const ZERO: Self = Self(0, PhantomData);
    pub const ONE: Self = Self(E8S, PhantomData);
    pub const TWO: Self = Self(2 * E8S, PhantomData);
    pub const MAX: Self = Self(u64::MAX, PhantomData);

    #[inline]
    pub const fn from_e8s(value: u64) -> Self {
        Self(value, PhantomData)
    }

    pub const fn from_unscaled(value: u64) -> Self {
        Self(value * E8S, PhantomData)
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self::from_e8s)
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self::from_e8s)
    }

    pub fn to_f64(&self) -> f64 {
        self.0 as f64 / E8S as f64
    }

    pub fn to_decimal(&self) -> Decimal {
        Decimal::from(self.0) / Decimal::from(E8S)
    }

    pub fn checked_mul(self, factor: Factor) -> Option<Self> {
        let mut result = self.to_decimal().checked_mul(factor.to_decimal())?;
        result.rescale(8);
        Some(Self::from_e8s(result.mantissa() as u64))
    }
    pub fn checked_div_factor(self, factor: Factor) -> Option<Self> {
        let mut result = self.to_decimal().checked_div(factor.to_decimal())?;
        result.rescale(8);
        Some(Self::from_e8s(result.mantissa() as u64))
    }

    pub fn checked_div(self, amount: Self) -> Option<Factor> {
        let mut result = self.to_decimal().checked_div(amount.to_decimal())?;
        result.rescale(8);
        Some(Factor::from_e8s(result.mantissa() as u64))
    }
}

impl<Unit> Sum for CheckedAmountOf<Unit> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.into_iter()
            .fold(CheckedAmountOf(0, PhantomData), |acc, x| {
                acc.checked_add(x).unwrap()
            })
    }
}

impl GLDT {
    pub fn checked_mul_rate(self, factor: GoldPrice) -> Option<USDG> {
        let mut result = self.to_decimal().checked_mul(factor.to_decimal())?;
        result.rescale(8);
        Some(USDG::from_e8s(result.mantissa() as u64))
    }
}

impl USDG {
    pub fn checked_div_rate(self, factor: GoldPrice) -> Option<GLDT> {
        let mut result = self.to_decimal().checked_div(factor.to_decimal())?;
        result.rescale(8);
        Some(GLDT::from_e8s(result.mantissa() as u64))
    }
}

impl Factor {
    pub fn pow(self, power: u64) -> Option<Factor> {
        if power == 0 {
            return Some(Factor::ONE);
        }
        let mut result = Factor::ONE;
        for _ in 0..power {
            result = result.checked_mul(self)?;
        }
        Some(result)
    }
}

macro_rules! impl_from {
    ($($t:ty),* $(,)?) => {$(
        impl<Unit> From<$t> for CheckedAmountOf<Unit> {
            #[inline]
            fn from(value: $t)
            -> Self {
                Self(u64::from(value), PhantomData)
            }
        }
    )*};
}

impl_from! { u8, u16, u32, u64 }

impl<Unit> From<CheckedAmountOf<Unit>> for candid::Nat {
    fn from(value: CheckedAmountOf<Unit>) -> Self {
        candid::Nat::from(value.0)
    }
}

impl<Unit> fmt::Debug for CheckedAmountOf<Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", DisplayAmount(self.0))
    }
}

impl<Unit> fmt::Display for CheckedAmountOf<Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", DisplayAmount(self.0))
    }
}

pub struct DisplayAmount(pub u64);

impl fmt::Display for DisplayAmount {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        const E8S: u64 = 100_000_000;
        let int = self.0 / E8S;
        let frac = self.0 % E8S;

        if frac > 0 {
            let frac_width: usize = {
                // Count decimal digits in the fraction part.
                let mut d = 0;
                let mut x = frac;
                while x > 0 {
                    d += 1;
                    x /= 10;
                }
                d
            };
            debug_assert!(frac_width <= 8);
            let frac_prefix: u64 = {
                // The fraction part without trailing zeros.
                let mut f = frac;
                while f % 10 == 0 {
                    f /= 10
                }
                f
            };

            write!(fmt, "{}.", int)?;
            for _ in 0..(8 - frac_width) {
                write!(fmt, "0")?;
            }
            write!(fmt, "{}", frac_prefix)
        } else {
            write!(fmt, "{}.0", int)
        }
    }
}
