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
use candid::{ Nat, Principal };
use canister_time::{ now_millis, run_interval, DAY_IN_MS };
use icrc_ledger_types::icrc1::account::{ Account, Subaccount };
use utils::env::Environment;
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
    let gldgov_token_info = match read_state(|s| s.data.tokens.get(&token).copied()) {
        Some(token_info) => token_info,
        None => {
            error!("ERROR : failed to get token information and ledger id for token {:?}", &token);
            return;
        }
    };
    // get the daily transfer amount of gldgov
    let amount_to_transfer = match
        read_state(|s| s.data.daily_reserve_transfer.get(&token).cloned())
    {
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
    if interval < DISTRIBUTION_INTERVAL {
        debug!("RESERVE POOL DISTRIBUTION: Time since last reserve distribution {} is less than one day. ", interval);
        return;
    }

    // check the reserve pool has enough GLDGov to correctly transfer
    match fetch_balance_of_sub_account(gldgov_token_info.ledger_id, RESERVE_POOL_SUB_ACCOUNT).await {
        Ok(balance) => {
            if balance < amount_to_transfer.clone() + gldgov_token_info.fee {
                debug!(
                    "Balance of reserve pool : {} is too low to make a transfer of {} plus a fee of {} ",
                    balance,
                    amount_to_transfer,
                    gldgov_token_info.fee
                );
                return;
            }
        }
        Err(e) => {
            error!(e);
            return;
        }
    }

    let reward_pool_account = Account {
        owner: read_state(|s| s.env.canister_id()),
        subaccount: Some(REWARD_POOL_SUB_ACCOUNT),
    };

    match
        transfer_token(
            RESERVE_POOL_SUB_ACCOUNT,
            reward_pool_account,
            gldgov_token_info.ledger_id,
            amount_to_transfer.clone()
        ).await
    {
        Ok(_) => {
            info!(
                "SUCCESS : {:?} GLDGov transferred to reward pool successfully",
                amount_to_transfer
            );
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

async fn fetch_balance_of_sub_account(
    ledger_canister_id: Principal,
    sub_account: Subaccount
) -> Result<Nat, String> {
    match
        icrc_ledger_canister_c2c_client::icrc1_balance_of(
            ledger_canister_id,
            &(Account {
                owner: read_state(|s| s.env.canister_id()),
                subaccount: Some(sub_account),
            })
        ).await
    {
        Ok(t) => { Ok(t) }
        Err(e) => { Err(format!("ERROR: {:?}", e.1)) }
    }
}
