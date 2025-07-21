use crate::model::event_transaction::EventTransaction;
use crate::model::event_transaction::StakePositionStateChange;
use crate::state::icrc3_commit_prepared_transaction;
use crate::state::icrc3_prepare_transaction;
use crate::state::mutate_state;
use crate::state::read_state;
use bity_ic_canister_time::timestamp_millis;
use candid::Nat;
use candid::Principal;
use gldt_stake_common::accounts::USER_STAKES_POOL;
use gldt_stake_common::ledgers::GLDT_TX_FEE;
use gldt_stake_common::manage_stake_position_interface::AddStakePositionErrors;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use gldt_stake_common::stake_position::StakeChange;
use gldt_stake_common::stake_position::StakePosition;
use gldt_stake_common::stake_position_response::StakePositionResponse;
use icrc_ledger_canister_c2c_client::icrc2_transfer_from;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc2::transfer_from::TransferFromArgs;
use tracing::error;
use utils::env::Environment;
use utils::retry_async::retry_async;

pub async fn add_stake_impl(
    caller: Principal,
    amount: Nat,
) -> Result<StakePositionResponse, ManageStakePositionError> {
    // 0. validate adding state eligibility
    read_state(|s| {
        s.is_canister_at_capacity_heap().map_err(|e| {
            ManageStakePositionError::AddStakeError(AddStakePositionErrors::CapacityExceeded(e))
        })
    })?;

    let gldt_fee = Nat::from(GLDT_TX_FEE);
    if gldt_fee >= amount {
        return Err(ManageStakePositionError::GeneralError(
            GeneralError::ModifyStakeError(format!(
                "Transfer fee ({}) exceeds amount to transfer ({}).",
                GLDT_TX_FEE, amount
            )),
        ));
    }
    let stake_amount = amount.clone() - gldt_fee;

    // 1. validate and prepare updated stake position (without mutating state yet)
    let updated_position = read_state(|s| {
        let stake_system = &s.data.stake_system;
        if let Some(existing_position) = stake_system.get_stake_position(&caller) {
            let mut new_position = existing_position.clone();
            new_position.change_stake(stake_amount.clone(), StakeChange::Increase)?;
            Ok(new_position)
        } else {
            if !stake_system.can_add_new_stake_position() {
                return Err(ManageStakePositionError::AddStakeError(
                    AddStakePositionErrors::MaxAllowedStakePositions(format!(
                        "Cannot add more stake positions. Limit of {} reached.",
                        stake_system.stake_positions_quantity_limit
                    )),
                ));
            }

            StakePosition::try_new(caller, stake_amount.clone())
                .map_err(ManageStakePositionError::AddStakeError)
        }
    })?;

    // 2. prepare ICRC3 transaction
    let transaction = EventTransaction::new(StakePositionStateChange::AddStake {
        amount_added: stake_amount.clone(),
        result_staked: updated_position.staked.clone(),
    });
    let prepared_tx = icrc3_prepare_transaction(transaction.clone()).map_err(|err| {
        error!("icrc3_prepare_transaction error: {:?}", err);
        ManageStakePositionError::GeneralError(GeneralError::TransactionPreparationError(
            err.to_string(),
        ))
    })?;

    // 3. perform token transfer
    transfer_gldt(&stake_amount).await.map_err(|e| {
        ManageStakePositionError::GeneralError(GeneralError::TransferError(e.to_string()))
    })?;

    // 4. commit ICRC3 transaction
    if let Err(e) = icrc3_commit_prepared_transaction(transaction, prepared_tx.timestamp) {
        error!("icrc3_commit_prepared_transaction failed: {:?}", e);
    }

    // 5. mutate state only after transfer is successful
    mutate_state(|s| {
        let stake_system = &mut s.data.stake_system;

        stake_system.upsert_stake_position(caller, updated_position.clone());
        stake_system.total_staked += stake_amount;
    });

    Ok((updated_position, timestamp_millis()).into())
}

pub async fn transfer_gldt(amount: &Nat) -> Result<Nat, String> {
    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());
    let caller = ic_cdk::api::msg_caller();
    let transfer_from_args = TransferFromArgs {
        spender_subaccount: None,
        from: Account {
            owner: caller,
            subaccount: None,
        },
        to: Account {
            owner: this_canister_id,
            subaccount: Some(USER_STAKES_POOL),
        },
        amount: amount.clone(),
        fee: Some(GLDT_TX_FEE.into()),
        memo: None,
        created_at_time: None,
    };

    match retry_async(
        || icrc2_transfer_from(gldt_ledger_id, transfer_from_args.clone()),
        3,
    )
    .await
    {
        Ok(transfer_response) => match transfer_response {
            icrc_ledger_canister::icrc2_transfer_from::Response::Ok(_) => {
                if GLDT_TX_FEE >= *amount {
                    return Err(format!(
                        "Transfer fee ({}) exceeds amount to transfer ({}).",
                        GLDT_TX_FEE, amount
                    ));
                }

                Ok(amount.clone() - Nat::from(GLDT_TX_FEE))
            }
            icrc_ledger_canister::icrc2_transfer_from::Response::Err(e) => {
                error!("TRANSFER ERROR: {e:?}, args: {:?}", transfer_from_args);
                Err(format!("{e:?}"))
            }
        },
        Err(e) => {
            error!("TRANSFER ERROR: {e:?}, args: {:?}", transfer_from_args);
            Err(format!("{e:?}"))
        }
    }
}
