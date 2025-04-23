use candid::Nat;
use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::claim_reward::{
    Args as ClaimRewardArgs, Response as ClaimRewardResponse,
};
use gldt_stake_common::{
    stake_position::{ClaimRewardErrors, RemoveRewardErrors, StakePosition, StakePositionId},
    stake_position_event::ClaimRewardStatus,
};
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use tracing::error;

use crate::{
    guards::{reject_anonymous_caller, GuardPrincipal},
    state::{mutate_state, read_state},
    utils::commit_changes,
};
use ic_cdk::{caller, update};

#[update]
#[trace]
async fn claim_reward(args: ClaimRewardArgs) -> ClaimRewardResponse {
    claim_reward_impl(args).await
}

async fn claim_reward_impl(args: ClaimRewardArgs) -> ClaimRewardResponse {
    let caller = caller();

    reject_anonymous_caller().map_err(ClaimRewardErrors::InvalidPrincipal)?;

    let _guard_principal =
        GuardPrincipal::new(caller).map_err(ClaimRewardErrors::AlreadyProcessing)?;

    // find the position
    let mut position = read_state(|s| s.data.stake_system.get_stake_position(args.id)).ok_or(
        ClaimRewardErrors::NotFound(format!(
            "Cant find active stake position with ID : {}",
            args.id
        )),
    )?;

    // check ownership
    if position.owned_by != caller {
        return Err(ClaimRewardErrors::NotAuthorized(
            "You do not have permission to claim rewards with this stake position".to_string(),
        ));
    }
    // check token type is correct
    let (token_ledger, ledger_fee) =
        read_state(|s| s.data.stake_system.reward_types.get(&args.token).cloned()).ok_or(
            ClaimRewardErrors::InvalidRewardToken(format!(
                "{} is not a recognized reward token. please pass a correct reward token",
                args.token
            )),
        )?;

    // check can claim rewards
    let reward = position.get_reward_by_token(&args.token);
    position.can_claim_reward(&args.token, &reward).map_err(|e| {
        match e {
            RemoveRewardErrors::InsufficientBalance(_) => {
                ClaimRewardErrors::TokenImbalance("Cant claim rewards because the fee to transfer is higher than the reward balance and would result in a failed transfer or a 0 reward transfer".to_string())
            }
            RemoveRewardErrors::RewardTokenTypeDoesNotExist(_) => {
                ClaimRewardErrors::InvalidRewardToken(format!(
                    "{} is not a recognized reward token. please pass a correct reward token",
                    args.token
                ))
            }
        }
    })?;

    reject_stake_position_for_invalid_state(&position, &args.id)?;

    set_status_of_position(&args.id, &position, ClaimRewardStatus::InProgress);
    commit_changes().await;

    // transfer the reward
    let amount_to_transfer = reward.clone() - ledger_fee;
    let transfer_args = TransferArg {
        from_subaccount: None,
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
        Ok(Ok(_)) => mutate_state(|s| {
            position
                .claimable_rewards
                .insert(args.token, Nat::from(0u64));
            position.claim_reward_status = ClaimRewardStatus::None;

            s.data
                .stake_system
                .update_stake_position(&args.id, position.clone());

            Ok((position, timestamp_millis(), args.id).into())
        }),
        Ok(Err(e)) => {
            error!(
                "CLAIM REWARD :: Failed :: position id - {} transfer error - {:?}. transfer args - {:?}",
                args.id, e, &transfer_args
            );
            set_status_of_position(
                &args.id,
                &position,
                ClaimRewardStatus::Failed(format!("{e:?}")),
            );
            Err(ClaimRewardErrors::TransferError(format!("{e:?}")))
        }
        Err(e) => {
            error!(
                "CLAIM REWARD :: Failed :: position id - {} call error - {:?}. transfer args - {:?}",
                args.id, e, &transfer_args
            );
            set_status_of_position(
                &args.id,
                &position,
                ClaimRewardStatus::Failed(format!("{e:?}")),
            );
            Err(ClaimRewardErrors::CallError(format!("{e:?}")))
        }
    }
}

fn reject_stake_position_for_invalid_state(
    position: &StakePosition,
    position_id: &StakePositionId,
) -> Result<(), ClaimRewardErrors> {
    match position.claim_reward_status {
        ClaimRewardStatus::None => Ok(()),
        ClaimRewardStatus::Failed(_) => {
            let mut updated_position = position.clone();
            updated_position.claim_reward_status = ClaimRewardStatus::None;
            mutate_state(|s| {
                s.data
                    .stake_system
                    .update_stake_position(position_id, updated_position)
            });
            Ok(())
        }
        ClaimRewardStatus::InProgress => Err(ClaimRewardErrors::AlreadyProcessing(
            "reward claim process has already been initiated and is in progress".to_string(),
        )),
    }
}

fn set_status_of_position(
    stake_position_id: &StakePositionId,
    stake_position: &StakePosition,
    new_status: ClaimRewardStatus,
) {
    let mut updated_position = stake_position.clone();
    updated_position.claim_reward_status = new_status;
    mutate_state(|s| {
        s.data
            .stake_system
            .update_stake_position(stake_position_id, updated_position)
    });
}
