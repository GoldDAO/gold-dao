use candid_gen::generate_candid_method;

pub mod claim_rewards;
pub mod set_reward_token_types;
pub mod remove_neuron_ownership;
pub mod add_neuron_ownership;
pub mod set_reserve_transfer_amount;

generate_candid_method!(sns_rewards_api, add_neuron_ownership, update);
generate_candid_method!(sns_rewards_api, remove_neuron_ownership, update);
generate_candid_method!(sns_rewards_api, claim_reward, update);
