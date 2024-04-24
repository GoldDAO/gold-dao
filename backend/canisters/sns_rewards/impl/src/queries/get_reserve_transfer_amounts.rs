use std::collections::HashMap;

use candid::Nat;
use ic_cdk_macros::query;
use types::TokenSymbol;

use crate::state::read_state;

#[query(hidden = true)]
fn get_reserve_transfer_amounts() -> HashMap<TokenSymbol, Nat> {
    read_state(|state| { state.data.daily_reserve_transfer.clone() })
}
