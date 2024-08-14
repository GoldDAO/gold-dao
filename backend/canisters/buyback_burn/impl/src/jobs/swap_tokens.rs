use crate::types::token_swaps::TokenSwap;
use crate::state::{ mutate_state, read_state, RuntimeState };
use candid::CandidType;
use canister_time::run_now_then_interval;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use serde::{ Deserialize, Serialize };
use std::time::Duration;
use tracing::error;
use types::Milliseconds;
use crate::token_swap::SwapConfig;

use canister_tracing_macros::trace;

use utils::env::Environment;

pub const NANOS_PER_MILLISECOND: u64 = 1_000_000;
const MAX_ATTEMPTS: u8 = 3;
const RETRY_DELAY: Duration = Duration::from_secs(5 * 60); // each 5 minutes

pub const MEMO_SWAP: [u8; 7] = [0x4f, 0x43, 0x5f, 0x53, 0x57, 0x41, 0x50]; // OC_SWAP

pub fn start_job() {
    let swap_interval = read_state(|s| s.data.swap_interval);
    run_now_then_interval(swap_interval, run);
}

fn prepare(args: SwapConfig, state: &mut RuntimeState) -> TokenSwap {
    let now = state.env.now();
    // TODO: implement that swap_client_id is stored in TokenSwap. All the clients should be queried in loop and for each of them the token swap should be created
    state.data.token_swaps.push_new(args, now)
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

// FIXME
async fn run_async() {
    let swap_clients = read_state(|state| { state.data.swap_clients.clone() });

    for swap_client in swap_clients.iter() {
        let args = swap_client.get_config();
        let token_swap = mutate_state(|state| prepare(args, state));

        if
            let Err(err) = retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, || async {
                process_token_swap(token_swap.clone()).await
            }).await
        {
            error!("Failed to swap tokens after {} attempts: {:?}", MAX_ATTEMPTS, err);
        }
    }
}

#[trace]
pub(crate) async fn process_token_swap(mut token_swap: TokenSwap) -> Result<(), String> {
    let swap_client = read_state(|state| {
        state.data.swap_clients.swap_clients.get(0).unwrap().clone()
    });

    let args = swap_client.get_config();

    let account = if let Some(a) = extract_result(&token_swap.deposit_account) {
        *a
    } else {
        match swap_client.deposit_account().await {
            Ok(a) => {
                mutate_state(|state| {
                    // let now = state.env.now();
                    token_swap.deposit_account = Some(Ok(a));
                    // FIXME: fix here an id
                    state.data.token_swaps.upsert(token_swap.clone());
                });
                a
            }
            Err(error) => {
                let msg = format!("{error:?}");
                mutate_state(|state| {
                    // let now = state.env.now();
                    token_swap.deposit_account = Some(Err(msg.clone()));
                    token_swap.success = Some(false);
                    state.data.token_swaps.upsert(token_swap);
                });
                error!("Failed to deposit tokens while swap: {}", msg.as_str());
                return Err(msg);
            }
        }
    };

    // Get the ICP balance

    // FIXME: make this dynamic
    // let amount_to_dex = 0;
    let input_amount: u128 = 1;
    let amount_to_dex = input_amount.saturating_sub(args.input_token.fee.into());

    if extract_result(&token_swap.transfer).is_none() {
        let now = read_state(|state| state.env.now());
        // FIXME: check that here it would work with ICP
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
                // let now = state.env.now();
                token_swap.notified_dex_at = Some(Ok(()));
                state.data.token_swaps.upsert(token_swap.clone());
            });
        }
    }

    // TODO: Should we use this parameter? We can try to also store the minimum ICP/GLDGov
    // price and then calculate the min_output_amount
    let min_output_amount = 0;

    let swap_result = if let Some(a) = extract_result(&token_swap.amount_swapped).cloned() {
        a
    } else {
        match
            swap_client.swap(
                amount_to_dex.saturating_sub(args.input_token.fee.into()),
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
        (true, amount_swapped.saturating_sub(args.output_token.fee.into()))
    } else {
        (false, amount_to_dex.saturating_sub(args.input_token.fee.into()))
    };

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

// fn enqueue_token_swap(token_swap: TokenSwap, attempt: u32, now: TimestampMillis, data: &mut Data) {
//     if attempt < 20 {
//         data.timer_jobs.enqueue_job(
//             TimerJob::ProcessTokenSwap(
//                 Box::new(ProcessTokenSwapJob {
//                     token_swap,
//                     attempt: attempt + 1,
//                 })
//             ),
//             now + 5 * SECOND_IN_MS,
//             now
//         );
//     }
// }

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

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    SwapFailed,
    TooManyFailedPinAttempts(Milliseconds),
    InternalError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub amount_out: u128,
}
