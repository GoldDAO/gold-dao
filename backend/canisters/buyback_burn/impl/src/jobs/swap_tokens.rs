use crate::jobs::swap_tokens::Response::InternalError;
use crate::model::token_swaps::TokenSwap;
use crate::state::{mutate_state, read_state, Data, State};
use crate::timer_job_types::ProcessTokenSwapJob;
use crate::timer_job_types::TimerJob;
use crate::token_swap::swap_client::SwapClient;
use icpswap_client::ICPSwapClient;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::error;
use types::TimestampMillis;
use utils::env::Environment;

use types::Milliseconds;
pub const NANOS_PER_MILLISECOND: u64 = 1_000_000;
pub const SECOND_IN_MS: Milliseconds = 1000;

pub const MEMO_SWAP: [u8; 7] = [0x4F, 0x43, 0x5F, 0x53, 0x57, 0x41, 0x50]; // OC_SWAP

fn prepare(args: SwapConfig, state: &mut State) -> Result<TokenSwap, Response> {
    let now = state.env.now();

    // if let Err(error) = state.data.pin_number.verify(args.pin.as_deref(), now) {
    //     return Err(match error {
    //         VerifyPinError::PinRequired => PinRequired,
    //         VerifyPinError::PinIncorrect(delay) => PinIncorrect(delay),
    //         VerifyPinError::TooManyFailedAttempted(delay) => TooManyFailedPinAttempts(delay),
    //     });
    // }

    Ok(state.data.token_swaps.push_new(args, now))
}

pub(crate) async fn process_token_swap(mut token_swap: TokenSwap, attempt: u32) -> Response {
    let args = token_swap.args.clone();
    let swap_client = read_state(|state| build_swap_client(&args, state));

    let account = if let Some(a) = extract_result(&token_swap.deposit_account) {
        *a
    } else {
        match swap_client.deposit_account().await {
            Ok(a) => {
                mutate_state(|state| {
                    let now = state.env.now();
                    token_swap.deposit_account = Some(Ok(a));
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
                log_error(
                    "Failed to get deposit account",
                    msg.as_str(),
                    &args,
                    attempt,
                );
                return InternalError(msg);
            }
        }
    };

    let amount_to_dex = args.input_amount.saturating_sub(args.input_token.fee);

    if extract_result(&token_swap.transfer).is_none() {
        let now = read_state(|state| state.env.now());
        let transfer_result = match icrc_ledger_canister_c2c_client::icrc1_transfer(
            args.input_token.ledger,
            &TransferArg {
                from_subaccount: None,
                to: account,
                fee: Some(args.input_token.fee.into()),
                created_at_time: Some(now * NANOS_PER_MILLISECOND),
                memo: Some(MEMO_SWAP.to_vec().into()),
                amount: amount_to_dex.into(),
            },
        )
        .await
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
        match swap_client
            .swap(
                amount_to_dex.saturating_sub(args.input_token.fee),
                args.min_output_amount,
            )
            .await
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
        (true, amount_swapped.saturating_sub(args.output_token.fee))
    } else {
        (false, amount_to_dex.saturating_sub(args.input_token.fee))
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

use crate::state::SwapConfig;
fn build_swap_client(args: &SwapConfig, state: &State) -> Box<dyn SwapClient> {
    let this_canister_id = state.env.canister_id();
    let input_token = args.input_token.clone();
    let output_token = args.output_token.clone();

    let (token0, token1) = if args.exchange_args.zero_for_one {
        (input_token, output_token)
    } else {
        (output_token, input_token)
    };

    Box::new(ICPSwapClient::new(
        this_canister_id,
        args.exchange_args.swap_canister_id,
        token0,
        token1,
        args.exchange_args.zero_for_one,
    ))
}

fn enqueue_token_swap(token_swap: TokenSwap, attempt: u32, now: TimestampMillis, data: &mut Data) {
    if attempt < 20 {
        data.timer_jobs.enqueue_job(
            TimerJob::ProcessTokenSwap(Box::new(ProcessTokenSwapJob {
                token_swap,
                attempt: attempt + 1,
            })),
            now + 5 * SECOND_IN_MS,
            now,
        );
    }
}

fn extract_result<T>(subtask: &Option<Result<T, String>>) -> Option<&T> {
    subtask.as_ref().and_then(|t| t.as_ref().ok())
}

fn log_error(message: &str, error: &str, args: &SwapConfig, attempt: u32) {
    error!(
        input_token = args.input_token.token.token_symbol(),
        output_token = args.output_token.token.token_symbol(),
        error,
        attempt,
        message
    );
}

use candid::CandidType;
use serde::{Deserialize, Serialize};

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
