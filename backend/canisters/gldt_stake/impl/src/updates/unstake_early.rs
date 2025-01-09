use candid::Nat;
use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::unstake_early::{
    Args as UnstakeEarlyArgs, Response as UnstakeEarlyResponse,
};
use gldt_stake_common::stake_position::{DissolveState, UnstakeEarlyRequestErrors};
use gldt_stake_common::stake_position_event::UnstakeState;
use gldt_stake_common::{ledgers::GLDT_TX_FEE, stake_position_event::UnstakeEarlyStatus};
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::error;

use crate::guards::GuardPrincipal;
use crate::utils::{commit_changes, set_unstake_state_of_position};
use crate::{
    guards::reject_anonymous_caller,
    state::{mutate_state, read_state},
};
use ic_cdk::{caller, update};

#[update]
#[trace]
async fn unstake_early(position_id: UnstakeEarlyArgs) -> UnstakeEarlyResponse {
    unstake_early_impl(position_id).await
}

async fn unstake_early_impl(position_id: UnstakeEarlyArgs) -> UnstakeEarlyResponse {
    // 1. check user isn't anon
    let caller = caller();
    reject_anonymous_caller().map_err(|e| UnstakeEarlyRequestErrors::InvalidPrincipal(e))?;
    let _guard_principal =
        GuardPrincipal::new(caller).map_err(|e| UnstakeEarlyRequestErrors::AlreadyProcessing(e))?;

    // find the position
    let position = read_state(|s| s.data.stake_system.get_stake_position(position_id)).ok_or(
        UnstakeEarlyRequestErrors::NotFound(format!(
            "Cant find active stake position with ID : {position_id}"
        )),
    )?;

    if position.owned_by != caller {
        return Err(UnstakeEarlyRequestErrors::NotAuthorized(format!(
            "You do not have permission to unstake this stake position early"
        )));
    }

    position
        .can_unstake_early()
        .map_err(|e| UnstakeEarlyRequestErrors::UnstakeErrors(e))?;

    let early_unstake_fee = position.calculate_unstake_early_fee();
    let position_stake = position.staked.clone();
    let amount_to_unstake = position.staked.clone() - early_unstake_fee.clone();
    let amount_for_user = amount_to_unstake - GLDT_TX_FEE;
    let gldt_ledger = read_state(|s| s.data.gldt_ledger_id);

    set_unstake_state_of_position(
        &position_id,
        &position,
        UnstakeState::EarlyUnstake(UnstakeEarlyStatus::InProgress),
    );
    commit_changes().await;

    let transfer_args = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: caller,
            subaccount: None,
        },
        fee: None,
        created_at_time: None,
        memo: None,
        amount: amount_for_user,
    };

    match icrc1_transfer(gldt_ledger, &transfer_args).await {
        Ok(Ok(_)) => {
            set_unstake_state_of_position(
                &position_id,
                &position,
                UnstakeState::EarlyUnstake(UnstakeEarlyStatus::UnstakedEarly),
            );
            let mut updated_position = position.clone();
            updated_position.dissolve_state = DissolveState::Dissolved;
            updated_position.dissolved_date = Some(timestamp_millis());
            updated_position.staked = Nat::from(0u64);
            mutate_state(|s| {
                s.data
                    .stake_system
                    .update_stake_position(&position_id, updated_position.clone());
                s.data.stake_system.pending_fee_transfer_amount += early_unstake_fee;
                s.data.stake_system.total_staked -= position_stake;
            });
            Ok((updated_position, timestamp_millis(), position_id).into())
        }
        Ok(Err(e)) => {
            error!(
                "UNSTAKE EARLY :: Failed :: position id - {} transfer error - {:?}. transfer args - {:?}",
                position_id, e, &transfer_args
            );
            set_unstake_state_of_position(
                &position_id,
                &position,
                UnstakeState::EarlyUnstake(UnstakeEarlyStatus::Failed(format!("{e:?}"))),
            );
            Err(UnstakeEarlyRequestErrors::TransferError(format!("{e:?}")))
        }
        Err(e) => {
            error!(
                "UNSTAKE EARLY :: Failed :: position id - {} call error - {:?}. transfer args - {:?}",
                position_id, e, &transfer_args
            );
            set_unstake_state_of_position(
                &position_id,
                &position,
                UnstakeState::EarlyUnstake(UnstakeEarlyStatus::Failed(format!("{e:?}"))),
            );
            Err(UnstakeEarlyRequestErrors::CallError(format!("{e:?}")))
        }
    }
}
