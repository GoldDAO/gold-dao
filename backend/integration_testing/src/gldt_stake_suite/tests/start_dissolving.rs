use gldt_stake_common::stake_position::DissolveState;

use crate::client::gldt_stake::{get_active_user_positions, start_dissolving};
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::create_stake_position_util;
use crate::{gldt_stake_suite::setup::default_test_setup, utils::tick_n_blocks};

#[test]
fn test_start_dissolving() {
    let mut test_env = default_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;

    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user_0, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );

    let user_positions = get_active_user_positions(pic, user_0, gldt_stake_canister_id, &None);
    assert_eq!(user_positions.len(), 1);

    let position = user_positions.get(0).unwrap();

    let response = start_dissolving(pic, user_0, gldt_stake_canister_id, &position.id).unwrap();

    assert_eq!(response.dissolve_state, DissolveState::Dissolving);

    tick_n_blocks(pic, 1);

    let user_positions = get_active_user_positions(pic, user_0, gldt_stake_canister_id, &None);
    assert_eq!(user_positions.len(), 1);

    let position = user_positions.get(0).unwrap();
    assert_eq!(position.dissolve_state, DissolveState::Dissolving);
}
