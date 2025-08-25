use crate::model::unallocated_rewards_pool::UnallocatedRewards;
use crate::model::unallocated_rewards_pool::UnallocatedRewardsPool;
use crate::state::{mutate_state, read_state};
use candid::Nat;
use futures::future::join_all;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use std::collections::BTreeSet;
use tracing::debug;
use tracing::error;
use tracing::info;
use types::TokenSymbol;

pub async fn process_rewards_impl() -> Result<(), BTreeSet<TokenSymbol>> {
    let _span = tracing::info_span!("PROCESS_REWARDS").entered();

    info!("start");

    let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());
    let unallocated_rewards_pool = read_state(|s| s.data.unallocated_rewards_pool.clone());

    let results = batch_rewards_transfer(reward_types, unallocated_rewards_pool).await;

    let mut errors = BTreeSet::new();

    for (token_symbol, transfer_result) in results.into_iter() {
        match transfer_result {
            Ok(rewards_to_allocate) => {
                info!(
                    "transfer successful for token: {}, rewards to allocate: {}",
                    token_symbol, rewards_to_allocate
                );
            }
            Err(err) => match &err {
                GeneralError::CallError(err) | GeneralError::TransferError(err) => {
                    error!(
                        "transfer failed for token: {}, error: {}",
                        token_symbol, err
                    );
                    errors.insert(token_symbol);
                }
                _ => {
                    debug!(
                        "ignoring non-transfer error for token: {}, error: {:?}",
                        token_symbol, err
                    );
                }
            },
        }
    }

    info!("finished");

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub async fn batch_rewards_transfer(
    reward_types: BTreeSet<TokenSymbol>,
    unallocated_rewards_pool: UnallocatedRewardsPool,
) -> Vec<(TokenSymbol, Result<Nat, GeneralError>)> {
    mutate_state(|s| {
        s.data.unallocated_rewards_pool.transition_to_transferring();
    });
    let mut transfer_futures = Vec::new();
    let mut token_symbols = Vec::new();

    for reward_type in reward_types {
        let token_ledger = reward_type.get_token_info().ledger_id;
        let ledger_fee = reward_type.get_token_info().fee;

        let future =
            unallocated_rewards_pool.transfer_part_of_rewards(token_ledger, Nat::from(ledger_fee));
        transfer_futures.push(future);
        token_symbols.push(reward_type);
    }

    let results = join_all(transfer_futures).await;

    mutate_state(|s| {
        s.data.unallocated_rewards_pool.transition_to_awaiting();
    });

    token_symbols.into_iter().zip(results.into_iter()).collect()
}
