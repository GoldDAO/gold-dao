use crate::utils::tick_n_blocks;
use crate::{generate_pocket_query_call, generate_pocket_update_call};
use gldt_stake_api_canister::queries::*;
use gldt_stake_api_canister::updates::*;

// Queries
generate_pocket_query_call!(_get_state_snapshot);
generate_pocket_query_call!(get_neurons);
generate_pocket_query_call!(get_position);
generate_pocket_query_call!(get_total_allocated_rewards);
generate_pocket_query_call!(get_total_staked);
generate_pocket_query_call!(get_apy_timeseries);
generate_pocket_query_call!(get_daily_analytics);
generate_pocket_query_call!(get_proposal_votes_of_neuron);
generate_pocket_query_call!(icrc3_get_properties);
generate_pocket_query_call!(icrc3_get_blocks);
generate_pocket_query_call!(icrc3_get_tip_certificate);
generate_pocket_query_call!(icrc3_supported_block_types);
generate_pocket_query_call!(icrc3_get_archives);

// Updates
generate_pocket_update_call!(create_neuron);
generate_pocket_update_call!(manage_sns_neuron);
generate_pocket_update_call!(manage_stake_position);
generate_pocket_update_call!(set_apy_limit);
generate_pocket_update_call!(get_apy_overall);
generate_pocket_update_call!(_set_position_withdraw_state);
generate_pocket_update_call!(_add_stake_mock);
generate_pocket_update_call!(_set_token_usd_values);
generate_pocket_update_call!(allocated_rewards_balance);
generate_pocket_update_call!(unallocated_rewards_balance);
generate_pocket_update_call!(processing_rewards_balance);

pub fn manage_stake_position_with_tick(
    pic: &pocket_ic::PocketIc,
    sender: candid::Principal,
    canister_id: candid::Principal,
    args: &manage_stake_position::Args,
) -> manage_stake_position::Response {
    let result = manage_stake_position(pic, sender, canister_id, args);
    tick_n_blocks(pic, 10);
    result
}
