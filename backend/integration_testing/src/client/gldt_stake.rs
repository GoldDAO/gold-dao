use crate::{generate_pocket_query_call, generate_pocket_update_call};
use gldt_stake_api_canister::queries::*;
use gldt_stake_api_canister::updates::*;

// Queries
generate_pocket_query_call!(get_neurons);
generate_pocket_query_call!(get_active_user_positions);
generate_pocket_query_call!(get_position_by_id);
generate_pocket_query_call!(get_total_allocated_rewards);
generate_pocket_query_call!(get_total_staked);
generate_pocket_query_call!(get_archive_canisters);
generate_pocket_query_call!(get_reward_rounds);
generate_pocket_query_call!(get_apy_timeseries);

// Updates
generate_pocket_update_call!(create_neuron);
generate_pocket_update_call!(create_stake_position);
generate_pocket_update_call!(start_dissolving);
generate_pocket_update_call!(claim_reward);
generate_pocket_update_call!(unstake);
generate_pocket_update_call!(unstake_early);
generate_pocket_update_call!(_add_reward_round);
generate_pocket_update_call!(process_oldest_reward_round);
generate_pocket_update_call!(_set_position_unstake_state);
generate_pocket_update_call!(get_historic_positions_by_user);
generate_pocket_update_call!(get_historic_position_by_id);
generate_pocket_update_call!(get_historic_positions_total_by_user);
generate_pocket_update_call!(_set_token_usd_values);
