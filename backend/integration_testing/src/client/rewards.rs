use crate::{ generate_pocket_query_call, generate_pocket_update_call };

use sns_rewards_api_canister::*;

generate_pocket_update_call!(add_neuron_ownership);
generate_pocket_update_call!(remove_neuron_ownership);
generate_pocket_update_call!(claim_reward);
