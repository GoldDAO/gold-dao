use crate::state::{mutate_state, read_state};
use crate::types::{SwapClient, SwapClientEnum, TokenSwap};
use crate::utils::{
    calculate_percentage_of_amount, get_token_balance, retry_with_attempts, RETRY_DELAY,
};
use canister_time::{run_now_then_interval, NANOS_PER_MILLISECOND, WEEK_IN_MS};
use canister_tracing_macros::trace;
use futures::future::join_all;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::{debug, error, info};
use types::TokenInfo;
use utils::env::Environment;
use utils::rand::generate_random_delay;

const MAX_ATTEMPTS: u8 = 1;

pub const MEMO_SWAP: [u8; 7] = [0x4f, 0x43, 0x5f, 0x53, 0x57, 0x41, 0x50]; // OC_SWAP

pub fn start_job() {
    let buyback_interval = read_state(|s| s.data.buyback_interval);
    if read_state(|s| s.data.burn_config.validate_burn_rate()) {
        run_now_then_interval(buyback_interval, run);
    } else {
        error!("Burn rate is invalid. The job wouldn't start");
    }
}

pub fn run() {
    ic_cdk::spawn(run_async_with_rand_delay());
}

pub fn run_now() {
    ic_cdk::spawn(run_async());
}

#[trace]
async fn run_async_with_rand_delay() {
    let buyback_interval = read_state(|s| s.data.buyback_interval);

    match generate_random_delay(buyback_interval).await {
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
    let mut futures = Vec::new();

    for swap_client in swap_clients.iter() {
        if let Some((future, token_swap_id)) =
            create_token_swap_if_possible(swap_client.clone()).await
        {
            futures.push(future);
            token_swap_ids.push(token_swap_id);
        }
    }

    let results = join_all(futures).await;

    let error_messages: Vec<String> = results.into_iter().filter_map(Result::err).collect();

    if error_messages.is_empty() {
        info!("Successfully processed all token swaps");
        for token_swap_id in token_swap_ids {
            let _ = mutate_state(|state| state.data.token_swaps.archive_swap(token_swap_id));
        }
    } else {
        error!(
            "Failed to process some token swaps:\n{}",
            error_messages.join("\n")
        );
    }
}

async fn create_token_swap_if_possible(
    swap_client: SwapClientEnum,
) -> Option<(impl std::future::Future<Output = Result<(), String>>, u128)> {
    let args = swap_client.get_config();
    let amount_to_dex = match burn_amount_per_interval(args.input_token).await {
        Ok(amount) => amount,
        Err(e) => {
            error!("Error calculating burn amount: {}", e);
            return None;
        }
    };

    let quote = get_swap_quote(&swap_client, amount_to_dex, args.input_token.fee).await;
    let min_burn_amount = read_state(|s| s.data.burn_config.min_burn_amount.e8s()) as u128;

    if quote > min_burn_amount + (args.output_token.fee as u128) {
        let token_swap = mutate_state(|state| {
            state
                .data
                .token_swaps
                .push_new(args.clone(), state.env.now())
        });
        let swap_id = token_swap.swap_id;
        let future = retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, move || {
            process_token_swap(swap_client.clone(), token_swap.clone(), amount_to_dex)
        });
        Some((future, swap_id))
    } else {
        error!(
            "Insufficient balance for swap: {:?}, min balance: {:?}",
            quote,
            min_burn_amount + (args.output_token.fee as u128)
        );
        None
    }
}

async fn get_swap_quote(swap_client: &SwapClientEnum, amount_to_dex: u128, fee: u64) -> u128 {
    match swap_client
        .get_quote(amount_to_dex.saturating_sub(fee.into()), 0)
        .await
    {
        Ok(Ok(quote)) => quote,
        _ => {
            error!("Failed to get the quote");
            0
        }
    }
}

pub(crate) async fn process_token_swap(
    swap_client: SwapClientEnum,
    mut token_swap: TokenSwap,
    amount_to_dex: u128,
) -> Result<(), String> {
    let swap_config = swap_client.get_config();

    let min_output_amount = 0;

    // Get the quote to decide whether swap or not
    let quote = match swap_client
        .get_quote(
            amount_to_dex.saturating_sub(swap_config.input_token.fee.into()),
            min_output_amount,
        )
        .await
    {
        Ok(quote) => match quote {
            Ok(q) => q,
            Err(error) => {
                let msg = format!("{error:?}");
                error!("Failed to get the quote: {}", msg.as_str());
                return Err(msg);
            }
        },
        Err(error) => {
            let msg = format!("{error:?}");
            error!("Failed to get the quote: {}", msg.as_str());
            return Err(msg);
        }
    };

    // NOTE: check if it makes sense to make swap (especially if there would be enough balance after the swap)
    let min_burn_amount = read_state(|s| s.data.burn_config.min_burn_amount.e8s()) as u128;
    if quote < min_burn_amount + (swap_config.output_token.fee as u128) {
        let msg = format!("Insufficient balance to swap: {:?}", quote);
        mutate_state(|state| {
            token_swap.deposit_account = Some(Err(msg.clone()));
            token_swap.success = Some(false);
            state.data.token_swaps.upsert(token_swap);
        });
        error!(msg);
        return Err(msg);
    }

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
        let transfer_result = match icrc_ledger_canister_c2c_client::icrc1_transfer(
            swap_config.input_token.ledger_id,
            &(TransferArg {
                from_subaccount: None,
                to: account,
                fee: Some(swap_config.input_token.fee.into()),
                created_at_time: Some(now * NANOS_PER_MILLISECOND),
                memo: Some(MEMO_SWAP.to_vec().into()),
                amount: amount_to_dex.into(),
            }),
        )
        .await
        {
            Ok(Ok(index)) => Ok(index),
            Ok(Err(error)) => {
                error!("Failed to deposit tokens to deposit account: {:?}", error);
                Err(format!("{error:?}"))
            }
            Err(error) => {
                error!("Failed to deposit tokens to deposit account: {:?}", error);
                Err(format!("{error:?}"))
            }
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
        match swap_client
            .swap(
                amount_to_dex.saturating_sub(swap_config.input_token.fee.into()),
                min_output_amount,
            )
            .await
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
        (
            true,
            amount_swapped.saturating_sub(swap_config.output_token.fee.into()),
        )
    } else {
        (
            false,
            amount_to_dex.saturating_sub(swap_config.input_token.fee.into()),
        )
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

pub async fn burn_amount_per_interval(input_token: TokenInfo) -> Result<u128, String> {
    if let Ok(available_amount) = get_token_balance(input_token.ledger_id).await {
        let burn_rate = read_state(|s| s.data.burn_config.burn_rate);
        let amount_per_week = calculate_percentage_of_amount(available_amount, burn_rate);
        debug!("amount_per_week: {}", amount_per_week);

        let buyback_interval = read_state(|s| s.data.buyback_interval);
        let times = (WEEK_IN_MS as u128) / buyback_interval.as_millis();

        let amount_per_interval = (amount_per_week / times).saturating_sub(input_token.fee.into());
        debug!("amount_per_interval: {}", amount_per_interval);
        Ok(amount_per_interval)
    } else {
        Err("Failed to get token balance".to_string())
    }
}
