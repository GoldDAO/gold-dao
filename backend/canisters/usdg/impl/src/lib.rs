use crate::numeric::{Factor, GoldPrice, GLDT};

pub mod cbor;
pub mod lifecycle;
pub mod management;
pub mod numeric;
pub mod queries;
pub mod state;
pub mod tasks;
pub mod transfer;
pub mod updates;
pub mod vault;

pub const E8S: u64 = 100_000_000;

/// Time constants
const SEC_NANOS: u64 = 1_000_000_000;

/// Minimum Amounts
const MINIMUM_MARGIN_AMOUNT: GLDT = GLDT::from_unscaled(50);

// Default price for 0.01g of gold of $0.83.
pub const DEFAULT_GOLD_PRICE: GoldPrice = GoldPrice::from_e8s(8_300_000);

pub const MINIMUM_COLLATERAL_RATIO: Factor = Factor::from_e8s(105_000_000);
