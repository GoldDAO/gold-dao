use candid::{Nat, Principal};
use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::unstake::{Args as UnstakeArgs, Response as UnstakeResponse};
use gldt_stake_common::ledgers::GLDT_TX_FEE;
use gldt_stake_common::stake_position::{
    DissolveState, StakePosition, StakePositionId, UnstakeErrors, UnstakeRequestErrors,
};
use gldt_stake_common::stake_position_event::{NormalUnstakeStatus, UnstakeState};
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::error;

use crate::guards::GuardPrincipal;
use crate::model::archive_system::archive_stake_position;
use crate::utils::{commit_changes, set_unstake_state_of_position};
use crate::{
    guards::reject_anonymous_caller,
    state::{mutate_state, read_state},
};
use ic_cdk::{caller, update};

#[update]
#[trace]
async fn unstake(position_id: UnstakeArgs) -> UnstakeResponse {
    unstake_impl(position_id).await
}

async fn unstake_impl(position_id: UnstakeArgs) -> UnstakeResponse {
    // 1. check user isn't anon
    let caller = caller();
    reject_anonymous_caller().map_err(UnstakeRequestErrors::InvalidPrincipal)?;
    let _guard_principal = GuardPrincipal::new(caller)
        .map_err(|e| UnstakeRequestErrors::UnstakeErrors(UnstakeErrors::AlreadyProcessing(e)))?;

    // find the position
    let position = read_state(|s| s.data.stake_system.get_stake_position(position_id)).ok_or(
        UnstakeRequestErrors::NotFound(
            "Cant find active stake position with ID : {position_id}".to_string(),
        ),
    )?;

    if position.owned_by != caller {
        return Err(UnstakeRequestErrors::NotAuthorized(
            "You do not have permission to unstake this stake position".to_string(),
        ));
    }

    position
        .can_unstake()
        .map_err(UnstakeRequestErrors::UnstakeErrors)?;

    let amount_to_unstake = position.staked.clone();
    let amount_to_transfer = amount_to_unstake.clone() - GLDT_TX_FEE;
    set_unstake_state_of_position(
        &position_id,
        &position,
        UnstakeState::NormalUnstake(NormalUnstakeStatus::InProgress),
    );
    commit_changes().await;

    let stake_position = transfer_stake_to_user(
        amount_to_transfer,
        amount_to_unstake,
        caller,
        position_id,
        position,
    )
    .await?;

    let position_id_to_archive = position_id;
    let position_to_archive = stake_position.clone();
    ic_cdk::spawn(async move {
        let _ = archive_stake_position(position_id_to_archive, position_to_archive).await;
    });

    Ok((stake_position, timestamp_millis(), position_id).into())
}

async fn transfer_stake_to_user(
    amount_to_transfer: Nat,
    amount_to_unstake: Nat,
    caller: Principal,
    position_id: StakePositionId,
    position: StakePosition,
) -> Result<StakePosition, UnstakeRequestErrors> {
    let gldt_ledger = read_state(|s| s.data.gldt_ledger_id);
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
    match icrc1_transfer(gldt_ledger, &transfer_args).await {
        Ok(Ok(_)) => {
            set_unstake_state_of_position(
                &position_id,
                &position,
                UnstakeState::NormalUnstake(NormalUnstakeStatus::Unstaked),
            );

            mutate_state(|s| {
                let mut updated_position = position.clone();
                updated_position.dissolve_state = DissolveState::Dissolved;
                updated_position.staked = Nat::from(0u64);
                s.data
                    .stake_system
                    .update_stake_position(&position_id, updated_position.clone());
                s.data.stake_system.total_staked -= amount_to_unstake;

                Ok(updated_position)
            })
        }
        Ok(Err(e)) => {
            error!(
                "UNSTAKE :: Failed :: position id - {} transfer error - {:?}. transfer args - {:?}",
                position_id, e, &transfer_args
            );
            set_unstake_state_of_position(
                &position_id,
                &position,
                UnstakeState::NormalUnstake(NormalUnstakeStatus::Failed(format!("{e:?}"))),
            );
            Err(UnstakeRequestErrors::TransferError(format!("{e:?}")))
        }
        Err(e) => {
            error!(
                "UNSTAKE :: Failed :: position id - {} call error - {:?}. transfer args - {:?}",
                position_id, e, &transfer_args
            );
            set_unstake_state_of_position(
                &position_id,
                &position,
                UnstakeState::NormalUnstake(NormalUnstakeStatus::Failed(format!("{e:?}"))),
            );
            Err(UnstakeRequestErrors::CallError(format!("{e:?}")))
        }
    }
}
