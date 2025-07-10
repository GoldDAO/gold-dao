use crate::transfer_rewards;
use ic_cdk::update;
pub use sns_rewards_api_canister::claim_rewards_batch::{
    Args as ClaimRewardsBatchArgs, Response as ClaimRewardsBatchResponse,
};
use sns_rewards_api_canister::claim_rewards_batch::{ClaimRewardError, ClaimRewardErrorType};
use tracing::error;

use crate::{
    state::{read_state, RuntimeState},
    utils::{
        authenticate_by_hotkey, fetch_neuron_data_by_id, AuthenticateByHotkeyResponse,
        FetchNeuronDataByIdResponse,
    },
};
use utils::env::Environment;

#[update]
async fn claim_rewards_batch(args: ClaimRewardsBatchArgs) -> ClaimRewardsBatchResponse {
    let caller = read_state(|s| s.env.caller());

    // 0. fetch all neurons in parallel
    let fetch_futures = args
        .claim_reward_args
        .iter()
        .map(|arg| fetch_neuron_data_by_id(&arg.neuron_id));
    let fetch_results = futures::future::join_all(fetch_futures).await;

    let mut authed_requests = vec![];
    let mut errors = vec![];

    for (i, result) in fetch_results.into_iter().enumerate() {
        let arg = &args.claim_reward_args[i];
        let neuron_id = arg.neuron_id.clone();
        let token = arg.token.clone();

        match result {
            FetchNeuronDataByIdResponse::Ok(neuron) => {
                match authenticate_by_hotkey(&neuron, &caller) {
                    AuthenticateByHotkeyResponse::Ok(_) => {
                        authed_requests.push((neuron_id, token));
                    }
                    AuthenticateByHotkeyResponse::NeuronHotKeyInvalid => {
                        errors.push(ClaimRewardError {
                            neuron_id,
                            token: Some(token),
                            error: ClaimRewardErrorType::NeuronHotKeyInvalid,
                        });
                    }
                }
            }
            FetchNeuronDataByIdResponse::NeuronDoesNotExist => {
                errors.push(ClaimRewardError {
                    neuron_id,
                    token: Some(token),
                    error: ClaimRewardErrorType::NeuronDoesNotExist,
                });
            }
            FetchNeuronDataByIdResponse::InternalError(e) => {
                errors.push(ClaimRewardError {
                    neuron_id,
                    token: Some(token),
                    error: ClaimRewardErrorType::InternalError(e),
                });
            }
        }
    }

    // 3. transfer rewards for each (authed_neuron, token) pair
    let mut transfer_futures = vec![];
    let mut transfer_meta = vec![]; // (neuron_id, token) for each future

    for (neuron_id, token) in &authed_requests {
        let token_info_opt = read_state(|s: &RuntimeState| s.data.tokens.get(token).cloned());
        let neuron_id_cloned = neuron_id.clone();
        match token_info_opt {
            Some(token_info) => {
                let token_cloned = token.clone();
                let token_info = token_info.clone();
                let caller_cloned = caller.clone();

                // collect the futures
                transfer_futures.push(transfer_rewards(
                    neuron_id_cloned.clone(),
                    caller_cloned,
                    token_info,
                ));
                transfer_meta.push((neuron_id_cloned, token_cloned));
            }
            None => {
                error!("Token info for type {token:?} not found in state");
                errors.push(ClaimRewardError {
                    neuron_id: neuron_id.clone(),
                    token: Some(token.clone()),
                    error: ClaimRewardErrorType::TokenSymbolInvalid(token.clone()),
                });
            }
        }
    }

    // 4. await all transfers and collect any transfer errors
    let transfer_results = futures::future::join_all(transfer_futures).await;

    for (result, (neuron_id, token)) in transfer_results.into_iter().zip(transfer_meta) {
        if let Err(e) = result {
            errors.push(ClaimRewardError {
                neuron_id,
                token: Some(token),
                error: ClaimRewardErrorType::TransferFailed(e),
            });
        }
    }

    // 5. return response
    if errors.is_empty() {
        ClaimRewardsBatchResponse::Ok(())
    } else {
        ClaimRewardsBatchResponse::Err(errors)
    }
}
