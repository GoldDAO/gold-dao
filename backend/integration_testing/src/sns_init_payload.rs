use std::collections::{ BTreeMap, HashMap };

use candid::Principal;
use sns_governance_canister::types::{
    governance::SnsMetadata,
    DefaultFollowees,
    DisburseMaturityInProgress,
    Governance,
    NervousSystemParameters,
    Neuron,
    NeuronId,
    NeuronPermission,
    NeuronPermissionList,
    VotingRewardsParameters,
};

pub fn create_weekly_sns_canister_data(
    week: u64 // initializes the sns with week n's data
) -> (Governance, Vec<Principal>, BTreeMap<String, Neuron>) {
    let sns_root_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    let sns_ledger_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 3]);
    let sns_swap_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 4]);

    let (neuron_data, users) = generate_neuron_data_for_week(week);

    return (
        Governance {
            deployed_version: None,
            neurons: neuron_data.clone(),
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
            genesis_timestamp_seconds: 1713164693u64,
            metrics: None,
            ledger_canister_id: Some(sns_ledger_canister_id.clone()),
            root_canister_id: Some(sns_root_canister_id.clone()),
            id_to_nervous_system_functions: BTreeMap::new(),
            mode: 2,
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
        },
        users,
        neuron_data,
    );
}

pub fn generate_neuron_data_for_week(week: u64) -> (BTreeMap<String, Neuron>, Vec<Principal>) {
    let mut neurons = BTreeMap::new();

    let user_1 = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    let user_2 = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

    neurons.insert(
        "146ed81314556807536d74005f4121b8769bba1992fce6b90c2949e855d04208".to_string(),
        Neuron {
            id: Some(
                NeuronId::new(
                    "146ed81314556807536d74005f4121b8769bba1992fce6b90c2949e855d04208"
                ).unwrap()
            ),
            permissions: vec![
                NeuronPermission {
                    principal: Some(Principal::anonymous()),
                    permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                },
                NeuronPermission {
                    principal: Some(user_1),
                    permission_type: vec![3, 4],
                }
            ],
            cached_neuron_stake_e8s: 20000u64,
            neuron_fees_e8s: 10000u64,
            created_timestamp_seconds: 1713164693,
            aging_since_timestamp_seconds: 1713164693,
            followees: BTreeMap::new(),
            maturity_e8s_equivalent: 100_000 * week,
            voting_power_percentage_multiplier: 1,
            source_nns_neuron_id: None,
            staked_maturity_e8s_equivalent: Some(123456),
            auto_stake_maturity: Some(false),
            vesting_period_seconds: Some(100000),
            disburse_maturity_in_progress: vec![],
            dissolve_state: Some(
                sns_governance_canister::types::neuron::DissolveState::WhenDissolvedTimestampSeconds(
                    100000000000
                )
            ),
        }
    );

    (neurons, vec![user_1, user_2])
}
