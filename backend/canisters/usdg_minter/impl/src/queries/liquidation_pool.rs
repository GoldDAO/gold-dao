use crate::state::read_state;
use crate::vault::FeeBucket;
use crate::{GLDT, USDG};
use ic_cdk::query;
use icrc_ledger_types::icrc1::account::Account;
use usdg_minter_api::queries::get_lp_position::LiquidationPoolPosition;

#[query]
fn get_lp_position(account: Option<Account>) -> LiquidationPoolPosition {
    let account = account.unwrap_or(ic_cdk::caller().into());

    read_state(|s| LiquidationPoolPosition {
        gldt_returns: s.liquidation_return.get(&account).unwrap_or(&GLDT::ZERO).0,
        usdg_available: s.liquidation_pool.get(&account).unwrap_or(&USDG::ZERO).0,
    })
}
