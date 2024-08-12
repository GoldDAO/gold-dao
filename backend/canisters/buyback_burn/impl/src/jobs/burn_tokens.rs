use crate::state::{ mutate_state, read_state };
use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use icrc_ledger_types::icrc1::account::{ Account, Subaccount };
use icrc_ledger_types::icrc1::transfer::TransferArg;
use std::time::Duration;
use tracing::debug;
use tracing::error;
use tracing::info;
use types::TokenInfo;
use crate::utils::fetch_balance_of_sub_account;
use ic_ledger_types::Tokens;

pub const BUYBACK_BURN_SUB_ACCOUNT: Subaccount = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const INTERVAL: Duration = Duration::from_secs(7 * 24 * 60 * 60); // 1 week

pub fn start_job() {
    run_now_then_interval(INTERVAL, run);
}

pub fn run() {}

pub const SUB_ACCOUNT: Subaccount = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[trace]
pub async fn process_token_burn() {
    // TODO: Delete extra clone here
    let burn_config = read_state(|s| s.data.burn_config.clone());

    // Get the minting account in order to burn tokens (sending the to minting account is equivalent to burning)
    let minting_account = Account {
        owner: burn_config.burn_address,
        subaccount: None,
    };

    let gldgov_ledger_canister_id = read_state(|s| s.data.gldgov_ledger_canister_id);

    // Specify GLDGov token details
    let gldgov_token_info = TokenInfo {
        ledger_id: gldgov_ledger_canister_id,
        fee: 100_000u64,
        decimals: 8u64,
    };

    // check the reserve pool has enough GLDGov to correctly transfer ( burn )
    match fetch_balance_of_sub_account(gldgov_token_info.ledger_id, SUB_ACCOUNT).await {
        Ok(balance) => {
            // TODO fix here in future. There should be no problem with converting u64 into u128.
            let total_to_burn =
                burn_config.min_icp_burn_amount + Tokens::from_e8s(gldgov_token_info.fee);
            if balance < total_to_burn.e8s() {
                debug!(
                    "Balance of reserve pool : {} is too low to make a burn of {} plus a fee of {} ",
                    balance,
                    burn_config.min_icp_burn_amount,
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

    // Send tokens to the minting account in order to burn them
    let args = TransferArg {
        from_subaccount: Some(BUYBACK_BURN_SUB_ACCOUNT),
        to: minting_account,
        fee: None,
        created_at_time: None,
        amount: burn_config.min_icp_burn_amount.e8s().into(),
        memo: None,
    };

    match icrc_ledger_canister_c2c_client::icrc1_transfer(burn_config.burn_address, &args).await {
        Ok(_) => {
            info!(
                "SUCCESS : {:?} GLDGov tokens burned from reserve pool",
                burn_config.min_icp_burn_amount
            );
            mutate_state(
                |s| {
                    // s.data.last_daily_gldgov_burn = Some(current_time_ms);
                }
            )
        }
        Err(e) => {
            error!(
                "ERROR : GLDGov failed to transfer from reserve pool to GLDGov minting account with error : {:?}",
                e
            );
        }
    }
}
