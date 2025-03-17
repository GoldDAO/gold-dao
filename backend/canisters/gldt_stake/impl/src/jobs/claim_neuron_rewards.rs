use candid::Nat;
use canister_time::{run_interval, timestamp_millis, HOUR_IN_MS};
use gldt_stake_common::reward_tokens::{LedgerFee, LedgerId, TokenSymbol};
use ic_cdk::spawn;
use icrc_ledger_canister_c2c_client::icrc1_balance_of;
use icrc_ledger_types::icrc1::account::Account;
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::claim_reward::Response as ClaimResponse;
use std::time::Duration;
use tracing::{error, info};
use types::TimestampMillis;

use crate::state::{mutate_state, read_state};

pub fn start_job() {
    run_interval(Duration::from_millis(HOUR_IN_MS), spawn_rewards_job);
}

fn spawn_rewards_job() {
    ic_cdk::spawn(process_rewards_impl())
}

async fn process_rewards_impl() {
    let now = timestamp_millis();
    info!("CLAIM_NEURON_REWARDS :: start");

    if !is_allowed_to_run(now) {
        return;
    }
    mutate_state(|s| {
        s.data.is_reward_claim_in_progress = true;
    });

    let neurons = read_state(|s| s.data.neuron_system.get_neurons());
    let neuron_ids: Vec<NeuronId> = neurons.iter().filter_map(|n| n.id.clone()).collect();
    let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());
    neuron_ids.into_iter().for_each(|neuron_id| {
        reward_types
            .clone()
            .into_iter()
            .for_each(|(token_symbol, (token_ledger, ledger_fee))| {
                let neuron_id = neuron_id.clone();
                spawn(async move {
                    let _ =
                        spawn_claim_procedure(neuron_id, token_symbol, token_ledger, ledger_fee)
                            .await;
                });
            })
    });
    mutate_state(|s| {
        s.data.is_reward_claim_in_progress = false;
    });
    info!("CLAIM_NEURON_REWARDS :: finished");
}

fn is_allowed_to_run(initial_run_time: TimestampMillis) -> bool {
    let distribution_in_progress = read_state(|s| s.data.is_reward_claim_in_progress);
    let distribution_interval = match read_state(|s| s.data.reward_claim_interval.clone()) {
        Some(interval) => interval,
        None => {
            info!("CLAIM_NEURON_REWARDS :: not correct time to run, finishing early");
            return false;
        }
    };
    let is_distribution_time_valid =
        distribution_interval.is_within_weekly_interval(initial_run_time.clone());

    // in_progress
    if distribution_in_progress {
        info!("CLAIM_NEURON_REWARDS :: reward claim alread in progress");
        return false;
    }
    if is_distribution_time_valid {
        return true;
    }

    false
}

async fn spawn_claim_procedure(
    neuron_id: NeuronId,
    token_symbol: TokenSymbol,
    token_ledger: LedgerId,
    ledger_fee: LedgerFee,
) -> Result<(), String> {
    info!(
        "CLAIM_NEURON_REWARDS :: neuron id - {} :: claim procedure started",
        neuron_id
    );
    let reward_balance = fetch_neuron_reward_balance(&neuron_id, &token_ledger).await?;
    // if there are more than 10 of any token type then claim the rewards
    info!(
        "CLAIM_NEURON_REWARDS :: neuron id - {} :: found rewards of {} {}",
        neuron_id, reward_balance, token_symbol
    );
    if reward_balance < Nat::from(1_000_000_000u64) {
        info!(
            "CLAIM_NEURON_REWARDS :: neuron id - {} :: reward of {} {} is less than the threshold of 1_000_000_000",
            neuron_id, reward_balance, token_symbol
        );
        return Err(format!("Not enough rewards to process this neuron"));
    };
    claim_reward(neuron_id.clone(), &token_symbol).await?;

    let reward = reward_balance - ledger_fee;
    mutate_state(|s| {
        s.data.reward_system.add_reward_round(
            reward.clone(),
            token_symbol.clone(),
            timestamp_millis(),
        )
    });
    info!(
        "CLAIM_NEURON_REWARDS :: neuron id - {} :: {} {} claim procedure finished successfully",
        neuron_id, reward, token_symbol
    );
    Ok(())
}

async fn fetch_neuron_reward_balance(
    neuron_id: &NeuronId,
    token_ledger: &LedgerId,
) -> Result<Nat, String> {
    let sns_rewards_canister_id = read_state(|s| s.data.goldao_sns_rewards_canister_id);
    match icrc1_balance_of(
        token_ledger.clone(),
        Account {
            owner: sns_rewards_canister_id.clone(),
            subaccount: Some(neuron_id.clone().into()),
        },
    )
    .await
    {
        Ok(balance) => Ok(balance),
        Err(e) => Err(format!("fetch_neuron_reward_balance error: {e:?}")),
    }
}

async fn claim_reward(neuron_id: NeuronId, token_symbol: &TokenSymbol) -> Result<(), String> {
    info!(
        "CLAIM_NEURON_REWARDS :: neuron id - {} :: attempting to claim {} reward",
        neuron_id, token_symbol
    );
    let sns_rewards_canister_id = read_state(|s| s.data.goldao_sns_rewards_canister_id);

    let mut args = sns_rewards_api_canister::claim_reward::Args {
        neuron_id: neuron_id.clone(),
        token: token_symbol.clone(),
    };

    if token_symbol == "GOLDAO" {
        args.token = "GLDGov".to_string()
    }

    match sns_rewards_c2c_client::claim_reward(sns_rewards_canister_id, args).await {
        Ok(response) => match response {
            ClaimResponse::Ok(_) => {
                info!(
                    "CLAIM_NEURON_REWARDS :: neuron id - {} :: {} rewards claimed successful",
                    neuron_id, token_symbol
                );
                Ok(())
            }
            other_response => {
                error!(
                    "CLAIM_NEURON_REWARDS :: neuron id - {} :: {} rewards claimed failed with error - {other_response:?}",
                    neuron_id, token_symbol
                );
                Err(format!("{other_response:?}"))
            }
        },
        Err(err) => {
            error!(
                "CLAIM_NEURON_REWARDS :: neuron id - {} :: {} rewards claimed failed with error - {err:?}",
                neuron_id, token_symbol
            );
            Err(format!("{err:?}"))
        }
    }
}
