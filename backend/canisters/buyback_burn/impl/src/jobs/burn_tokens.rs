use crate::state::{ mutate_state, read_state };
use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::{ error, info };
use crate::utils::calculate_burn_amount;
use candid::Principal;
use candid::Nat;
use crate::utils::retry_with_attempts;
use crate::utils::RETRY_DELAY;

const MAX_ATTEMPTS: u8 = 3;

pub fn start_job() {
    let burn_interval = read_state(|s| s.data.burn_config.burn_interval);
    run_now_then_interval(burn_interval, run);
}

// TODO Add retry here
pub fn run() {
    ic_cdk::spawn(run_async());
}

async fn run_async() {
    if
        let Err(err) = retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, || async {
            process_token_burn().await
        }).await
    {
        error!("Failed to swap tokens after {} attempts: {:?}", MAX_ATTEMPTS, err);
    }
}

#[trace]
pub async fn process_token_burn() -> Result<(), String> {
    // Retrieve the burn configuration and ledger canister ID from the state
    let burn_config = read_state(|s| s.data.burn_config.clone());
    let gldgov_ledger_canister_id = read_state(|s| s.data.gldgov_ledger_canister_id);

    // Fetch the canister's GLDGov token balance
    let available_amount = get_token_balance(gldgov_ledger_canister_id).await?;

    // Calculate the amount of tokens to burn
    let amount_to_burn = calculate_burn_amount(available_amount, burn_config.burn_rate);

    // Minimum burn amount in ICP tokens (converted from e8s)
    let min_icp_burn_amount: u128 = burn_config.min_icp_burn_amount.e8s().into();

    // Check if the amount to burn is above the minimum threshold
    if amount_to_burn < min_icp_burn_amount {
        // Attempt to burn the calculated amount of tokens
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
                Ok(())
            }
            Err(e) => {
                let error_message = format!(
                    "ERROR: Failed to burn GLDGov tokens from the reserve pool: {:?}",
                    e
                );
                error!("{}", error_message);
                Err(error_message)
            }
        }
    } else {
        // Log an error if the amount to burn is below the threshold
        let error_message = format!(
            "ERROR: Calculated burn amount {} is below the minimum threshold of {}.",
            amount_to_burn,
            min_icp_burn_amount
        );
        error!("{}", error_message);
        Err(error_message)
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
