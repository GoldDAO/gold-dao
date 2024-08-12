use crate::jobs::swap_tokens::Response::InternalError;
use crate::types::token_swaps::TokenSwap;
use crate::state::SwapConfig;
use crate::state::{ mutate_state, read_state, Data, RuntimeState };
use candid::CandidType;
use canister_time::run_now_then_interval;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use serde::{ Deserialize, Serialize };
use std::time::Duration;
use tracing::error;
use types::Milliseconds;
use types::TimestampMillis;

use utils::env::Environment;

pub const NANOS_PER_MILLISECOND: u64 = 1_000_000;
pub const SECOND_IN_MS: Milliseconds = 1000;

const INTERVAL: Duration = Duration::from_secs(7 * 24 * 60 * 60); // 1 week

pub const MEMO_SWAP: [u8; 7] = [0x4f, 0x43, 0x5f, 0x53, 0x57, 0x41, 0x50]; // OC_SWAP

pub fn start_job() {
    run_now_then_interval(INTERVAL, run);
}

fn prepare(args: SwapConfig, state: &mut RuntimeState) -> Result<TokenSwap, Response> {
    let now = state.env.now();
    Ok(state.data.token_swaps.push_new(args, now))
}

pub fn run() {
    // let token_swap = match mutate_state(|state| prepare(state.data., state)) {
    //     Ok(ts) => ts,
    //     Err(response) => return response,
    // };

    // ic_cdk::spawn(create_proposal(canister_id, args));
}

pub(crate) async fn process_token_swap(mut token_swap: TokenSwap, attempt: u32) -> Response {
    let swap_client = read_state(|state| state.data.icpswap_client);
    let args = swap_client.get_config();

    let account = if let Some(a) = extract_result(&token_swap.deposit_account) {
        *a
    } else {
        match swap_client.deposit_account().await {
            Ok(a) => {
                mutate_state(|state| {
                    let now = state.env.now();
                    token_swap.deposit_account = Some(Ok(a));
                    // FIXME: fix here an id
                    state.data.token_swaps.upsert(token_swap.clone());
                });
                a
            }
            Err(error) => {
                let msg = format!("{error:?}");
                mutate_state(|state| {
                    let now = state.env.now();
                    token_swap.deposit_account = Some(Err(msg.clone()));
                    token_swap.success = Some(false);
                    state.data.token_swaps.upsert(token_swap);
                });
                // log_error("Failed to get deposit account", msg.as_str(), attempt);
                return InternalError(msg);
            }
        }
    };

    // Get the ICP balance

    // FIXME: make this dynamic
    let amount_to_dex = 0;
    // let amount_to_dex = args.input_amount.saturating_sub(args.input_token.fee.into());

    if extract_result(&token_swap.transfer).is_none() {
        let now = read_state(|state| state.env.now());
        let transfer_result = match
            icrc_ledger_canister_c2c_client::icrc1_transfer(
                args.input_token.ledger_id,
                &(TransferArg {
                    from_subaccount: None,
                    to: account,
                    fee: Some(args.input_token.fee.into()),
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
                    let now = state.env.now();
                    token_swap.transfer = Some(Ok(index.0.try_into().unwrap()));
                    state.data.token_swaps.upsert(token_swap.clone());
                });
            }
            Err(msg) => {
                mutate_state(|state| {
                    let now = state.env.now();
                    token_swap.transfer = Some(Err(msg.clone()));
                    token_swap.success = Some(false);
                    state.data.token_swaps.upsert(token_swap);
                });
                log_error("Failed to transfer tokens", msg.as_str(), &args, attempt);
                return InternalError(msg);
            }
        }
    }

    if extract_result(&token_swap.notified_dex_at).is_none() {
        if let Err(error) = swap_client.deposit(amount_to_dex).await {
            let msg = format!("{error:?}");
            mutate_state(|state| {
                let now = state.env.now();
                token_swap.notified_dex_at = Some(Err(msg.clone()));
                state.data.token_swaps.upsert(token_swap.clone());
                enqueue_token_swap(token_swap, attempt, now, &mut state.data);
            });
            log_error("Failed to deposit tokens", msg.as_str(), &args, attempt);
            return InternalError(msg);
        } else {
            mutate_state(|state| {
                let now = state.env.now();
                token_swap.notified_dex_at = Some(Ok(()));
                state.data.token_swaps.upsert(token_swap.clone());
            });
        }
    }

    let swap_result = if let Some(a) = extract_result(&token_swap.amount_swapped).cloned() {
        a
    } else {
        match
            swap_client.swap(
                amount_to_dex.saturating_sub(args.input_token.fee.into()),
                // FIXME: make this dynamic
                0
            ).await
        {
            Ok(a) => {
                mutate_state(|state| {
                    let now = state.env.now();
                    token_swap.amount_swapped = Some(Ok(a.clone()));
                    state.data.token_swaps.upsert(token_swap.clone());
                });
                a
            }
            Err(error) => {
                let msg = format!("{error:?}");
                mutate_state(|state| {
                    let now = state.env.now();
                    token_swap.amount_swapped = Some(Err(msg.clone()));
                    state.data.token_swaps.upsert(token_swap.clone());
                    enqueue_token_swap(token_swap, attempt, now, &mut state.data);
                });
                log_error("Failed to swap tokens", msg.as_str(), &args, attempt);
                return InternalError(msg);
            }
        }
    };

    let (successful_swap, amount_out) = if let Ok(amount_swapped) = swap_result {
        (true, amount_swapped.saturating_sub(args.output_token.fee.into()))
    } else {
        (false, amount_to_dex.saturating_sub(args.input_token.fee.into()))
    };

    if extract_result(&token_swap.withdrawn_from_dex_at).is_none() {
        if let Err(error) = swap_client.withdraw(successful_swap, amount_out).await {
            let msg = format!("{error:?}");
            mutate_state(|state| {
                let now = state.env.now();
                token_swap.withdrawn_from_dex_at = Some(Err(msg.clone()));
                state.data.token_swaps.upsert(token_swap.clone());
                enqueue_token_swap(token_swap, attempt, now, &mut state.data);
            });
            log_error("Failed to withdraw tokens", msg.as_str(), &args, attempt);
            return InternalError(msg);
        } else {
            mutate_state(|state| {
                let now = state.env.now();
                token_swap.withdrawn_from_dex_at = Some(Ok(amount_out));
                token_swap.success = Some(successful_swap);
                state.data.token_swaps.upsert(token_swap);
            });
        }
    }

    if successful_swap {
        Response::Success(SuccessResult { amount_out })
    } else {
        Response::SwapFailed
    }
}

fn enqueue_token_swap(token_swap: TokenSwap, attempt: u32, now: TimestampMillis, data: &mut Data) {
    if attempt < 20 {
        data.timer_jobs.enqueue_job(
            TimerJob::ProcessTokenSwap(
                Box::new(ProcessTokenSwapJob {
                    token_swap,
                    attempt: attempt + 1,
                })
            ),
            now + 5 * SECOND_IN_MS,
            now
        );
    }
}

// TODO: think on how to add delay here
async fn retry_with_attempts<F, Fut>(
    max_attempts: u8,
    _delay_duration: Duration,
    mut f: F
)
    -> Result<(), String>
    where F: FnMut() -> Fut, Fut: std::future::Future<Output = Result<(), String>>
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

fn extract_result<T>(subtask: &Option<Result<T, String>>) -> Option<&T> {
    subtask.as_ref().and_then(|t| t.as_ref().ok())
}

fn log_error(message: &str, error: &str, args: &SwapConfig, attempt: u32) {
    // TODO: fix here
    error!(
        input_token = args.input_token.ledger_id.to_string(),
        output_token = args.output_token.ledger_id.to_string(),
        error,
        attempt,
        message
    );
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    SwapFailed,
    PinRequired,
    PinIncorrect(Milliseconds),
    TooManyFailedPinAttempts(Milliseconds),
    InternalError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub amount_out: u128,
}
