use crate::model::event_transaction::EventTransaction;
use crate::model::event_transaction::StakePositionStateChange;
use crate::state::icrc3_commit_prepared_transaction;
use crate::state::icrc3_prepare_transaction;
use crate::state::{mutate_state, read_state};
use crate::utils::set_withdraw_state_of_position;
use bity_ic_canister_time::timestamp_millis;
use candid::{Nat, Principal};
use gldt_stake_common::accounts::USER_STAKES_POOL;
use gldt_stake_common::manage_stake_position_interface::DissolveInstantlyRequestErrors;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use gldt_stake_common::manage_stake_position_interface::WithdrawErrors;
use gldt_stake_common::stake_position::DecreaseType;
use gldt_stake_common::stake_position::StakeChange;
use gldt_stake_common::stake_position::StakePosition;
use gldt_stake_common::stake_position_event::WithdrawState;
use gldt_stake_common::stake_position_response::StakePositionResponse;
use gldt_stake_common::{ledgers::GLDT_TX_FEE, stake_position_event::DissolveInstantlyStatus};
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::error;
use utils::numeric::Percentage;

pub async fn dissolve_instantly_impl(
    caller: Principal,
    position: &mut StakePosition,
    fraction: u8,
) -> Result<StakePositionResponse, ManageStakePositionError> {
    // 0. validate dissolving eligibility
    let percentage = Percentage::new(fraction).map_err(|e| {
        ManageStakePositionError::GeneralError(GeneralError::InvalidPercentage(e.to_string()))
    })?;
    if percentage == 0 {
        return Err(ManageStakePositionError::GeneralError(
            GeneralError::InvalidPercentage("Dissolve percentage cannot be zero".to_string()),
        ));
    }

    position
        .can_dissolve_instantly(percentage)
        .map_err(DissolveInstantlyRequestErrors::WithdrawErrors)?;

    // 1. calculate amounts
    let proportional_amount_to_withdraw = percentage.apply_to(&position.staked);
    let instant_dissolve_fee =
        position.calculate_dissolve_instantly_fee(proportional_amount_to_withdraw.clone());

    // 2. calculate amount to be sent to user
    if instant_dissolve_fee > proportional_amount_to_withdraw {
        return Err(ManageStakePositionError::GeneralError(
            GeneralError::ModifyStakeError(format!(
                "Instant dissolve fee ({}) exceeds the amount being withdrawn ({}).",
                instant_dissolve_fee, proportional_amount_to_withdraw
            )),
        ));
    }
    let amount_to_user = proportional_amount_to_withdraw.clone() - instant_dissolve_fee.clone(); // including fee

    // 3. set state to in-progress
    set_withdraw_state_of_position(
        caller,
        position,
        WithdrawState::EarlyWithdraw(DissolveInstantlyStatus::InProgress),
    );

    // 4. check if the position is fully dissolved
    if percentage != 100 {
        // not a full dissolve - just decrease stake
        position.change_stake(
            proportional_amount_to_withdraw.clone(),
            StakeChange::Decrease(DecreaseType::Fractional),
        )?;
    } else if position.has_rewards() {
        // full position dissolve with rewards - return error that rewards must be claimed first
        return Err(ManageStakePositionError::DissolveInstantlyError(
            DissolveInstantlyRequestErrors::WithdrawErrors(
                WithdrawErrors::InvalidDissolveInstantlyAmount(format!(
                    "Cannot early withdraw. The stake position has rewards {:?}.",
                    position.claimable_rewards
                )),
            ),
        ));
    } else {
        // full position dissolve without rewards - set stake to zero
        position.change_stake(
            proportional_amount_to_withdraw.clone(),
            StakeChange::Decrease(DecreaseType::Full),
        )?;
    }

    // 5. prepare ICRC3 transaction
    let transaction = EventTransaction::new(StakePositionStateChange::DissolveInstantly {
        fraction,
        amount_dissolved: amount_to_user.clone(),
        result_staked: position.staked.clone(),
    });
    let prepared_tx = icrc3_prepare_transaction(transaction.clone()).map_err(|err| {
        error!("icrc3_prepare_transaction error: {:?}", err);
        ManageStakePositionError::GeneralError(GeneralError::TransactionPreparationError(
            err.to_string(),
        ))
    })?;

    // 6. perform transfer to user
    let stake_position = transfer_stake_to_user(
        amount_to_user,
        caller,
        position,
        instant_dissolve_fee,
        proportional_amount_to_withdraw,
    )
    .await?;

    // 7. commit ICRC3 transaction
    if let Err(e) = icrc3_commit_prepared_transaction(transaction, prepared_tx.timestamp) {
        error!("icrc3_commit_prepared_transaction failed: {:?}", e);
    }

    Ok((stake_position, timestamp_millis()).into())
}

async fn transfer_stake_to_user(
    amount_to_user: Nat,
    caller: Principal,
    position: &mut StakePosition,
    instant_dissolve_fee: Nat,
    amount_to_withdraw: Nat,
) -> Result<&mut StakePosition, ManageStakePositionError> {
    let gldt_ledger = read_state(|s| s.data.gldt_ledger_id);

    if GLDT_TX_FEE >= amount_to_user {
        return Err(ManageStakePositionError::GeneralError(
            GeneralError::TransferError(format!(
                "Transfer fee ({}) is bigger or equals to amount to user ({}).",
                GLDT_TX_FEE, amount_to_user
            )),
        ));
    }
    let transfer_amount = amount_to_user.clone() - Nat::from(GLDT_TX_FEE);

    let transfer_args = TransferArg {
        from_subaccount: Some(USER_STAKES_POOL),
        to: Account {
            owner: caller,
            subaccount: None,
        },
        fee: None,
        created_at_time: None,
        memo: None,
        amount: transfer_amount,
    };

    match icrc1_transfer(gldt_ledger, &transfer_args).await {
        Ok(Ok(_)) => {
            set_withdraw_state_of_position(
                caller,
                position,
                WithdrawState::EarlyWithdraw(DissolveInstantlyStatus::DissolvedInstantly),
            );

            mutate_state(|s| {
                s.data
                    .stake_system
                    .upsert_stake_position(caller, position.clone());
                s.data.stake_system.pending_fee_transfer_amount += instant_dissolve_fee;
                s.data.stake_system.total_staked -= amount_to_withdraw;
            });
            Ok(position)
        }
        Ok(Err(e)) => {
            error!(
                "DISSOLVE INSTANTLY :: Failed :: principal - {} transfer error - {:?}. transfer args - {:?}",
                caller, e, &transfer_args
            );
            set_withdraw_state_of_position(
                caller,
                position,
                WithdrawState::EarlyWithdraw(DissolveInstantlyStatus::Failed(format!("{e:?}"))),
            );
            Err(ManageStakePositionError::GeneralError(
                GeneralError::TransferError(format!("{e:?}")),
            ))
        }
        Err(e) => {
            error!(
                "DISSOLVE INSTANTLY :: Failed :: principal - {} call error - {:?}. transfer args - {:?}",
                caller, e, &transfer_args
            );
            set_withdraw_state_of_position(
                caller,
                position,
                WithdrawState::EarlyWithdraw(DissolveInstantlyStatus::Failed(format!("{e:?}"))),
            );
            Err(ManageStakePositionError::GeneralError(
                GeneralError::CallError(format!("{e:?}")),
            ))
        }
    }
}
