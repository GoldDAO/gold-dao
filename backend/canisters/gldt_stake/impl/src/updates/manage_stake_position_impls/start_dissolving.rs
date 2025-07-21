use crate::model::event_transaction::EventTransaction;
use crate::model::event_transaction::StakePositionStateChange;
use crate::state::mutate_state;
use candid::Nat;
use candid::Principal;
use canister_time::timestamp_millis;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use gldt_stake_common::stake_position::DecreaseType;
use gldt_stake_common::stake_position::StakeChange;
use gldt_stake_common::stake_position::StakePosition;
use gldt_stake_common::stake_position_event::DissolveStakeEvent;
use gldt_stake_common::stake_position_response::StakePositionResponse;
use tracing::error;
use utils::numeric::Percentage;

pub fn start_dissolving_impl(
    caller: Principal,
    position: &mut StakePosition,
    fraction: u8,
) -> Result<StakePositionResponse, ManageStakePositionError> {
    // 0. validate dissolving eligibility
    position.can_start_dissolving()?;

    let percentage = Percentage::new(fraction).map_err(|e| {
        ManageStakePositionError::GeneralError(GeneralError::InvalidPercentage(e.to_string()))
    })?;

    if percentage == 0 {
        return Err(ManageStakePositionError::GeneralError(
            GeneralError::InvalidPercentage("Dissolve percentage cannot be zero".to_string()),
        ));
    }

    // 1. calculate amounts
    let decrease_amount = percentage.apply_to(&position.staked);
    let dissolve_at = timestamp_millis() + position.dissolve_delay.as_millis() as u64;

    // 2. insert dissolve event into position
    position.insert_event(DissolveStakeEvent {
        percentage,
        dissolved_date: dissolve_at,
        amount: decrease_amount.clone(),
        completed: false,
    });

    // 3. check if the position is fully dissolved
    if percentage != 100 {
        position.change_stake(
            decrease_amount.clone(),
            StakeChange::Decrease(DecreaseType::Fractional),
        )?;
    } else {
        position.staked = Nat::from(0u64);
    }

    // 4. add EventTransaction before mutation
    let transaction = EventTransaction::new(StakePositionStateChange::StartDissolving {
        fraction,
        amount_dissolved: decrease_amount.clone(),
        result_staked: position.staked.clone(),
        dissolve_events: position.dissolve_events.clone(),
    });

    if let Err(e) = crate::state::icrc3_add_transaction(transaction) {
        error!("icrc3_add_transaction failed: {:?}", e);
        return Err(ManageStakePositionError::GeneralError(
            GeneralError::TransactionAddError(e.to_string()),
        ));
    }

    // 5. mutate state with upserting position
    mutate_state(|s| {
        s.data
            .stake_system
            .upsert_stake_position(caller, position.clone())
    });

    Ok((position, timestamp_millis()).into())
}
