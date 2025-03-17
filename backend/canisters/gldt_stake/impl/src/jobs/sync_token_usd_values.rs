use candid::{Nat, Principal};
use canister_time::{run_now_then_interval, HOUR_IN_MS};
use gldt_stake_common::{ledgers::GLDT_TX_FEE, reward_tokens::TokenSymbol};
use icpswap_token_canister_c2c_client::getToken;
use std::{collections::HashMap, time::Duration};
use tracing::debug;

use crate::state::{mutate_state, read_state};

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(HOUR_IN_MS), sync_token_usd_values_job);
}

pub fn sync_token_usd_values_job() {
    ic_cdk::spawn(sync_token_usd_values_impl())
}

async fn sync_token_usd_values_impl() {
    let gldt_token_ledger = read_state(|s| s.data.gldt_ledger_id);
    let mut tokens = read_state(|s| s.data.stake_system.reward_types.clone());
    tokens.insert(
        "GLDT".to_string(),
        (gldt_token_ledger, Nat::from(GLDT_TX_FEE)),
    );
    let icpswap_token_canister_id = match Principal::from_text("moe7a-tiaaa-aaaag-qclfq-cai") {
        Ok(id) => id,
        Err(e) => {
            debug!("SYNC_TOKEN_USD_VALUES :: cant parse icpswap_token_canister_id - error {e}");
            return;
        }
    };

    let mut new_price_map: HashMap<TokenSymbol, f64> = HashMap::new();
    let current_price_map = read_state(|s| s.data.stake_system.token_usd_values.clone());

    for (token_symbol, (canister_id, _)) in tokens {
        match getToken(icpswap_token_canister_id, &(canister_id.to_string())).await {
            Ok(token_info) => {
                new_price_map.insert(token_symbol, token_info.priceUSD);
            }
            Err(e) => {
                debug!("SYNC_TOKEN_USD_VALUES :: error fetching {token_symbol} USD value with error - {e:?}. setting previous value or 0 if no previous value");
                let current_token_price = current_price_map.get(&token_symbol).unwrap_or(&0f64);
                new_price_map.insert(token_symbol, *current_token_price);
            }
        }
    }

    mutate_state(|s| s.data.stake_system.set_token_usd_values(new_price_map));
}
