use crate::guards::GuardExchangeJob;
use crate::state::{mutate_state, read_state};
use crate::types::ExchangeJob;
use crate::types::{SwapClient, SwapClientEnum, TokenSwap};
use crate::utils::run_now_then_interval_with_args;
use crate::utils::{get_token_balance, retry_with_attempts, RETRY_DELAY};
use bity_ic_canister_time::NANOS_PER_MILLISECOND;
use bity_ic_canister_tracing_macros::trace;
use candid::Nat;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::debug;
use tracing::{error, info};
use utils::env::Environment;
use utils::rand::generate_random_delay;

const MAX_ATTEMPTS: u8 = 1;

pub const MEMO_SWAP: [u8; 7] = [0x4f, 0x43, 0x5f, 0x53, 0x57, 0x41, 0x50]; // OC_SWAP

pub fn start_job() {
    // We only need the keys and the intervals to initialize the timers
    let exchange_jobs = read_state(|s| s.data.exchange_jobs.exchange_jobs.clone());

    for (job_id, exchange_job) in exchange_jobs {
        info!(
            "Starting token swap job ID: {:?} for exchange: {:?} with interval: {:?}",
            job_id, exchange_job.exchange, exchange_job.job_interval
        );

        let interval = exchange_job.job_interval;

        let timer_id = run_now_then_interval_with_args(interval, move || {
            run(job_id);
        });

        mutate_state(|state| {
            if let Some(job) = state.data.exchange_jobs.exchange_jobs.get_mut(&job_id) {
                job.timer_id = Some(timer_id);
            }
            info!("Timer saved into state for job ID: {:?}", job_id);
        });
    }
}

pub fn run(exchange_job_id: u128) {
    ic_cdk::futures::spawn(run_async_with_rand_delay(exchange_job_id));
}

#[trace]
async fn run_async_with_rand_delay(exchange_job_id: u128) {
    let interval = read_state(|s| {
        s.data
            .exchange_jobs
            .exchange_jobs
            .get(&exchange_job_id)
            .map(|j| j.job_interval)
    });

    if let Some(job_interval) = interval {
        match generate_random_delay(job_interval).await {
            Ok(random_delay) => {
                debug!(
                    "Scheduling token swap job after random delay of {:?}",
                    random_delay
                );
                ic_cdk_timers::set_timer(random_delay, move || {
                    ic_cdk::futures::spawn(run_async(exchange_job_id))
                });
            }
            Err(e) => error!(
                "Failed to generate random delay for job {}: {}",
                exchange_job_id, e
            ),
        }
    }
}

#[trace]
async fn run_async(exchange_job_id: u128) {
    let exchange_job = read_state(|state| {
        state
            .data
            .exchange_jobs
            .exchange_jobs
            .get(&exchange_job_id)
            .cloned()
            .expect("Exchange job not found")
    });

    info!(
        "Running token swap job for exchange: {:?}",
        exchange_job.exchange
    );

    if let Some((future, token_swap_id)) = create_token_swap_if_possible(exchange_job.clone()).await
    {
        if future.await.is_ok() {
            let _ = mutate_state(|state| state.data.token_swaps.archive_swap(token_swap_id));

            if let Err(e) = transfer_to_destination(&exchange_job).await {
                error!("Failed to transfer to destination: {}", e);
            }
        } else {
            error!("Failed to process token swap for job {:?}", exchange_job);
        }
    };
}

async fn transfer_to_destination(exchange_job: &ExchangeJob) -> Result<Nat, String> {
    let Some(destination_account) = exchange_job.destination_account else {
        return Ok(Nat::from(0_u64));
    };

    let output_token_info = exchange_job
        .exchange
        .get_config()
        .output_token
        .get_prod_token_info();
    let available_amount = get_token_balance(output_token_info.ledger_id, None)
        .await
        .map_err(|e| format!("Error calculating available amount: {}", e))?;

    // Ensure balance covers the fee
    if available_amount <= output_token_info.fee {
        let err_string = format!(
            "Balance ({}) is too low to cover fee ({}) for destination transfer.",
            available_amount, output_token_info.fee
        );
        info!("{}", err_string);
        return Err(err_string);
    }

    let amount_to_send = available_amount.clone() - output_token_info.fee.clone();

    let now = read_state(|state| state.env.now());
    let transfer_result = match icrc_ledger_canister_c2c_client::icrc1_transfer(
        output_token_info.ledger_id,
        &(TransferArg {
            from_subaccount: Default::default(),
            to: destination_account,
            fee: Some(output_token_info.fee.into()),
            created_at_time: Some(now * NANOS_PER_MILLISECOND),
            memo: Some(MEMO_SWAP.to_vec().into()),
            amount: amount_to_send,
        }),
    )
    .await
    {
        Ok(Ok(index)) => Ok(index),
        Ok(Err(error)) => {
            error!("Ledger error transferring to destination: {:?}", error);
            Err(format!("{:?}", error))
        }
        Err(error) => {
            error!(
                "Canister call error transferring to destination: {:?}",
                error
            );
            Err(format!("{:?}", error))
        }
    };

    transfer_result
}

async fn create_token_swap_if_possible(
    exchange_job: ExchangeJob,
) -> Option<(impl std::future::Future<Output = Result<(), String>>, u128)> {
    let _guard_exchange_job =
        match GuardExchangeJob::new(exchange_job.exchange.get_swap_client_id()) {
            Ok(_guard_exchange_job) => _guard_exchange_job,
            Err(e) => {
                // Guard is already held → retry later
                error!(
                    "Exchange job is already being processed, retrying later: {}",
                    e
                );

                let retry_job = exchange_job.clone();
                ic_cdk_timers::set_timer(RETRY_DELAY, move || {
                    ic_cdk::futures::spawn(async move {
                        let _ = create_token_swap_if_possible(retry_job).await;
                    });
                });

                return None;
            }
        };

    let args = exchange_job.exchange.get_config();
    let swap_client = exchange_job.exchange.clone();
    let input_token_info = args.input_token.get_prod_token_info();
    let output_token_info = args.output_token.get_prod_token_info();

    let available_amount =
        match get_token_balance(input_token_info.ledger_id, exchange_job.source_subaccount).await {
            Ok(amount) => amount,
            Err(e) => {
                error!("Error calculating burn amount: {}", e);
                return None;
            }
        };

    let amount_to_dex =
        u128::try_from(exchange_job.rate_per_interval.apply_to(&available_amount).0)
            .expect("Failed to convert Nat");

    let quote = get_swap_quote(&swap_client, amount_to_dex, input_token_info.fee).await;
    let min_required = (exchange_job.min_amount.e8s() as u128) + (output_token_info.fee as u128);

    if let Some(max_amount) = exchange_job.max_amount {
        let max_allowed = max_amount.e8s() as u128;
        if quote > max_allowed {
            error!(
                "Swap amount out of bounds: quote = {:?}, max = {:?}",
                quote, max_allowed
            );
            return None;
        }
    }

    if quote >= min_required {
        let token_swap = mutate_state(|state| {
            state
                .data
                .token_swaps
                .push_new(args.clone(), state.env.now())
        });
        let swap_id = token_swap.swap_id;
        let future = retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, move || {
            process_token_swap(exchange_job.clone(), token_swap.clone(), amount_to_dex)
        });
        Some((future, swap_id))
    } else {
        error!(
            "Swap amount out of bounds: quote = {:?}, min = {:?}",
            quote, min_required
        );
        None
    }
}

async fn get_swap_quote(swap_client: &SwapClientEnum, amount_to_dex: u128, fee: u64) -> u128 {
    if amount_to_dex <= fee as u128 {
        error!(
            "Amount too small for swap: amount={}, fee={}",
            amount_to_dex, fee
        );
        return 0;
    }

    match swap_client
        .get_quote(amount_to_dex.saturating_sub(fee.into()), 0)
        .await
    {
        Ok(Ok(quote)) => quote,
        Ok(Err(dex_err)) => {
            error!(
                "DEX rejected quote request: amount_to_dex={}, err={:?}",
                amount_to_dex, dex_err
            );
            0
        }
        Err(call_err) => {
            error!(
                "Failed to call get_quote: amount_to_dex={}, err={:?}",
                amount_to_dex, call_err
            );
            0
        }
    }
}

pub(crate) async fn process_token_swap(
    exchange_job: ExchangeJob,
    mut token_swap: TokenSwap,
    amount_to_dex: u128,
) -> Result<(), String> {
    let swap_client = exchange_job.exchange.clone();
    let swap_config = swap_client.get_config();
    let input_token_info = swap_config.input_token.get_prod_token_info();
    let output_token_info = swap_config.output_token.get_prod_token_info();
    let min_output_amount = exchange_job.min_amount.e8s() as u128;

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
            input_token_info.ledger_id,
            &(TransferArg {
                from_subaccount: exchange_job.source_subaccount,
                to: account,
                fee: Some(input_token_info.fee.into()),
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
                amount_to_dex.saturating_sub(input_token_info.fee.into()),
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
            amount_swapped.saturating_sub(output_token_info.fee.into()),
        )
    } else {
        (
            false,
            amount_to_dex.saturating_sub(input_token_info.fee.into()),
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
