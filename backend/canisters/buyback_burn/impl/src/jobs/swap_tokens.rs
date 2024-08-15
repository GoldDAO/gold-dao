use canister_time::run_now_then_interval;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::error;
use canister_tracing_macros::trace;
use utils::env::Environment;
use crate::types::token_swaps::TokenSwap;
use crate::state::{ mutate_state, read_state, RuntimeState };
use crate::swap_clients::{ SwapClient, SwapConfig };
use crate::utils::{
    calculate_percentage_of_amount,
    get_token_balance,
    retry_with_attempts,
    RETRY_DELAY,
};

pub const NANOS_PER_MILLISECOND: u64 = 1_000_000;
const MAX_ATTEMPTS: u8 = 3;

pub const MEMO_SWAP: [u8; 7] = [0x4f, 0x43, 0x5f, 0x53, 0x57, 0x41, 0x50]; // OC_SWAP

pub fn start_job() {
    let swap_interval = read_state(|s| s.data.swap_interval);
    run_now_then_interval(swap_interval, run);
}

pub fn prepare_swap(args: SwapConfig, state: &mut RuntimeState) -> TokenSwap {
    let now = state.env.now();
    // TODO: implement that swap_client_id is stored in TokenSwap. All the clients should be queried in loop and for each of them the token swap should be created
    state.data.token_swaps.push_new(args, now)
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

#[trace]
async fn run_async() {
    let swap_clients = read_state(|state| { state.data.swap_clients.clone() });

    for swap_client in swap_clients.iter() {
        let args = swap_client.get_config();
        let token_swap = mutate_state(|state| prepare_swap(args, state));

        if
            let Err(err) = retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, || async {
                process_token_swap(swap_client, token_swap.clone()).await
            }).await
        {
            error!("Failed to swap tokens after {} attempts: {:?}", MAX_ATTEMPTS, err);
        }
    }
}

pub(crate) async fn process_token_swap(
    swap_client: &Box<dyn SwapClient>,
    mut token_swap: TokenSwap
) -> Result<(), String> {
    let swap_config = swap_client.get_config();

    let burn_rate = read_state(|s| s.data.burn_config.burn_rate);

    let available_amount = get_token_balance(swap_config.input_token.ledger_id).await?;
    let input_amount = calculate_percentage_of_amount(available_amount, burn_rate);
    let amount_to_dex = input_amount.saturating_sub(swap_config.input_token.fee.into());

    // TODO: Should we use this parameter? We can try to also store the minimum ICP/GLDGov
    // price and then calculate the min_output_amount
    let min_output_amount = 0;

    // Get the deposit account
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

    // Deposit tokens to the deposit account
    if extract_result(&token_swap.transfer).is_none() {
        let now = read_state(|state| state.env.now());
        // FIXME: check that here it would work with ICP
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
                // let now = state.env.now();
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
