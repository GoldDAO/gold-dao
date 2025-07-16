use canister_time::{run_now_then_interval, HOUR_IN_MS};
use icpswap_token_canister_c2c_client::getToken;
use std::{collections::HashMap, time::Duration};
use tracing::debug;
use types::TokenSymbol;
use utils::consts::ICPSWAP_TOKEN_CANISTER_ID;

use crate::state::{mutate_state, read_state};

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(HOUR_IN_MS), sync_token_usd_values_job);
}

pub fn sync_token_usd_values_job() {
    ic_cdk::futures::spawn(sync_token_usd_values_impl())
}

async fn sync_token_usd_values_impl() {
    let mut tokens = read_state(|s| s.data.stake_system.reward_types.clone());

    tokens.insert(TokenSymbol::GLDT);

    let mut new_price_map: HashMap<TokenSymbol, f64> = HashMap::new();
    let current_price_map = read_state(|s| s.data.stake_system.token_usd_values.clone());

    for token_symbol in tokens {
        let ledger_id = token_symbol.get_token_info().ledger_id;
        match getToken(ICPSWAP_TOKEN_CANISTER_ID, &(ledger_id.to_string())).await {
            Ok(token_info) => {
                new_price_map.insert(token_symbol, token_info.priceUSD);
            }
            Err(e) => {
                debug!("SYNC_TOKEN_USD_VALUES :: error fetching {:?} USD value with error - {:?}. setting previous value or 0 if no previous value", token_symbol, e);
                let current_token_price = current_price_map.get(&token_symbol).unwrap_or(&0f64);
                new_price_map.insert(token_symbol, *current_token_price);
            }
        }
    }

    mutate_state(|s| s.data.stake_system.set_token_usd_values(new_price_map));
}
