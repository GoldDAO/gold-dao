use candid::CandidType;
use serde::{Deserialize, Serialize};

// NOTE: take into account the zero_to_one variable value, and set constraints accordingly. E.g. if zero_to_one=true, MaxSellRatio(500) means at most 0.5 ICP per GOLDAO.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SwapConstraint {
    // Only swap if sell_amount / buy_amount <= N. E.g. MaxSellRatio(500): at most 500 ICP per GOLDAO.
    MaxSellRatio(u64),
    // Only swap if sell_amount / buy_amount >= N. E.g. MinSellRatio(100): at least 100 ICP per GOLDAO.
    MinSellRatio(u64),
    // Only swap if buy_amount / sell_amount <= N. E.g. MaxBuyRatio(10000): at most 10000 GOLDAO per ICP.
    MaxBuyRatio(u64),
    // Only swap if buy_amount / sell_amount >= N. E.g. MinBuyRatio(5000): at least 5000 GOLDAO per ICP.
    MinBuyRatio(u64),
}

impl SwapConstraint {
    // Returns true if the quote satisfies the constraint.
    // Uses cross-multiplication to avoid integer division precision loss.
    pub fn check_quote(&self, sell_amount: u128, buy_amount: u128) -> bool {
        if sell_amount == 0 || buy_amount == 0 {
            return false;
        }
        match self {
            SwapConstraint::MaxSellRatio(n) => sell_amount <= *n as u128 * buy_amount,
            SwapConstraint::MinSellRatio(n) => sell_amount >= *n as u128 * buy_amount,
            SwapConstraint::MaxBuyRatio(n) => buy_amount <= *n as u128 * sell_amount,
            SwapConstraint::MinBuyRatio(n) => buy_amount >= *n as u128 * sell_amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // MaxSellRatio: sell/buy <= N
    #[test]
    fn max_sell_ratio_passes() {
        // sell=100, buy=100 → sell/buy=1, max=1 → pass
        assert!(SwapConstraint::MaxSellRatio(1).check_quote(100, 100));
    }

    #[test]
    fn max_sell_ratio_fails() {
        // sell=199, buy=100 → sell/buy=1.99, max=1 → fail
        assert!(!SwapConstraint::MaxSellRatio(1).check_quote(199, 100));
    }

    // MinSellRatio: sell/buy >= N
    #[test]
    fn min_sell_ratio_passes() {
        // sell=500, buy=100 → sell/buy=5, min=5 → pass
        assert!(SwapConstraint::MinSellRatio(5).check_quote(500, 100));
    }

    #[test]
    fn min_sell_ratio_fails() {
        // sell=400, buy=100 → sell/buy=4, min=5 → fail
        assert!(!SwapConstraint::MinSellRatio(5).check_quote(400, 100));
    }

    // MaxBuyRatio: buy/sell <= N
    #[test]
    fn max_buy_ratio_passes() {
        // sell=1, buy=10000 → buy/sell=10000, max=10000 → pass
        assert!(SwapConstraint::MaxBuyRatio(10000).check_quote(1, 10000));
    }

    #[test]
    fn max_buy_ratio_fails() {
        // sell=1, buy=10001 → buy/sell=10001, max=10000 → fail
        assert!(!SwapConstraint::MaxBuyRatio(10000).check_quote(1, 10001));
    }

    // MinBuyRatio: buy/sell >= N
    #[test]
    fn min_buy_ratio_passes() {
        // sell=1, buy=10000 → buy/sell=10000, min=5000 → pass
        assert!(SwapConstraint::MinBuyRatio(5000).check_quote(1, 10000));
    }

    #[test]
    fn min_buy_ratio_fails() {
        // sell=1, buy=4999 → buy/sell=4999, min=5000 → fail
        assert!(!SwapConstraint::MinBuyRatio(5000).check_quote(1, 4999));
    }

    // Edge cases
    #[test]
    fn zero_amounts_always_fail() {
        assert!(!SwapConstraint::MaxSellRatio(1000).check_quote(0, 100));
        assert!(!SwapConstraint::MinBuyRatio(1).check_quote(100, 0));
    }
}
