use crate::client::gldt_stake::_get_state_snapshot;
use crate::client::gldt_stake::manage_stake_position_with_tick;
use crate::gldt_stake_suite::setup::default_test_setup;
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::create_stake_position_util;
use crate::utils::tick_n_blocks;
use candid::Nat;
use gldt_stake_api_canister::_get_state_snapshot::StateSnapshot;
pub use gldt_stake_api_canister::manage_stake_position::Response as ManageStakePositionResponse;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionArgs;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionArgs::AddStake;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionArgs::ClaimRewards;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionArgs::DissolveInstantly;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionArgs::StartDissolving;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionArgs::Withdraw;
use rand::prelude::IndexedRandom;
use rand::Rng;
use types::TokenSymbol;

fn random_action() -> ManageStakePositionArgs {
    let mut rng = rand::rng();
    match rng.random_range(0..=5) {
        0 => {
            let base = rng.random_range(500_000_000u128..5_000_000_000u128);
            let bonus = rng
                .random_bool(0.2)
                .then(|| 10_000_000_000u128)
                .unwrap_or(0);
            AddStake {
                amount: Nat::from(base + bonus),
            }
        }
        1 => {
            let tokens = [TokenSymbol::GLDT];
            let mut selected = vec![];
            let count = rng.random_range(1..=tokens.len());
            for _ in 0..count {
                let token = *tokens.choose(&mut rng).unwrap();
                if !selected.contains(&token) {
                    selected.push(token);
                }
            }
            ClaimRewards { tokens: selected }
        }
        2 => {
            // Start dissolving with skew toward small or full dissolves
            let fraction = if rng.random_bool(0.1) {
                100 // Full dissolve
            } else {
                rng.random_range(1..100)
            };
            StartDissolving { fraction }
        }
        3 => {
            // Instant dissolve, possibly large
            let fraction = if rng.random_bool(0.05) {
                100 // Full instant dissolve
            } else {
                rng.random_range(1..=100)
            };
            DissolveInstantly { fraction }
        }
        4 => Withdraw {},
        5 => {
            // Edge test: stake zero or minimal amount (invalid or edge case)
            let amount = if rng.random_bool(0.5) {
                0
            } else {
                rng.random_range(1..1_000_000u128)
            };
            AddStake {
                amount: Nat::from(amount),
            }
        }
        _ => unreachable!(),
    }
}

fn validate_state_transition(
    before: &StateSnapshot,
    after: &StateSnapshot,
    input: &ManageStakePositionArgs,
    response: &ManageStakePositionResponse,
) -> bool {
    println!(
        "Input: {:?}, Response: {:?}, Before: {:?}, After: {:?}",
        input, response, before, after
    );
    match response {
        Ok(_) => {
            match input {
                ManageStakePositionArgs::AddStake { .. } => {
                    // Staked should increase
                    let before_staked = before
                        .position
                        .as_ref()
                        .map(|p| p.staked.clone())
                        .unwrap_or_default();
                    let after_staked = after
                        .position
                        .as_ref()
                        .map(|p| p.staked.clone())
                        .unwrap_or_default();
                    if after_staked < before_staked {
                        println!(
                            "AddStake: after_staked < before_staked! {} < {}",
                            after_staked, before_staked
                        );
                        return false;
                    }
                }
                ManageStakePositionArgs::DissolveInstantly { .. }
                | ManageStakePositionArgs::StartDissolving { .. } => {
                    // Total staked should decrease (or stay the same if already zero)
                    let before_total = before.total_staked.clone();
                    let after_total = after.total_staked.clone();
                    if after_total > before_total {
                        println!(
                            "Dissolve: after_total_staked > before_total_staked! {} > {}",
                            after_total, before_total
                        );
                        return false;
                    }
                }
                _ => {}
            }
            // If the action succeeded, the position must have changed
            before.position != after.position
        }
        Err(_) => {
            // If the action failed, the position must be unchanged
            before.position.clone().unwrap().staked == after.position.clone().unwrap().staked
        }
    }
}

#[test]
fn fuzz_test_manage_stake_position() {
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;

    let pic = &pic.borrow();

    // --- Create stake position ---
    let (user, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        100_000_000_000_u128,
    );

    let num_actions = 10;
    for i in 0..num_actions {
        let snapshot_before = _get_state_snapshot(pic, user, gldt_stake_canister_id, &());
        let action = random_action();

        println!("Action {}: {:?}", i, action);

        let response = manage_stake_position_with_tick(pic, user, gldt_stake_canister_id, &action);
        println!("Response {}: {:?}", i, response);
        tick_n_blocks(pic, 1);

        let snapshot_after = _get_state_snapshot(pic, user, gldt_stake_canister_id, &());

        // Add your assertions/invariants here
        assert!(validate_state_transition(
            &snapshot_before,
            &snapshot_after,
            &action,
            &response
        ));
    }
}
