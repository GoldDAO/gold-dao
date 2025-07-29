use crate::model::event_transaction::EventTransaction;
use crate::model::event_transaction::StakePositionStateChange;
use crate::state::icrc3_commit_prepared_transaction;
use crate::state::icrc3_prepare_transaction;
use crate::state::{mutate_state, read_state};
use crate::utils::set_withdraw_state_of_position;
use bity_ic_canister_time::timestamp_millis;
use candid::{Nat, Principal};
use gldt_stake_common::accounts::USER_STAKES_POOL;
use gldt_stake_common::ledgers::GLDT_TX_FEE;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use gldt_stake_common::manage_stake_position_interface::WithdrawErrors;
use gldt_stake_common::manage_stake_position_interface::WithdrawRequestErrors;
use gldt_stake_common::stake_position::StakePosition;
use gldt_stake_common::stake_position_event::{NormalWithdrawStatus, WithdrawState};
use gldt_stake_common::stake_position_response::StakePositionResponse;
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::error;

pub async fn withdraw_impl(
    caller: Principal,
    position: StakePosition,
) -> Result<StakePositionResponse, ManageStakePositionError> {
    // 0. validate withdraw eligibility
    if position.staked == 0_u64 && position.has_rewards() {
        return Err(ManageStakePositionError::WithdrawError(
            WithdrawRequestErrors::WithdrawErrors(WithdrawErrors::CantWithdrawWithRewardsBalance(
                "This stake position has rewards available to claim.".to_string(),
            )),
        ));
    }

    position
        .can_withdraw()
        .map_err(WithdrawRequestErrors::WithdrawErrors)?;

    // 1. calculate amounts
    let amount_to_withdraw = position.amount_available_for_withdraw();
    let dissolved_events = position.completed_dissolve_events();
    // let amount_to_transfer = amount_to_withdraw.clone() - GLDT_TX_FEE;

    set_withdraw_state_of_position(
        caller,
        &position,
        WithdrawState::NormalWithdraw(NormalWithdrawStatus::InProgress),
    );

    // 2. prepare ICRC3 transaction
    let transaction = EventTransaction::new(StakePositionStateChange::Withdraw {
        amount_withdrawn: amount_to_withdraw.clone(),
        dissolved_events,
    });
    let prepared_tx = match icrc3_prepare_transaction(transaction.clone()) {
        Ok(tx) => tx,
        Err(err) => {
            set_withdraw_state_of_position(
                caller,
                &position,
                WithdrawState::NormalWithdraw(NormalWithdrawStatus::Failed(format!(
                    "Transaction preparation failed: {err}"
                ))),
            );
            return Err(ManageStakePositionError::GeneralError(
                GeneralError::TransactionPreparationError(err.to_string()),
            ));
        }
    };

    // 3. perform transfer to user
    let stake_position = transfer_stake_to_user(amount_to_withdraw, caller, position).await?;

    // 4. commit ICRC3 transaction
    if let Err(e) = icrc3_commit_prepared_transaction(transaction, prepared_tx.timestamp) {
        error!(
            "[withdraw_impl] icrc3_commit_prepared_transaction failed: {:?}",
            e
        );
    }

    Ok((stake_position, timestamp_millis()).into())
}

async fn transfer_stake_to_user(
    amount_to_withdraw: Nat,
    caller: Principal,
    position: StakePosition,
) -> Result<StakePosition, WithdrawRequestErrors> {
    let gldt_ledger = read_state(|s| s.data.gldt_ledger_id);

    if GLDT_TX_FEE >= amount_to_withdraw {
        return Err(WithdrawRequestErrors::TransferError(format!(
            "
            Ledger fee ({GLDT_TX_FEE}) is bigger or equals to amount to user ({amount_to_withdraw}). Skipping transfer."
        )));
    }

    let amount_to_transfer = amount_to_withdraw.clone() - GLDT_TX_FEE;
    let transfer_args = TransferArg {
        from_subaccount: Some(USER_STAKES_POOL),
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
            set_withdraw_state_of_position(
                caller,
                &position,
                WithdrawState::NormalWithdraw(NormalWithdrawStatus::Withdrawn),
            );

            let updated_position = mutate_state(|s| {
                let mut updated_position = position.clone();
                let _ = updated_position.cleanup_completed_dissolve_events();

                s.data
                    .stake_system
                    .upsert_stake_position(caller, updated_position.clone());
                s.data.stake_system.total_staked -= amount_to_withdraw;

                updated_position
            });

            Ok(updated_position)
        }
        Ok(Err(e)) => {
            error!(
                "WITHDRAW :: Failed :: principal - {} transfer error - {:?}. transfer args - {:?}",
                caller, e, &transfer_args
            );
            set_withdraw_state_of_position(
                caller,
                &position,
                WithdrawState::NormalWithdraw(NormalWithdrawStatus::Failed(format!("{e:?}"))),
            );
            Err(WithdrawRequestErrors::TransferError(format!("{e:?}")))
        }
        Err(e) => {
            error!(
                "WITHDRAW :: Failed :: principal - {} call error - {:?}. transfer args - {:?}",
                caller, e, &transfer_args
            );
            set_withdraw_state_of_position(
                caller,
                &position,
                WithdrawState::NormalWithdraw(NormalWithdrawStatus::Failed(format!("{e:?}"))),
            );
            Err(WithdrawRequestErrors::CallError(format!("{e:?}")))
        }
    }
}
