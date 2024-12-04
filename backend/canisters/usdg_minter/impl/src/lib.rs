use crate::numeric::{Factor, GoldPrice, GLDT, USDG};
use ic_cdk::export_candid;
use icrc_ledger_types::icrc1::account::Account;
use usdg_minter_api::lifecycle::MinterArgument;
use usdg_minter_api::updates::add_margin_to_vault::AddMarginArg;
use usdg_minter_api::updates::borrow_from_vault::BorrowArg;
use usdg_minter_api::updates::open_vault::{OpenVaultArg, OpenVaultSuccess};
use usdg_minter_api::{ApiVault, VaultError};

pub mod cbor;
pub mod lifecycle;
pub mod logs;
pub mod management;
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

// Default price for 0.01g of gold of $0.83.
pub const DEFAULT_GOLD_PRICE: GoldPrice = GoldPrice::from_e8s(83_000_000);

pub const MINIMUM_COLLATERAL_RATIO: Factor = Factor::from_e8s(105_000_000);

export_candid!();
