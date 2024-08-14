use crate::state::{ mutate_state, read_state };
use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::{ error, info };
use crate::utils::calculate_burn_amount;
use candid::Principal;
use candid::Nat;

pub fn start_job() {
    let burn_interval = read_state(|s| s.data.burn_config.burn_interval);
    run_now_then_interval(burn_interval, run);
}

// TODO Add retry here
pub fn run() {
    ic_cdk::spawn(process_token_burn());
}

#[trace]
pub async fn process_token_burn() {
    let burn_config = read_state(|s| s.data.burn_config.clone());

    let gldgov_ledger_canister_id = read_state(|s| s.data.gldgov_ledger_canister_id);

    // Obtain the canister's GLDGov balance
    match get_token_balance(gldgov_ledger_canister_id).await {
        Ok(available_amount) => {
            // Safely calculate the amount to be burned
            let amount_to_burn = calculate_burn_amount(available_amount, burn_config.burn_rate);

            let min_icp_burn_amount: u128 = burn_config.min_icp_burn_amount.e8s().into();

            // Check if the amount to burn is above the minimum threshold
            if amount_to_burn < min_icp_burn_amount {
                match
                    burn_tokens(
                        gldgov_ledger_canister_id,
                        burn_config.burn_address.into(),
                        amount_to_burn
                    ).await
                {
                    Ok(_) => {
                        info!("SUCCESS: {} GLDGov tokens burned from the reserve pool.", amount_to_burn);
                        mutate_state(
                            |s| {
                                // FIXME: update the last burn timestamp or other state changes
                                // s.data.last_burn_time = Some(current_time_ms());
                            }
                        );
                    }
                    Err(e) => {
                        error!(
                            "ERROR: Failed to transfer GLDGov tokens from the reserve pool to the minting account: {:?}",
                            e
                        );
                    }
                }
            }
        }
        Err(e) => {
            let error_message = format!(
                "Failed to fetch token balance of the current canister {} from ledger canister ID {}: {:?}",
                ic_cdk::api::id(),
                gldgov_ledger_canister_id,
                e
            );
            error!("{}", error_message);
            return;
        }
    }
}

async fn get_token_balance(ledger_id: Principal) -> Result<Nat, String> {
    icrc_ledger_canister_c2c_client
        ::icrc1_balance_of(
            ledger_id,
            &(Account {
                owner: ic_cdk::api::id(),
                subaccount: None,
            })
        ).await
        .map_err(|e| format!("Failed to fetch token balance: {:?}", e))
}

async fn burn_tokens(
    ledger_id: Principal,
    burn_address: Account,
    amount: u128
) -> Result<(), String> {
    let args = TransferArg {
        from_subaccount: None,
        to: burn_address,
        fee: None,
        created_at_time: None,
        amount: amount.into(),
        memo: None,
    };

    // Match the result of the transfer operation
    match icrc_ledger_canister_c2c_client::icrc1_transfer(ledger_id, &args).await {
        Ok(Ok(_)) => Ok(()), // Successful transfer
        Ok(Err(transfer_error)) => Err(format!("Failed to transfer tokens: {:?}", transfer_error)), // Transfer error
        Err(e) => Err(format!("Failed to send transfer request: {:?}", e)), // Communication or other errors
    }
}
