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
use types::{ Milliseconds, TimestampMillis, TokenSymbol };

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
    // check we're more than 1 day since the last distribution. The last_daily_reserve_transfer_time will be 0 on the first distribution because in state it's initialized with ::default() // 0
    let previous_time_ms = read_state(|s| s.data.last_daily_reserve_transfer_time);
    let current_time_ms = now_millis();

    if !is_valid_distribution_time(previous_time_ms, current_time_ms) {
        debug!(
            "RESERVE POOL DISTRIBUTION: Time since last reserve distribution is less than one day. "
        );
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
                "SUCCESS : {:?} GLDGov tokens transferred to reward pool successfully",
                amount_to_transfer
            );
            mutate_state(|s| {
                s.data.last_daily_reserve_transfer_time = current_time_ms;
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

pub fn is_valid_distribution_time(
    previous_time: TimestampMillis,
    now_time: TimestampMillis
) -> bool {
    // convert the milliseconds to the number of days since UNIX Epoch.
    // integer division means partial days will be truncated down or effectively rounded down. e.g 245.5 becomes 245
    let previous_in_days = previous_time / DISTRIBUTION_INTERVAL;
    let current_in_days = now_time / DISTRIBUTION_INTERVAL;
    // never allow distributions to happen twice i.e if the last run distribution in days since UNIX epoch is the same as the current time in days since the last UNIX Epoch then return early.
    current_in_days != previous_in_days
}

#[cfg(test)]
mod tests {
    use super::is_valid_distribution_time;

    #[test]
    fn test_is_valid_distribution_time() {
        // Scenario 1 - prev time is 1 day prior but less than 24 hours in interval - should be valid
        let prev_time = 1712765654000; // 2024 Apr 10 16:14:14 UTC
        let now_time = 1712834054000; // 2024 Apr 11 11:14:14 UTC
        let is_valid_time = is_valid_distribution_time(prev_time, now_time);
        assert_eq!(is_valid_time, true);

        // Scenario 2 - prev time is 0 days prior ( same day ) - should NOT be valid
        let prev_time = 1712793600000; // 2024 Apr 11 00:00:00 UTC
        let now_time = 1712834054000; // 2024 Apr 11 11:14:14 UTC
        let is_valid_time = is_valid_distribution_time(prev_time, now_time);
        assert_eq!(is_valid_time, false);

        // Scenario 3 - prev time is 2 days prior - should be valid
        let prev_time = 1712620800000; // 2024 Apr 09 00:00:00 UTC
        let now_time = 1712834054000; // 2024 Apr 11 11:14:14 UTC
        let is_valid_time = is_valid_distribution_time(prev_time, now_time);
        assert_eq!(is_valid_time, true);

        // Scenario 4 - prev time is exactly the same as now time - should NOT be valid
        let prev_time = 1712834054000; // 2024 Apr 11 11:14:14 UTC
        let now_time = 1712834054000; // 2024 Apr 11 11:14:14 UTC
        let is_valid_time = is_valid_distribution_time(prev_time, now_time);
        assert_eq!(is_valid_time, false);
    }
}
