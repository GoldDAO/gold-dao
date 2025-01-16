use crate::memory::{record_event, with_event_iter};
use crate::state::event::{Event, EventType};
use crate::state::State;
use crate::vault::FeeBucket;
use usdg_minter_api::lifecycle::InitArgument;

// Updates the state to reflect the given state transition.
// public because it's used in tests since process_event
// requires canister infrastructure to retrieve time
pub fn apply_state_transition(state: &mut State, payload: EventType, _timestamp: u64) {
    match payload {
        EventType::Init { .. } => panic!("should have only one init event"),
        EventType::Upgrade {
            new_medium_fee_percent,
        } => {
            if let Some(new_medium_fee) = new_medium_fee_percent {
                let new_medium_fee = new_medium_fee as f64 / 100.0;
                state
                    .interest_rates
                    .insert(FeeBucket::Medium, new_medium_fee);
            }
        }
        EventType::OpenVault {
            owner,
            margin_amount,
            borrowed_amount,
            fee_bucket,
            block_index: _,
        } => {
            state.record_vault_creation(owner, borrowed_amount, margin_amount, fee_bucket);
        }
        EventType::Borrow {
            vault_id,
            borrowed_amount,
            block_index: _,
        } => state.record_borrow_from_vault(vault_id, borrowed_amount),
        EventType::AddMargin {
            vault_id,
            margin_added,
            block_index: _,
        } => state.record_add_margin_to_vault(vault_id, margin_added),
        EventType::Repay {
            vault_id,
            debt,
            block_index: _,
        } => state.record_repay_debt_to_vault(vault_id, debt),
        EventType::Close {
            vault_id,
            block_index: _,
        } => state.record_close_vault(vault_id),
        EventType::TransferExecuted {
            transfer_id,
            block_index: _,
        } => state.record_process_pending_transfer(transfer_id),
        EventType::DepositLiquidity {
            caller,
            amount,
            block_index: _,
        } => state.deposit_liquidity(caller, amount),
        EventType::WithdrawLiquidity {
            caller,
            amount,
            block_index: _,
        } => state.withdraw_liquidity(amount, caller),
        EventType::ClaimReturns {
            caller,
            amount,
            block_index: _,
        } => state.record_claimed_returns(caller, amount),
        EventType::Redeem {
            owner,
            current_rate,
            amount,
            block_index: _,
        } => {
            let _ = state.record_redemption(owner, amount, current_rate);
        }
        EventType::ChargeFee => {
            state.update_interest_rate();
            state.charge_fee();
        }
        EventType::Liquidate { vault_id } => {
            state.record_liquidate_vault_liquidation_pool(vault_id);
        }
        EventType::Redistribute { vault_id } => {
            state.record_redistribute_vault(vault_id);
        }
        EventType::UpdateVault {
            vault_id,
            new_owner,
            fee_bucket,
        } => state.record_update_vault(vault_id, new_owner, fee_bucket),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn timestamp_nanos() -> u64 {
    ic_cdk::api::time()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn timestamp_nanos() -> u64 {
    use std::time::SystemTime;

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

/// Records the given event payload in the event log and updates the state to reflect the change.
pub fn process_event(state: &mut State, payload: EventType) {
    let ts = timestamp_nanos();
    apply_state_transition(state, payload.clone(), ts);
    record_event(payload, ts);
}

/// Recomputes the minter state from the event log.
///
/// # Panics
///
/// This function panics if:
///   * The event log is empty.
///   * The first event in the log is not an Init event.
///   * One of the events in the log invalidates the minter's state invariants.
pub fn replay_events() -> State {
    with_event_iter(|mut iter| {
        let mut state = match iter.next().expect("the event log should not be empty") {
            Event {
                payload:
                    EventType::Init {
                        usdg_ledger_id,
                        gldt_ledger_id,
                        gold_dao_governance_id,
                        xrc_id,
                    },
                timestamp: _,
            } => State::new(InitArgument {
                usdg_ledger_id,
                gldt_ledger_id,
                gold_dao_governance_id,
                xrc_id,
            }),
            other => panic!("the first event must be an Init event, got: {other:?}"),
        };
        for event in iter {
            apply_state_transition(&mut state, event.payload, event.timestamp);
        }
        state
    })
}
