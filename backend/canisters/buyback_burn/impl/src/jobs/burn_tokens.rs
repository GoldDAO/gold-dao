use crate::state::read_state;
use crate::utils::{ get_token_balance, RETRY_DELAY };
use candid::{ Nat, Principal };
use canister_tracing_macros::trace;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::error;
use crate::utils::retry_with_attempts;

const MAX_ATTEMPTS: u8 = 1;

pub fn run() {
    error!("The job started");
    ic_cdk::spawn(run_async());
}

#[trace]
async fn run_async() {
    if
        let Err(err) = retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, || async {
            process_token_burn().await
        }).await
    {
        error!("Failed to swap tokens after {} attempts: {:?}", MAX_ATTEMPTS, err);
    }
}

pub async fn process_token_burn() -> Result<(), String> {
    let burn_config = read_state(|s| s.data.burn_config.clone());
    let gldgov_ledger_canister_id = read_state(|s| s.data.gldgov_token_info.ledger_id);

    let amount_to_burn = get_token_balance(gldgov_ledger_canister_id).await?;
    let min_burn_amount: u128 = burn_config.min_burn_amount.e8s().into();

    if amount_to_burn > min_burn_amount {
        let minting_account = match
            icrc_ledger_canister_c2c_client::icrc1_minting_account(gldgov_ledger_canister_id).await
        {
            Ok(account) => {
                match account {
                    Some(a) => a,
                    None => {
                        return Err("Minting account is None".to_string());
                    }
                }
            }
            Err(e) => {
                return Err(
                    format!("Failed to get minting account (in order to burn tokens): {:?}", e)
                );
            }
        };

        match burn_tokens(gldgov_ledger_canister_id, minting_account, amount_to_burn.clone()).await {
            Ok(_) => { Ok(()) }
            Err(e) => {
                let error_message = format!(
                    "Failed to burn GLDGov tokens from the buyback and burn canister: {:?}",
                    e
                );
                error!("{}", error_message);
                Err(error_message)
            }
        }
    } else {
        let error_message = format!(
            "Calculated burn amount {} is below the minimum threshold of {}.",
            amount_to_burn,
            min_burn_amount
        );
        error!("{}", error_message);
        Err(error_message)
    }
}

async fn burn_tokens(
    ledger_id: Principal,
    burn_address: Account,
    amount: Nat
) -> Result<(), String> {
    let args = TransferArg {
        from_subaccount: None,
        to: burn_address,
        // NOTE: There should be no fees for burnings
        fee: None,
        created_at_time: None,
        amount,
        memo: None,
    };

    match icrc_ledger_canister_c2c_client::icrc1_transfer(ledger_id, &args).await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(transfer_error)) => Err(format!("Failed to transfer tokens: {:?}", transfer_error)),
        Err(e) => Err(format!("Failed to send transfer request: {:?}", e)),
    }
}
