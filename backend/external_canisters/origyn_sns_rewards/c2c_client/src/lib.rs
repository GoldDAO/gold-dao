use canister_client::generate_candid_c2c_call;
use ogy_sns_rewards_api_canister::*;

// Queries

// Updates
generate_candid_c2c_call!(add_neuron_ownership);
generate_candid_c2c_call!(claim_reward);
