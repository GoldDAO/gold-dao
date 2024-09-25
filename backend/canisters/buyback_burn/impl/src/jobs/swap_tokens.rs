use crate::state::{ mutate_state, read_state };
use crate::types::SwapClient;
use crate::utils::{
    calculate_percentage_of_amount,
    get_token_balance,
    retry_with_attempts,
    RETRY_DELAY,
};
use utils::rand::generate_random_delay;
use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use futures::future::join_all;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::{ debug, error, info };
use utils::env::Environment;
use crate::types::TokenSwap;

use canister_time::NANOS_PER_MILLISECOND;
const MAX_ATTEMPTS: u8 = 1;

pub const MEMO_SWAP: [u8; 7] = [0x4f, 0x43, 0x5f, 0x53, 0x57, 0x41, 0x50]; // OC_SWAP

pub fn start_job() {
    let swap_interval = read_state(|s| s.data.swap_interval);
    run_now_then_interval(swap_interval, run);
}

pub fn run() {
    ic_cdk::spawn(run_async_with_rand_delay());
}

#[trace]
async fn run_async_with_rand_delay() {
    let swap_interval = read_state(|s| s.data.swap_interval);

    match generate_random_delay(swap_interval).await {
        Ok(random_delay) => {
            ic_cdk_timers::set_timer(random_delay, || ic_cdk::spawn(run_async()));
        }
        Err(e) => {
            error!("Failed to generate random delay: {}", e);
        }
    }
}

#[trace]
async fn run_async() {
    let swap_clients = read_state(|state| state.data.swap_clients.clone());
    let mut token_swap_ids = Vec::new();

    // TODO: check that everything here is correct
    let futures: Vec<_> = swap_clients
        .iter()
        .map(|swap_client| {
            let args = swap_client.get_config();
            let token_swap = mutate_state(|state|
                state.data.token_swaps.push_new(args, state.env.now())
            );
            token_swap_ids.push(token_swap.swap_id);

            async move {
                retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, || async {
                    process_token_swap(swap_client.as_ref(), token_swap.clone()).await
                }).await
            }
        })
        .collect();

    let results = join_all(futures).await;

    let mut error_messages = Vec::new();
    for result in results {
        if let Err(e) = result {
            error_messages.push(e);
        }
    }

    if error_messages.is_empty() {
        info!("Successfully processed all token swaps");
        for token_swap_id in token_swap_ids {
            let _ = mutate_state(|state| state.data.token_swaps.archive_swap(token_swap_id));
        }

        // NOTE: added burning tokens
        crate::jobs::burn_tokens::run();
    } else {
        error!("Failed to process some token swaps:\n{}", error_messages.join("\n"));
    }
}

pub(crate) async fn process_token_swap(
    swap_client: &dyn SwapClient,
    mut token_swap: TokenSwap
) -> Result<(), String> {
    let swap_config = swap_client.get_config();

    let burn_rate = read_state(|s| s.data.burn_config.burn_rate);

    let available_amount = get_token_balance(swap_config.input_token.ledger_id).await?;

    // FIXME: add here minimum amount to swap
    let input_amount = calculate_percentage_of_amount(available_amount, burn_rate);
    debug!("input_amount: {}", input_amount);
    let amount_to_dex = input_amount.saturating_sub(swap_config.input_token.fee.into());
    debug!("amount_to_dex: {}", amount_to_dex);

    if amount_to_dex == 0 {
        return Err("Insufficient balance to swap".to_string());
    }

    // NOTE: Should we use this parameter? We can try to also store the minimum ICP/GLDGov
    // price and then calculate the min_output_amount
    let min_output_amount = 0;

    // Get the deposit account
    let account = if let Some(a) = extract_result(&token_swap.deposit_account) {
        *a
    } else {
        match swap_client.deposit_account().await {
            Ok(a) => {
                mutate_state(|state| {
                    token_swap.deposit_account = Some(Ok(a));
                    state.data.token_swaps.upsert(token_swap.clone());
                });
                a
            }
            Err(error) => {
                let msg = format!("{error:?}");
                mutate_state(|state| {
                    token_swap.deposit_account = Some(Err(msg.clone()));
                    token_swap.success = Some(false);
                    state.data.token_swaps.upsert(token_swap);
                });
                error!("Failed to deposit tokens while swap: {}", msg.as_str());
                return Err(msg);
            }
        }
    };

    // Deposit tokens to the deposit account
    if extract_result(&token_swap.transfer).is_none() {
        let now = read_state(|state| state.env.now());
        let transfer_result = match
            icrc_ledger_canister_c2c_client::icrc1_transfer(
                swap_config.input_token.ledger_id,
                &(TransferArg {
                    from_subaccount: None,
                    to: account,
                    fee: Some(swap_config.input_token.fee.into()),
                    created_at_time: Some(now * NANOS_PER_MILLISECOND),
                    memo: Some(MEMO_SWAP.to_vec().into()),
                    amount: amount_to_dex.into(),
                })
            ).await
        {
            Ok(Ok(index)) => Ok(index),
            Ok(Err(error)) => Err(format!("{error:?}")),
            Err(error) => Err(format!("{error:?}")),
        };

        match transfer_result {
            Ok(index) => {
                mutate_state(|state| {
                    token_swap.transfer = Some(Ok(index.0.try_into().unwrap()));
                    state.data.token_swaps.upsert(token_swap.clone());
                });
            }
            Err(msg) => {
                mutate_state(|state| {
                    token_swap.transfer = Some(Err(msg.clone()));
                    token_swap.success = Some(false);
                    state.data.token_swaps.upsert(token_swap);
                });
                error!("Failed to transfer tokens: {}", msg.as_str());
                return Err(msg);
            }
        }
    }

    // Notify DEX
    if extract_result(&token_swap.notified_dex_at).is_none() {
        if let Err(error) = swap_client.deposit(amount_to_dex).await {
            let msg = format!("{error:?}");
            mutate_state(|state| {
                token_swap.notified_dex_at = Some(Err(msg.clone()));
                state.data.token_swaps.upsert(token_swap.clone());
            });
            error!("Failed to deposit tokens: {}", msg.as_str());
            return Err(msg);
        } else {
            mutate_state(|state| {
                token_swap.notified_dex_at = Some(Ok(()));
                state.data.token_swaps.upsert(token_swap.clone());
            });
        }
    }

    // Swap the tokens
    let swap_result = if let Some(a) = extract_result(&token_swap.amount_swapped).cloned() {
        a
    } else {
        match
            swap_client.swap(
                amount_to_dex.saturating_sub(swap_config.input_token.fee.into()),
                min_output_amount
            ).await
        {
            Ok(a) => {
                mutate_state(|state| {
                    token_swap.amount_swapped = Some(Ok(a.clone()));
                    state.data.token_swaps.upsert(token_swap.clone());
                });
                a
            }
            Err(error) => {
                let msg = format!("{error:?}");
                mutate_state(|state| {
                    token_swap.amount_swapped = Some(Err(msg.clone()));
                    state.data.token_swaps.upsert(token_swap.clone());
                });
                error!("Failed to swap tokens: {}", msg.as_str());
                return Err(msg);
            }
        }
    };

    let (successful_swap, amount_out) = if let Ok(amount_swapped) = swap_result {
        (true, amount_swapped.saturating_sub(swap_config.output_token.fee.into()))
    } else {
        (false, amount_to_dex.saturating_sub(swap_config.input_token.fee.into()))
    };

    // Withdraw tokens from the DEX
    if extract_result(&token_swap.withdrawn_from_dex_at).is_none() {
        if let Err(error) = swap_client.withdraw(successful_swap, amount_out).await {
            let msg = format!("{error:?}");
            mutate_state(|state| {
                token_swap.withdrawn_from_dex_at = Some(Err(msg.clone()));
                state.data.token_swaps.upsert(token_swap.clone());
            });
            error!("Failed to withdraw tokens: {}", msg.as_str());
            return Err(msg);
        } else {
            mutate_state(|state| {
                token_swap.withdrawn_from_dex_at = Some(Ok(amount_out));
                token_swap.success = Some(successful_swap);
                state.data.token_swaps.upsert(token_swap);
            });
        }
    }

    if successful_swap {
        Ok(())
    } else {
        Err("The swap failed".to_string())
    }
}

fn extract_result<T>(subtask: &Option<Result<T, String>>) -> Option<&T> {
    subtask.as_ref().and_then(|t| t.as_ref().ok())
}
