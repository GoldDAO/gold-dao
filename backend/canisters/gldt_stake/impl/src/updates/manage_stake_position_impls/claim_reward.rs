use crate::model::event_transaction::EventTransaction;
use crate::model::event_transaction::StakePositionStateChange;
use crate::{
    model::allocated_rewards_pool::{AllocatedRewards, AllocatedRewardsPool},
    state::{mutate_state, read_state},
};
use candid::{Nat, Principal};
use futures::future::join_all;
use gldt_stake_common::manage_stake_position_interface::ClaimRewardErrors;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use gldt_stake_common::manage_stake_position_interface::RemoveRewardErrors;
use gldt_stake_common::{stake_position::StakePosition, stake_position_event::ClaimRewardStatus};
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use tracing::error;
use types::TokenSymbol;

pub async fn claim_rewards_batch_impl(
    caller: Principal,
    tokens: Vec<TokenSymbol>,
) -> Result<Vec<Result<(), ClaimRewardErrors>>, ManageStakePositionError> {
    // 0. load caller's stake position
    let mut position = read_state(|s| s.data.stake_system.get_stake_position(&caller)).unwrap();

    // 1. mark reward claim as in progress
    mutate_state(|s| {
        position.claim_reward_status = ClaimRewardStatus::InProgress;
        s.data
            .stake_system
            .upsert_stake_position(caller, position.clone());
    });

    // 2. prepare async reward transfer futures per token
    let mut futures = vec![];
    for token in &tokens {
        futures.push(checked_transfer_rewards(caller, position.clone(), *token));
    }

    // 3. execute all transfers in parallel
    let results = join_all(futures).await;

    let mut reward_updates: Vec<(TokenSymbol, Nat)> = vec![];
    let mut overall_status = ClaimRewardStatus::None;

    // 4. collect updates and determine final status
    for (token, res) in tokens.iter().zip(results.iter()) {
        if let Ok((_, (amt, status))) = res {
            reward_updates.push((*token, amt.clone()));
            if status != &ClaimRewardStatus::None {
                overall_status = status.clone();
            }
        }
    }

    // 5. snapshot claimable rewards before mutation
    let rewards_before_claim: Vec<(TokenSymbol, Nat)> = position
        .claimable_rewards
        .iter()
        .map(|(k, v)| (*k, v.clone()))
        .collect();

    // 6. record ICRC3 transaction if any rewards changed
    if !reward_updates.is_empty() {
        let transaction = EventTransaction::new(StakePositionStateChange::ClaimRewards {
            rewards_before_claim,
            reward_updates: reward_updates.clone(),
        });

        // 7. try to persist transaction
        if let Err(e) = crate::state::icrc3_add_transaction(transaction) {
            error!("icrc3_add_transaction failed: {:?}", e);
            mutate_state(|s| {
                position.claim_reward_status = ClaimRewardStatus::Failed(e.to_string());
                s.data
                    .stake_system
                    .upsert_stake_position(caller, position.clone());
            });
            return Err(ManageStakePositionError::GeneralError(
                GeneralError::TransactionAddError(e.to_string()),
            ));
        };

        // 8. apply reward updates and final status
        mutate_state(|s| {
            if let Some(mut position) = s.data.stake_system.get_stake_position(&caller) {
                for (token, new_amount) in &reward_updates {
                    position
                        .claimable_rewards
                        .insert(*token, new_amount.clone());
                }
                position.claim_reward_status = overall_status;
                s.data.stake_system.upsert_stake_position(caller, position);
            }
        });
    }

    // 9. return simplified results
    Ok(results.into_iter().map(|r| r.map(|_| ())).collect())
}

pub async fn checked_transfer_rewards(
    caller: Principal,
    position: StakePosition,
    token: TokenSymbol,
) -> Result<(TokenSymbol, (Nat, ClaimRewardStatus)), ClaimRewardErrors> {
    let token_ledger = token.get_token_info().ledger_id;
    let ledger_fee = token.get_token_info().fee;

    let reward = position.get_reward_by_token(&token);
    position
        .can_claim_reward(&token, &reward)
        .map_err(|e| match e {
            RemoveRewardErrors::InsufficientBalance(_) => {
                ClaimRewardErrors::TokenImbalance("Reward is too small after fee".into())
            }
            RemoveRewardErrors::RewardTokenTypeDoesNotExist(_) => {
                ClaimRewardErrors::InvalidRewardToken(format!(
                    "{:?} is not a recognized reward token.",
                    token
                ))
            }
        })?;

    if ledger_fee > reward {
        return Err(ClaimRewardErrors::TransferError(format!(
            "
            Ledger fee ({ledger_fee}) is greater than reward ({reward}). Skipping transfer."
        )));
    }

    let amount_to_transfer = reward.clone() - ledger_fee;
    let transfer_args = TransferArg {
        from_subaccount: Some(AllocatedRewardsPool::SUBACCOUNT),
        to: Account {
            owner: caller,
            subaccount: None,
        },
        fee: None,
        created_at_time: None,
        memo: None,
        amount: amount_to_transfer,
    };

    match icrc1_transfer(token_ledger, &transfer_args).await {
        Ok(Ok(_)) => Ok((token, (Nat::from(0u64), ClaimRewardStatus::None))),
        Ok(Err(e)) => {
            error!("Transfer error for token {:?}: {:?}", token, e);
            Err(ClaimRewardErrors::TransferError(format!("{e:?}")))
        }
        Err(e) => {
            error!("Call error for token {:?}: {:?}", token, e);
            Err(ClaimRewardErrors::CallError(format!("{e:?}")))
        }
    }
}
