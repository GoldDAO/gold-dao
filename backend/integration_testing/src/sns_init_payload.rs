use std::collections::BTreeMap;

use candid::Principal;
use sns_governance_canister::types::{
    governance::SnsMetadata,
    DefaultFollowees,
    Governance,
    NervousSystemParameters,
    NeuronPermissionList,
    VotingRewardsParameters,
};

pub fn get_sns_init_args(
    sns_ledger_canister_id: Principal,
    sns_root_canister_id: Principal,
    sns_swap_canister_id: Principal
) -> Governance {
    return Governance {
        deployed_version: None,
        neurons: BTreeMap::new(),
        proposals: BTreeMap::new(),
        parameters: Some(NervousSystemParameters {
            default_followees: Some(DefaultFollowees {
                followees: BTreeMap::new(),
            }),
            reject_cost_e8s: Some(10000u64),
            neuron_minimum_stake_e8s: Some(20000u64),
            transaction_fee_e8s: Some(10000u64),
            max_proposals_to_keep_per_action: Some(10),
            initial_voting_period_seconds: Some(86401),
            wait_for_quiet_deadline_increase_seconds: Some(60 * 60),
            max_number_of_neurons: Some(1000u64),
            neuron_minimum_dissolve_delay_to_vote_seconds: Some(1u64),
            max_followees_per_function: Some(10),
            max_dissolve_delay_seconds: Some(10000000u64),
            max_neuron_age_for_age_bonus: Some(10000000),
            max_number_of_proposals_with_ballots: Some(100u64),
            neuron_claimer_permissions: Some(NeuronPermissionList {
                permissions: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            }),
            neuron_grantable_permissions: Some(NeuronPermissionList {
                permissions: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            }),
            max_number_of_principals_per_neuron: Some(10),
            voting_rewards_parameters: Some(VotingRewardsParameters {
                round_duration_seconds: Some(1000),
                reward_rate_transition_duration_seconds: Some(100),
                initial_reward_rate_basis_points: Some(5),
                final_reward_rate_basis_points: Some(5),
            }),
            max_dissolve_delay_bonus_percentage: Some(10u64),
            max_age_bonus_percentage: Some(10u64),
            maturity_modulation_disabled: Some(false),
        }),
        latest_reward_event: None,
        in_flight_commands: BTreeMap::new(),
        genesis_timestamp_seconds: 1u64,
        metrics: None,
        ledger_canister_id: Some(sns_ledger_canister_id.clone()),
        root_canister_id: Some(sns_root_canister_id.clone()),
        id_to_nervous_system_functions: BTreeMap::new(),
        mode: 1,
        swap_canister_id: Some(sns_swap_canister_id.clone()),
        sns_metadata: Some(SnsMetadata {
            logo: None,
            url: Some("https://simgov.simgov".to_string()),
            name: Some("Simulation Gov".to_string()),
            description: Some("Simulation Gov desc".to_string()),
        }),
        sns_initialization_parameters: "".to_string(),
        pending_version: None,
        is_finalizing_disburse_maturity: None,
        maturity_modulation: None,
    };
}
