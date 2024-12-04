use crate::numeric::{Factor, GoldPrice, GLDT, USDG};
use crate::E8S;
use proptest::prelude::*;

fn arb_usd_amount() -> impl Strategy<Value = USDG> {
    (1..10_000_000_000_000_000_u64).prop_map(|a| USDG::from_e8s(a))
}

fn arb_amount() -> impl Strategy<Value = u64> {
    1..2_100_000_000_000_000_u64
}

fn arb_gldt_amount() -> impl Strategy<Value = GLDT> {
    arb_amount().prop_map(|a| GLDT::from_e8s(a))
}

fn arb_factor_amount() -> impl Strategy<Value = Factor> {
    (0..2_100_000_000_000_000_u64).prop_map(|a| Factor::from_e8s(a))
}

#[test]
fn gldt_mul_by_gold_price_available() {
    let gldt_token = GLDT::ONE;
    let rate_amount = GoldPrice::from_e8s(21_000 * E8S);
    assert_eq!(
        gldt_token.checked_mul_rate(rate_amount).unwrap(),
        USDG::from_e8s(21_000 * E8S)
    );
}

#[test]
fn gldt_mul_by_ratio() {
    let gldt_token = GLDT::ONE;
    let ratio_amount = Factor::from_e8s(50_000_000);
    assert_eq!(
        gldt_token.checked_mul(ratio_amount).unwrap(),
        GLDT::from_e8s(50_000_000_u64)
    );
}

#[test]
fn gldt_mul_by_zero() {
    let gldt_token = GLDT::ONE;
    let ratio_amount = Factor::ZERO;
    assert_eq!(gldt_token.checked_mul(ratio_amount).unwrap(), GLDT::ZERO);
}

#[test]
fn ratio_mul_by_zero() {
    let gldt_token = GLDT::ZERO;
    let ratio_amount = Factor::ONE;
    assert_eq!(gldt_token.checked_mul(ratio_amount).unwrap(), GLDT::ZERO);
}

#[test]
fn usdg_mul_by_ratio() {
    let usdg_token = USDG::from_e8s(98_u64);
    let ratio = Factor::from_e8s(50_000_000);
    assert_eq!(
        usdg_token.checked_mul(ratio).unwrap(),
        USDG::from_e8s(49_u64)
    );
}

#[test]
fn usdg_div_by_gold_price() {
    let rate = GoldPrice::from_e8s(1_000 * E8S);
    let usdg = USDG::from_e8s(100 * E8S);
    let result = usdg.checked_div_rate(rate).unwrap();
    assert_eq!(GLDT::from_e8s(10_000_000_u64), result);
}

#[test]
fn factor_pow() {
    let factor = Factor::from_e8s(94_000_000);
    assert_eq!(factor.pow(0_u64).unwrap(), Factor::ONE);
    assert_eq!(factor.pow(1_u64).unwrap(), factor);
    assert_eq!(factor.pow(2_u64).unwrap(), Factor::from_e8s(88360000));
}

#[test]
fn test_mul() {
    let rate = GoldPrice::ONE;
    let usdg = USDG::ONE;
    let gldt = GLDT::ONE;

    assert_eq!(gldt.checked_mul_rate(rate).unwrap(), USDG::ONE);
    assert_eq!(usdg.checked_div_rate(rate).unwrap(), GLDT::ONE);

    assert_eq!(usdg.checked_div(usdg).unwrap(), Factor::ONE);
    assert_eq!(gldt.checked_div(gldt).unwrap(), Factor::ONE);
}

proptest! {
  #[test]
  fn test_addition_properties_usdg(
    usdg1 in arb_usd_amount(),
    usdg2 in arb_usd_amount(),
    usdg3 in arb_usd_amount(),
    ){
    // commutativity
    prop_assert_eq!(usdg1.checked_add(usdg2).unwrap(), usdg2.checked_add(usdg1).unwrap());

    // neutral element
    prop_assert_eq!(usdg1.checked_add(USDG::ZERO).unwrap(), usdg1);

    // associativity
    let sum_usdg1 = usdg1.checked_add(usdg2).unwrap();
    let sum_usdg2 = usdg2.checked_add(usdg3).unwrap();
    prop_assert_eq!(sum_usdg1.checked_add(usdg3).unwrap(), sum_usdg2.checked_add(usdg1).unwrap());
  }

  #[test]
  fn test_addition_properties_gldt(
    gldt1 in arb_gldt_amount(),
    gldt2 in arb_gldt_amount(),
    gldt3 in arb_gldt_amount(),
    ) {
    // commutativity
    prop_assert_eq!(gldt1.checked_add(gldt2).unwrap(), gldt2.checked_add(gldt1).unwrap());

    // neutral element
    prop_assert_eq!(gldt1.checked_add(GLDT::ZERO).unwrap(), gldt1);

    // associativity
    let sum_gldt1 = gldt1.checked_add(gldt2).unwrap();
    let sum_gldt2 = gldt2.checked_add(gldt3).unwrap();
    prop_assert_eq!(sum_gldt1.checked_add(gldt3).unwrap(), sum_gldt2.checked_add(gldt1).unwrap());
  }

  #[test]
  fn test_addition_properties_factor(
    factor1 in arb_factor_amount(),
    factor2 in arb_factor_amount(),
    factor3 in arb_factor_amount(),
    ) {
    // commutativity
    prop_assert_eq!(factor1.checked_add(factor2).unwrap(), factor2.checked_add(factor1).unwrap());

    // neutral element
    prop_assert_eq!(factor1.checked_add(Factor::ZERO).unwrap(), factor1);

    // associativity
    let sum_factor1 = factor1.checked_add(factor2).unwrap();
    let sum_factor2 = factor2.checked_add(factor3).unwrap();
    prop_assert_eq!(sum_factor1.checked_add(factor3).unwrap(), sum_factor2.checked_add(factor1).unwrap());
  }

  #[test]
  fn test_mul_factor(
    factor1 in arb_factor_amount(),
    factor2 in arb_factor_amount(),
    ) {
    // commutativity
    prop_assert_eq!(factor1.checked_mul(factor2).unwrap(), factor2.checked_mul(factor1).unwrap());

    // neutral element
    prop_assert_eq!(factor1.checked_mul(Factor::ONE).unwrap(), factor1);
  }
}
