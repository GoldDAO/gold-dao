use std::collections::HashMap;

use ic_cdk_macros::query;
use types::{ TokenInfo, TokenSymbol };

use crate::state::read_state;

#[query(hidden = true)]
fn get_reward_token_types() -> HashMap<TokenSymbol, TokenInfo> {
    read_state(|state| { state.data.tokens.clone() })
}
