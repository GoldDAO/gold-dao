use crate::state::read_state;
use bity_ic_canister_time::{run_now_then_interval, timestamp_millis, HOUR_IN_MS};
use candid::{Nat, Principal};
use futures::future::join_all;
use icrc_ledger_canister_c2c_client::icrc1_balance_of;
use icrc_ledger_types::icrc1::account::Account;
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::claim_rewards_batch::{
    Args as BatchClaimArgs, Response as ClaimResponse,
};
use std::time::Duration;
use tracing::{error, info};
use types::TimestampMillis;

const CLAIM_REWARDS_THRESHOLD: u64 = 10_000_000_u64;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(HOUR_IN_MS), spawn_claim_rewards_job);
}

fn spawn_claim_rewards_job() {
    ic_cdk::futures::spawn(claim_rewards_impl());
}

async fn claim_rewards_impl() {
    let now = timestamp_millis();

    if !is_allowed_to_run(now) {
        return;
    }

    claim_rewards().await
}

pub async fn claim_rewards() {
    let _span = tracing::info_span!("CLAIM_NEURON_REWARDS").entered();

    info!("start");

    let neurons = read_state(|s| s.data.neuron_system.get_neurons());
    let neuron_ids: Vec<NeuronId> = neurons.iter().filter_map(|n| n.id.clone()).collect();
    let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());

    let mut futures = Vec::new();
    let mut neuron_token_pairs = Vec::new();
    for neuron_id in &neuron_ids {
        for token_symbol in &reward_types {
            let token_symbol = *token_symbol;
            let token_ledger = token_symbol.get_prod_token_info().ledger_id;
            futures.push(fetch_neuron_reward_balance(neuron_id, token_ledger));
            neuron_token_pairs.push((neuron_id.clone(), token_symbol));
        }
    }

    let results = join_all(futures).await;

    let mut claim_reward_args = Vec::new();

    for (idx, result) in results.into_iter().enumerate() {
        let (neuron_id, token_symbol) = &neuron_token_pairs[idx];
        match result {
            Ok(balance) if balance >= CLAIM_REWARDS_THRESHOLD => {
                claim_reward_args.push(
                    sns_rewards_api_canister::claim_rewards_batch::ClaimRewardArgs {
                        neuron_id: neuron_id.clone(),
                        token: *token_symbol,
                    },
                );
            }
            Ok(balance) => {
                info!(
                    "neuron id - {} :: token - {:?} :: balance below threshold: {}",
                    neuron_id, token_symbol, balance
                );
            }
            Err(e) => {
                error!(
                    "neuron id - {} :: token - {:?} :: error: {}",
                    neuron_id, token_symbol, e
                );
            }
        }
    }

    if claim_reward_args.is_empty() {
        info!("no eligible neurons found for claiming rewards.");
        return;
    }

    let claim_reward_args_len = claim_reward_args.len();
    let sns_rewards_canister_id = read_state(|s| s.data.goldao_sns_rewards_canister_id);
    let args = BatchClaimArgs { claim_reward_args };

    match sns_rewards_c2c_client::claim_rewards_batch(sns_rewards_canister_id, args).await {
        Ok(ClaimResponse::Ok(())) => {
            info!(
                "successfully claimed rewards for {} entries",
                claim_reward_args_len
            );
        }
        Ok(ClaimResponse::Err(errors)) => {
            for error in errors {
                error!(
                    "neuron id - {} :: token - {:?} :: error - {:?}",
                    error.neuron_id, error.token, error.error
                );
            }
        }
        Err(e) => error!("batch claim failed: {:?}", e),
    }

    info!("finished");
}

fn is_allowed_to_run(initial_run_time: TimestampMillis) -> bool {
    let _span = tracing::info_span!("IS_ALLOWED_TO_RUN").entered();

    let is_awaiting = read_state(|s| s.data.unallocated_rewards_pool.is_awaiting());
    let reward_claim_interval = match read_state(|s| s.data.reward_claim_interval.clone()) {
        Some(interval) => interval,
        None => {
            info!("no claim interval set, aborting");
            return false;
        }
    };

    let is_time_valid = reward_claim_interval.is_within_daily_interval(initial_run_time);

    if !is_awaiting {
        info!("claim already in progress");
        return false;
    }

    is_time_valid
}

async fn fetch_neuron_reward_balance(
    neuron_id: &NeuronId,
    token_ledger: Principal,
) -> Result<Nat, String> {
    let sns_rewards_canister_id = read_state(|s| s.data.goldao_sns_rewards_canister_id);

    icrc1_balance_of(
        token_ledger,
        Account {
            owner: sns_rewards_canister_id,
            subaccount: Some(neuron_id.clone().into()),
        },
    )
    .await
    .map_err(|e| format!("fetch balance error: {:?}", e))
}
