use crate::state::read_state;
use crate::utils::{get_token_balance, retry_with_attempts, RETRY_DELAY};
use candid::{Nat, Principal};
use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::{error, info};

const MAX_ATTEMPTS: u8 = 3;

pub fn start_job() {
    let burn_interval = read_state(|s| s.data.burn_config.burn_interval);
    run_now_then_interval(burn_interval, run);
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

#[trace]
async fn run_async() {
    if let Err(err) = retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, || async {
        process_token_burn().await
    })
    .await
    {
        error!(
            "Failed to swap tokens after {} attempts: {:?}",
            MAX_ATTEMPTS, err
        );
    }
}
pub async fn process_token_burn() -> Result<(), String> {
    // Retrieve the burn configuration and ledger canister ID from the state
    let burn_config = read_state(|s| s.data.burn_config.clone());
    let gldgov_ledger_canister_id = read_state(|s| s.data.gldgov_ledger_canister_id);

    // Fetch the canister's GLDGov token balance
    let amount_to_burn = get_token_balance(gldgov_ledger_canister_id).await?;

    // Minimum burn amount in ICP tokens (converted from e8s)
    let min_burn_amount: u128 = burn_config.min_burn_amount.e8s().into();

    // Check if the amount to burn is above the minimum threshold
    if amount_to_burn < min_burn_amount {
        // Attempt to burn the calculated amount of tokens
        match burn_tokens(
            gldgov_ledger_canister_id,
            burn_config.burn_address.into(),
            amount_to_burn.clone(),
        )
        .await
        {
            Ok(_) => {
                info!(
                    "SUCCESS: {} GLDGov tokens burned from the buyback and burn canister.",
                    amount_to_burn
                );
                Ok(())
            }
            Err(e) => {
                let error_message = format!(
                    "ERROR: Failed to burn GLDGov tokens from the buyback and burn canister: {:?}",
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
            amount_to_burn, min_burn_amount
        );
        error!("{}", error_message);
        Err(error_message)
    }
}

async fn burn_tokens(
    ledger_id: Principal,
    burn_address: Account,
    amount: Nat,
) -> Result<(), String> {
    let args = TransferArg {
        from_subaccount: None,
        to: burn_address,
        // NOTE: There should be no fees for burnings
        fee: None,
        created_at_time: None,
        amount: amount,
        memo: None,
    };

    match icrc_ledger_canister_c2c_client::icrc1_transfer(ledger_id, &args).await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(transfer_error)) => Err(format!("Failed to transfer tokens: {:?}", transfer_error)),
        Err(e) => Err(format!("Failed to send transfer request: {:?}", e)),
    }
}
