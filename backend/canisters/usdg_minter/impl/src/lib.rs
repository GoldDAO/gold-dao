use crate::numeric::{Factor, GoldPrice, GLDT, USDG};
use ic_cdk::export_candid;
use icrc_ledger_types::icrc1::account::Account;
use usdg_minter_api::lifecycle::MinterArgument;
use usdg_minter_api::queries::get_events::{GetEventsArg, GetEventsResult};
use usdg_minter_api::queries::get_lp_position::LiquidationPoolPosition;
use usdg_minter_api::updates::add_margin_to_vault::AddMarginArg;
use usdg_minter_api::updates::borrow_from_vault::BorrowArg;
use usdg_minter_api::updates::deposit_liquidity::DepositArg;
use usdg_minter_api::updates::open_vault::{OpenVaultArg, OpenVaultSuccess};
use usdg_minter_api::updates::redeem::RedeemArg;
use usdg_minter_api::updates::repay_debt_to_vault::RepayDebtArg;
use usdg_minter_api::updates::withdraw_liquidity::WithdrawArg;
use usdg_minter_api::LiquidityError;
use usdg_minter_api::{ApiVault, VaultError};

pub mod cbor;
pub mod guard;
pub mod lifecycle;
pub mod logs;
pub mod management;
pub mod memory;
pub mod numeric;
pub mod queries;
pub mod state;
pub mod transfer;
pub mod updates;
pub mod vault;

pub const E8S: u64 = 100_000_000;

/// Time constants
const SEC_NANOS: u64 = 1_000_000_000;

/// Minimum Amounts
const MINIMUM_MARGIN_AMOUNT: GLDT = GLDT::from_unscaled(50);
const MINIMUM_BORROW_AMOUNT: USDG = USDG::from_unscaled(10);
pub const MINIMUM_CLAIMABLE_RETURN: GLDT = GLDT::from_unscaled(5);
pub const MINIMUM_REDEEM_AMOUNT: USDG = USDG::from_unscaled(10);

// Default price for 0.01g of gold of $0.83.
pub const DEFAULT_GOLD_PRICE: GoldPrice = GoldPrice::from_e8s(83_000_000);

pub const MINIMUM_COLLATERAL_RATIO: Factor = Factor::from_e8s(105_000_000);

// The default medium rate is 5%
pub const DEFAULT_MEDIUM_RATE: f64 = 0.05;
// Determines the rate at which the interest rate will evolve.
pub const ALPHA_FACTOR: f64 = 0.1;
pub const MAXIUM_INTEREST_RATE: f64 = 1.0;
pub const MINIMUM_INTEREST_RATE: f64 = 0.01;

export_candid!();
