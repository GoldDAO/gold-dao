use candid::Nat;
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;
use tracing::error;

pub const RETRY_DELAY: Duration = Duration::from_secs(5 * 60); // each 5 minutes

pub async fn get_token_balance(ledger_id: Principal) -> Result<Nat, String> {
    icrc_ledger_canister_c2c_client::icrc1_balance_of(
        ledger_id,
        &(Account {
            owner: ic_cdk::api::canister_self(),
            subaccount: None,
        }),
    )
    .await
    .map_err(|e| format!("Failed to fetch token balance: {:?}", e))
}

pub async fn retry_with_attempts<F, Fut>(
    max_attempts: u8,
    _delay_duration: Duration,
    mut f: F,
) -> Result<(), String>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<(), String>>,
{
    for attempt in 1..=max_attempts {
        match f().await {
            Ok(_) => {
                return Ok(());
            }
            Err(err) => {
                error!("Attempt {}: Error - {:?}", attempt, err);
                if attempt == max_attempts {
                    return Err(err);
                }
            }
        }
    }
    Ok(())
}
