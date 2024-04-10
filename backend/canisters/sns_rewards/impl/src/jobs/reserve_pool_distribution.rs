/*!
# Reserve pool distribution

- fn distribute_reserve_pool
transfers tokens from reserve pool to the reward pool on a daily basis.
- currently this only happens for GLDGov
- the daily amount to be transferred is decided via a proposal

*/

use crate::{
    consts::{ RESERVE_POOL_SUB_ACCOUNT, REWARD_POOL_SUB_ACCOUNT },
    state::{ mutate_state, read_state },
    utils::transfer_token,
};
use canister_time::{ now_millis, run_interval, DAY_IN_MS };
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;
use tracing::{ debug, error, info };
use types::{ Milliseconds, TokenSymbol };

const DISTRIBUTION_INTERVAL: Milliseconds = DAY_IN_MS;

pub fn start_job() {
    run_interval(Duration::from_millis(DISTRIBUTION_INTERVAL), run_distribution);
}

pub fn run_distribution() {
    ic_cdk::spawn(distribute_reserve_pool())
}

pub async fn distribute_reserve_pool() {
    debug!("RESERVE POOL DISTRIBUTION - START");
    handle_gldgov_distribution().await;
    debug!("RESERVE POOL DISTRIBUTION - FINISH");
}

async fn handle_gldgov_distribution() {
    // chceck GLDGov is a valid token string
    let token = match TokenSymbol::parse("GLDGov") {
        Ok(t) => t,
        Err(e) => {
            error!("ERROR : failed to parse GLDGov token. error : {:?}", e);
            return;
        }
    };
    // get the gldgov ledger id
    let gldgov_ledger_id = match read_state(|s| s.data.tokens.get(&token).copied()) {
        Some(token_info) => token_info.ledger_id,
        None => {
            error!("ERROR : failed to get token information and ledger id for token {:?}", &token);
            return;
        }
    };
    // get the daily transfer amount of gldgov
    let amount = match read_state(|s| s.data.daily_reserve_transfer.get(&token).cloned()) {
        Some(amount) => amount,
        None => {
            error!("ERROR: can't find daily transfer amount for token : {:?} in state", token);
            return;
        }
    };
    // check we're more than 1 day since the last distribution
    let last_run = read_state(|s| s.data.last_daily_reserve_transfer_time);
    let time_now = now_millis();
    let interval = time_now - last_run;
    if interval < DAY_IN_MS {
        debug!("RESERVE POOL DISTRIBUTION: Time since last reserve distribution {} is less than one day. ", interval);
        return;
    }

    let reward_pool_account = Account {
        owner: ic_cdk::api::id(),
        subaccount: Some(REWARD_POOL_SUB_ACCOUNT),
    };

    match
        transfer_token(
            RESERVE_POOL_SUB_ACCOUNT,
            reward_pool_account,
            gldgov_ledger_id,
            amount.clone()
        ).await
    {
        Ok(_) => {
            info!("SUCCESS : {:?} GLDGov transferred to reward pool successfully", amount);
            mutate_state(|s| {
                s.data.last_daily_reserve_transfer_time = time_now;
            })
        }
        Err(e) => {
            // TODO - should we update the last_daily_reserve_transfer_time here even though it didn't succeed. If we see a failure we'd still want to correct it, upgrade and let the transfer run instead of stopping because it previously failed.
            error!(
                "ERROR : GLDGov failed to transfer from reserve pool to reward pool with error : {:?}",
                e
            );
        }
    }
}
