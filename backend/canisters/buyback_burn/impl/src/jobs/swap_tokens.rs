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
use std::time::Duration;
use tracing::{error, info};
use utils::env::Environment;
use utils::rand::generate_random_delay;

const MAX_ATTEMPTS: u8 = 1;

pub const MEMO_SWAP: [u8; 7] = [0x4f, 0x43, 0x5f, 0x53, 0x57, 0x41, 0x50]; // OC_SWAP

pub fn start_job() {
    let unconstrained_jobs = read_state(|s| {
        let jobs: Vec<_> = s
            .data
            .exchange_jobs
            .exchange_jobs
            .values()
            .filter(|j| j.constraints.is_empty())
            .cloned()
            .collect();
        jobs
    });

    // Step 1: start one independent timer per unconstrained job (GLDT)
    for job in unconstrained_jobs {
        let job_id = job.id;
        info!(
            "Starting unconstrained swap job ID: {} for exchange: {:?} with interval: {:?}",
            job_id, job.exchange, job.job_interval
        );
        let timer_id = run_now_then_interval_with_args(job.job_interval, move || run(job_id));
        mutate_state(|s| {
            if let Some(j) = s.data.exchange_jobs.exchange_jobs.get_mut(&job_id) {
                j.timer_id = Some(timer_id);
            }
        });
    }

    // Steps 2 & 3: single timer that checks constrained jobs in priority order,
    // falling back to staking ICP if none of their quotes pass.
    let constrained_jobs_interval = read_state(|s| {
        s.data
            .stake_icp_config
            .as_ref()
            .map(|c| Duration::from_millis(c.job_interval_ms))
    });
    let Some(interval) = constrained_jobs_interval else {
        info!("No StakeIcpConfig found, skipping constrained swap + stake_icp job.");
        return;
    };
    run_now_then_interval_with_args(interval, || {
        ic_cdk::futures::spawn(run_constrained_jobs_then_stake_icp())
    });
}

async fn run_constrained_jobs_then_stake_icp() {
    let interval = read_state(|s| {
        s.data
            .stake_icp_config
            .as_ref()
            .map(|c| Duration::from_millis(c.job_interval_ms))
    });
    let Some(interval) = interval else { return };

    let delay = match generate_random_delay(interval).await {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to generate random delay: {}", e);
            return;
        }
    };

    ic_cdk_timers::set_timer(delay, || {
        ic_cdk::futures::spawn(async {
            // run constrained jobs in ascending ID order (smaller ID = higher priority).
            let mut constrained_jobs: Vec<ExchangeJob> = read_state(|s| {
                s.data
                    .exchange_jobs
                    .exchange_jobs
                    .values()
                    .filter(|j| !j.constraints.is_empty())
                    .cloned()
                    .collect()
            });
            constrained_jobs.sort_by_key(|j| j.id);

            for job in constrained_jobs {
                let input_token_info = job.exchange.get_config().input_token.get_prod_token_info();
                let nominal_sell: u128 = 100_000_000; // 1 ICP in e8s — ratio check only
                let quote = get_swap_quote(&job.exchange, nominal_sell, input_token_info.fee).await;

                if job
                    .constraints
                    .iter()
                    .all(|c| c.check_quote(nominal_sell, quote))
                {
                    info!(
                        "Constrained job {} quote passed (sell={}, buy={}), running swap.",
                        job.id, nominal_sell, quote
                    );
                    run_swap_job(job).await;
                    return; // exclusive: skip lower-priority jobs and stake ICP
                }

                info!(
                    "Constrained job {} quote not satisfied (sell={}, buy={}), trying next.",
                    job.id, nominal_sell, quote
                );
            }

            // Step 3: no constrained job ran — stake ICP as fallback.
            info!("No constrained swap conditions met, staking ICP.");
            crate::jobs::stake_icp::stake_icp().await;
        })
    });
}

async fn run_swap_job(job: ExchangeJob) {
    if let Some((future, token_swap_id)) = create_token_swap_if_possible(job.clone()).await {
        if future.await.is_ok() {
            let _ = mutate_state(|s| s.data.token_swaps.archive_swap(token_swap_id));
            if let Err(e) = transfer_to_destination(&job).await {
                error!("Failed to transfer to destination: {}", e);
            }
        } else {
            error!("Failed to process token swap for job {:?}", job);
        }
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
        "Running unconstrained swap job for exchange: {:?}",
        exchange_job.exchange
    );
    run_swap_job(exchange_job).await;
}

pub(crate) async fn transfer_to_destination(exchange_job: &ExchangeJob) -> Result<Nat, String> {
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
    }?;

    if let Some(action) = &exchange_job.post_transfer_action {
        if let Err(e) = action.execute_post_transfer_action().await {
            error!("Post-transfer action failed: {}", e);
        }
    }

    Ok(transfer_result)
}

pub(crate) async fn create_token_swap_if_possible(
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

    if amount_to_dex <= input_token_info.fee as u128 {
        error!(
            "Amount to swap ({}) is too small to cover fee ({}) for job {}, skipping.",
            amount_to_dex, input_token_info.fee, exchange_job.id
        );
        return None;
    }

    let amount_to_dex = amount_to_dex - input_token_info.fee as u128;
    info!(
        "Amount to swap for job {}: {}",
        exchange_job.id, amount_to_dex
    );

    let quote = get_swap_quote(&swap_client, amount_to_dex, input_token_info.fee).await;

    for constraint in &exchange_job.constraints {
        if !constraint.check_quote(amount_to_dex, quote) {
            info!(
                "Swap rejected by constraint {:?}: sell={}, buy={}",
                constraint, amount_to_dex, quote
            );
            return None;
        }
    }

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

    let (successful_swap, _amount_out) = if let Ok(amount_swapped) = swap_result {
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
        if let Err(error) = swap_client.withdraw(successful_swap).await {
            let msg = format!("{error:?}");
            mutate_state(|state| {
                token_swap.withdrawn_from_dex_at = Some(Err(msg.clone()));
                state.data.token_swaps.upsert(token_swap.clone());
            });
            error!("Failed to withdraw tokens: {}", msg.as_str());
            return Err(msg);
        } else {
            mutate_state(|state| {
                token_swap.withdrawn_from_dex_at = Some(Ok(ic_cdk::api::time().into()));
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
