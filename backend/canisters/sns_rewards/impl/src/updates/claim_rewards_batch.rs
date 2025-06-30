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
    let fetch_futures = args.neuron_id.iter().map(fetch_neuron_data_by_id);
    let fetch_results = futures::future::join_all(fetch_futures).await;

    // 1. separate OK and error results
    let mut valid_neurons = vec![];
    let mut errors = vec![];

    for (i, res) in fetch_results.into_iter().enumerate() {
        let neuron_id = args.neuron_id[i].clone();
        match res {
            FetchNeuronDataByIdResponse::Ok(neuron) => {
                valid_neurons.push((neuron_id, neuron));
            }
            FetchNeuronDataByIdResponse::InternalError(e) => {
                errors.push(ClaimRewardError {
                    neuron_id,
                    token: None,
                    error: ClaimRewardErrorType::InternalError(e),
                });
            }
            FetchNeuronDataByIdResponse::NeuronDoesNotExist => {
                errors.push(ClaimRewardError {
                    neuron_id,
                    token: None,
                    error: ClaimRewardErrorType::NeuronDoesNotExist,
                });
            }
        }
    }

    // 2. authenticate all valid neurons
    let mut authed_neurons = vec![];
    for (neuron_id, neuron) in valid_neurons {
        match authenticate_by_hotkey(&neuron, &caller) {
            AuthenticateByHotkeyResponse::Ok(_) => authed_neurons.push(neuron_id),
            AuthenticateByHotkeyResponse::NeuronHotKeyInvalid => {
                errors.push(ClaimRewardError {
                    neuron_id,
                    token: None,
                    error: ClaimRewardErrorType::NeuronHotKeyInvalid,
                });
            }
        }
    }

    // 3. transfer rewards for each (token, authed_neuron) pair
    let mut transfer_futures = vec![];

    for neuron_id in &authed_neurons {
        for token in &args.tokens {
            let token_info_opt = read_state(|s: &RuntimeState| s.data.tokens.get(token).cloned());

            match token_info_opt {
                Some(token_info) => {
                    let neuron_id = neuron_id.clone();
                    let token_info = token_info.clone();
                    let token = token.clone();
                    let caller = caller.clone();

                    transfer_futures.push(async move {
                        transfer_rewards(&neuron_id, caller, &token_info)
                            .await
                            .map_err(|e| ClaimRewardError {
                                neuron_id,
                                token: Some(token),
                                error: ClaimRewardErrorType::TransferFailed(e),
                            })
                    });
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
    }

    // 4. await all transfers and collect any transfer errors
    let transfer_results = futures::future::join_all(transfer_futures).await;

    for result in transfer_results {
        if let Err(e) = result {
            errors.push(e);
        }
    }

    // 5. return response
    if errors.is_empty() {
        ClaimRewardsBatchResponse::Ok(())
    } else {
        ClaimRewardsBatchResponse::Err(errors)
    }
}
