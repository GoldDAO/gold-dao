use candid::{Nat, Principal};
use sha2::{Digest, Sha256};
use sns_governance_canister::types::{Governance, Neuron, NeuronId};
use sns_ledger_canister::types::Account;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone)]
pub struct SnsInitArgs {
    pub governance_args: Governance,
    pub ledger_args: sns_ledger_canister::types::InitArgs,
    pub root_args: sns_root_canister::types::SnsRootCanister,
    pub index_args: sns_index_canister::types::InitArg,
    pub swap_args: sns_swap_canister::types::Init,
}

impl SnsInitArgs {
    pub fn ogy(
        canister_ids: &CanisterIds,
        neuron_data: &HashMap<usize, Neuron>,
        controller: Principal,
    ) -> SnsInitArgs {
        let neuron_data_with_neuron_keys: BTreeMap<String, Neuron> = neuron_data
            .iter() // Iterate over the entries of the original map
            .map(|(key, value)| {
                (
                    neuron_id_from_number(key.clone()).to_string(),
                    value.clone(),
                )
            }) // Convert usize keys to String
            .collect();
        let governance_args = Governance {
            deployed_version: None,
            neurons: neuron_data_with_neuron_keys,
            proposals: BTreeMap::new(),
            parameters: Some(NervousSystemParameters {
                default_followees: Some(DefaultFollowees {
                    followees: BTreeMap::new(),
                }),
                reject_cost_e8s: Some(100_000_000_000_u64),
                neuron_minimum_stake_e8s: Some(10_000_000_000u64),
                transaction_fee_e8s: Some(100_000u64),
                max_proposals_to_keep_per_action: Some(100),
                initial_voting_period_seconds: Some(345_600),
                wait_for_quiet_deadline_increase_seconds: Some(86_400),
                max_number_of_neurons: Some(200_000u64),
                neuron_minimum_dissolve_delay_to_vote_seconds: Some(7_890_048_u64),
                max_followees_per_function: Some(15),
                max_dissolve_delay_seconds: Some(63_115_200_u64),
                max_neuron_age_for_age_bonus: Some(63_115_200),
                max_number_of_proposals_with_ballots: Some(700_u64),
                neuron_claimer_permissions: Some(NeuronPermissionList {
                    permissions: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                }),
                neuron_grantable_permissions: Some(NeuronPermissionList {
                    permissions: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                }),
                max_number_of_principals_per_neuron: Some(5),
                voting_rewards_parameters: Some(VotingRewardsParameters {
                    round_duration_seconds: Some(86_400),
                    reward_rate_transition_duration_seconds: Some(0),
                    initial_reward_rate_basis_points: Some(10),
                    final_reward_rate_basis_points: Some(10),
                }),
                max_dissolve_delay_bonus_percentage: Some(100_u64),
                max_age_bonus_percentage: Some(50_u64),
                maturity_modulation_disabled: Some(false),
            }),
            latest_reward_event: None,
            in_flight_commands: BTreeMap::new(),
            genesis_timestamp_seconds: 1713271942u64,
            metrics: None,
            ledger_canister_id: Some(canister_ids.ledger_id.clone()),
            root_canister_id: Some(canister_ids.root_id.clone()),
            id_to_nervous_system_functions: BTreeMap::new(),
            mode: 1,
            swap_canister_id: Some(canister_ids.swap_id.clone()),
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

        // FIXME: replace with better args
        let ledger_args = sns_ledger_canister::types::InitArgs {
            decimals: Some(8),
            token_symbol: "GOLD_TEST".to_string(),
            transfer_fee: Nat::from(200_000_u64),
            metadata: vec![],
            minting_account: Account {
                owner: canister_ids.governance_id.clone(),
                subaccount: None,
            },
            initial_balances: Vec::new(),
            fee_collector_account: None,
            archive_options: sns_ledger_canister::types::ArchiveOptions {
                num_blocks_to_archive: 1000,
                max_transactions_per_response: Some(1000),
                trigger_threshold: 2000,
                more_controller_ids: None,
                max_message_size_bytes: Some(2 * 1024 * 1024), // 2 MB
                cycles_for_archive_creation: Some(1_000_000_000_000),
                node_max_memory_size_bytes: Some(4 * 1024 * 1024 * 1024), // 4 GB
                controller_id: controller.clone(),
            },
            max_memo_length: None,
            token_name: "Test Token".to_string(),
            feature_flags: None,
        };

        let root_args = sns_root_canister::types::SnsRootCanister {
            dapp_canister_ids: vec![],
            timers: None,
            testflight: false,
            archive_canister_ids: vec![],
            governance_canister_id: Some(canister_ids.governance_id),
            index_canister_id: Some(canister_ids.index_id),
            swap_canister_id: Some(canister_ids.swap_id),
            ledger_canister_id: Some(canister_ids.ledger_id),
        };

        let index_args = sns_index_canister::types::InitArg {
            ledger_id: canister_ids.ledger_id,
            retrieve_blocks_from_ledger_interval_seconds: Some(60),
        };

        let swap_args = sns_swap_canister::types::Init {
            neuron_basket_construction_parameters: Some(
                sns_swap_canister::types::NeuronBasketConstructionParameters {
                    count: 3,
                    dissolve_delay_interval_seconds: 15_778_800, // 6 months in seconds
                },
            ),
            nns_proposal_id: Some(1),
            nns_governance_canister_id: controller.to_string(),
            sns_governance_canister_id: canister_ids.governance_id.to_text(),
            sns_ledger_canister_id: canister_ids.ledger_id.to_text(),
            icp_ledger_canister_id: canister_ids.ledger_id.to_text(),
            sns_root_canister_id: canister_ids.root_id.to_text(),
            fallback_controller_principal_ids: vec![controller.to_string()],
            transaction_fee_e8s: Some(10_000),
            neuron_minimum_stake_e8s: Some(1_000_000),
            confirmation_text: Some("Confirm your participation".to_string()),
            restricted_countries: None,
            min_participants: Some(1),
            min_icp_e8s: None,
            max_icp_e8s: None,
            min_direct_participation_icp_e8s: Some(1_000_000),
            max_direct_participation_icp_e8s: Some(10_000_000_000),
            min_participant_icp_e8s: Some(1_000_000),
            max_participant_icp_e8s: Some(10_000_000_000),
            swap_start_timestamp_seconds: Some(1_700_000_000),
            swap_due_timestamp_seconds: Some(1_700_086_400),
            sns_token_e8s: Some(1_000_000_000),
            // neurons_fund_participants: None,
            should_auto_finalize: Some(true),
            neurons_fund_participation_constraints: None,
            neurons_fund_participation: Some(false), // Set to false for testing
        };

        Self {
            governance_args,
            ledger_args,
            root_args,
            index_args,
            swap_args,
        }
    }

    pub fn goldao(
        canister_ids: &CanisterIds,
        neuron_data: &HashMap<usize, Neuron>,
        controller: Principal,
    ) -> SnsInitArgs {
        let governance_args = generate_sns_init_args(
            neuron_data,
            canister_ids.ledger_id,
            canister_ids.root_id,
            canister_ids.swap_id,
        );

        // FIXME: replace with better args
        let ledger_args = sns_ledger_canister::types::InitArgs {
            decimals: Some(8),
            token_symbol: "GOLD_TEST".to_string(),
            transfer_fee: Nat::from(200_000_u64),
            metadata: vec![],
            minting_account: Account {
                owner: canister_ids.governance_id.clone(),
                subaccount: None,
            },
            initial_balances: Vec::new(),
            fee_collector_account: None,
            archive_options: sns_ledger_canister::types::ArchiveOptions {
                num_blocks_to_archive: 1000,
                max_transactions_per_response: Some(1000),
                trigger_threshold: 2000,
                more_controller_ids: None,
                max_message_size_bytes: Some(2 * 1024 * 1024), // 2 MB
                cycles_for_archive_creation: Some(1_000_000_000_000),
                node_max_memory_size_bytes: Some(4 * 1024 * 1024 * 1024), // 4 GB
                controller_id: controller.clone(),
            },
            max_memo_length: None,
            token_name: "Test Token".to_string(),
            feature_flags: None,
        };

        let root_args = sns_root_canister::types::SnsRootCanister {
            dapp_canister_ids: vec![],
            timers: None,
            testflight: false,
            archive_canister_ids: vec![],
            governance_canister_id: Some(canister_ids.governance_id),
            index_canister_id: Some(canister_ids.index_id),
            swap_canister_id: Some(canister_ids.swap_id),
            ledger_canister_id: Some(canister_ids.ledger_id),
        };

        let index_args = sns_index_canister::types::InitArg {
            ledger_id: canister_ids.ledger_id,
            retrieve_blocks_from_ledger_interval_seconds: Some(60),
        };

        let swap_args = sns_swap_canister::types::Init {
            neuron_basket_construction_parameters: Some(
                sns_swap_canister::types::NeuronBasketConstructionParameters {
                    count: 3,
                    dissolve_delay_interval_seconds: 15_778_800, // 6 months in seconds
                },
            ),
            nns_proposal_id: Some(1),
            nns_governance_canister_id: controller.to_string(),
            sns_governance_canister_id: canister_ids.governance_id.to_text(),
            sns_ledger_canister_id: canister_ids.ledger_id.to_text(),
            icp_ledger_canister_id: canister_ids.ledger_id.to_text(),
            sns_root_canister_id: canister_ids.root_id.to_text(),
            fallback_controller_principal_ids: vec![controller.to_string()],
            transaction_fee_e8s: Some(10_000),
            neuron_minimum_stake_e8s: Some(1_000_000),
            confirmation_text: Some("Confirm your participation".to_string()),
            restricted_countries: None,
            min_participants: Some(1),
            min_icp_e8s: None,
            max_icp_e8s: None,
            min_direct_participation_icp_e8s: Some(1_000_000),
            max_direct_participation_icp_e8s: Some(10_000_000_000),
            min_participant_icp_e8s: Some(1_000_000),
            max_participant_icp_e8s: Some(10_000_000_000),
            swap_start_timestamp_seconds: Some(1_700_000_000),
            swap_due_timestamp_seconds: Some(1_700_086_400),
            sns_token_e8s: Some(1_000_000_000),
            // neurons_fund_participants: None,
            should_auto_finalize: Some(true),
            neurons_fund_participation_constraints: None,
            neurons_fund_participation: Some(false), // Set to false for testing
        };

        Self {
            governance_args,
            ledger_args,
            root_args,
            index_args,
            swap_args,
        }
    }

    pub fn wtn(
        canister_ids: &CanisterIds,
        neuron_data: &HashMap<usize, Neuron>,
        controller: Principal,
    ) -> SnsInitArgs {
        let neuron_data_with_neuron_keys: BTreeMap<String, Neuron> = neuron_data
            .iter() // Iterate over the entries of the original map
            .map(|(key, value)| {
                (
                    neuron_id_from_number(key.clone()).to_string(),
                    value.clone(),
                )
            }) // Convert usize keys to String
            .collect();
        let governance_args = Governance {
            deployed_version: None,
            neurons: neuron_data_with_neuron_keys,
            proposals: BTreeMap::new(),
            parameters: Some(NervousSystemParameters {
                default_followees: Some(DefaultFollowees {
                    followees: BTreeMap::new(),
                }),
                reject_cost_e8s: Some(10000u64),
                neuron_minimum_stake_e8s: Some(20000u64),
                transaction_fee_e8s: Some(10000u64),
                max_proposals_to_keep_per_action: Some(10),
                initial_voting_period_seconds: Some(2591000),
                wait_for_quiet_deadline_increase_seconds: Some(1295500),
                max_number_of_neurons: Some(1000u64),
                neuron_minimum_dissolve_delay_to_vote_seconds: Some(10u64),
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
                maturity_modulation_disabled: Some(true),
            }),
            latest_reward_event: None,
            in_flight_commands: BTreeMap::new(),
            genesis_timestamp_seconds: 1713271942u64,
            metrics: None,
            ledger_canister_id: Some(canister_ids.ledger_id.clone()),
            root_canister_id: Some(canister_ids.root_id.clone()),
            id_to_nervous_system_functions: BTreeMap::new(),
            mode: 1,
            swap_canister_id: Some(canister_ids.swap_id.clone()),
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

        // FIXME: replace with better args
        let ledger_args = sns_ledger_canister::types::InitArgs {
            decimals: Some(8),
            token_symbol: "GOLD_TEST".to_string(),
            transfer_fee: Nat::from(200_000_u64),
            metadata: vec![],
            minting_account: Account {
                owner: canister_ids.governance_id.clone(),
                subaccount: None,
            },
            initial_balances: Vec::new(),
            fee_collector_account: None,
            archive_options: sns_ledger_canister::types::ArchiveOptions {
                num_blocks_to_archive: 1000,
                max_transactions_per_response: Some(1000),
                trigger_threshold: 2000,
                more_controller_ids: None,
                max_message_size_bytes: Some(2 * 1024 * 1024), // 2 MB
                cycles_for_archive_creation: Some(1_000_000_000_000),
                node_max_memory_size_bytes: Some(4 * 1024 * 1024 * 1024), // 4 GB
                controller_id: controller.clone(),
            },
            max_memo_length: None,
            token_name: "Test Token".to_string(),
            feature_flags: None,
        };

        let root_args = sns_root_canister::types::SnsRootCanister {
            dapp_canister_ids: vec![],
            timers: None,
            testflight: false,
            archive_canister_ids: vec![],
            governance_canister_id: Some(canister_ids.governance_id),
            index_canister_id: Some(canister_ids.index_id),
            swap_canister_id: Some(canister_ids.swap_id),
            ledger_canister_id: Some(canister_ids.ledger_id),
        };

        let index_args = sns_index_canister::types::InitArg {
            ledger_id: canister_ids.ledger_id,
            retrieve_blocks_from_ledger_interval_seconds: Some(60),
        };

        let swap_args = sns_swap_canister::types::Init {
            neuron_basket_construction_parameters: Some(
                sns_swap_canister::types::NeuronBasketConstructionParameters {
                    count: 3,
                    dissolve_delay_interval_seconds: 15_778_800, // 6 months in seconds
                },
            ),
            nns_proposal_id: Some(1),
            nns_governance_canister_id: controller.to_string(),
            sns_governance_canister_id: canister_ids.governance_id.to_text(),
            sns_ledger_canister_id: canister_ids.ledger_id.to_text(),
            icp_ledger_canister_id: canister_ids.ledger_id.to_text(),
            sns_root_canister_id: canister_ids.root_id.to_text(),
            fallback_controller_principal_ids: vec![controller.to_string()],
            transaction_fee_e8s: Some(10_000),
            neuron_minimum_stake_e8s: Some(1_000_000),
            confirmation_text: Some("Confirm your participation".to_string()),
            restricted_countries: None,
            min_participants: Some(1),
            min_icp_e8s: None,
            max_icp_e8s: None,
            min_direct_participation_icp_e8s: Some(1_000_000),
            max_direct_participation_icp_e8s: Some(10_000_000_000),
            min_participant_icp_e8s: Some(1_000_000),
            max_participant_icp_e8s: Some(10_000_000_000),
            swap_start_timestamp_seconds: Some(1_700_000_000),
            swap_due_timestamp_seconds: Some(1_700_086_400),
            sns_token_e8s: Some(1_000_000_000),
            // neurons_fund_participants: None,
            should_auto_finalize: Some(true),
            neurons_fund_participation_constraints: None,
            neurons_fund_participation: Some(false), // Set to false for testing
        };

        Self {
            governance_args,
            ledger_args,
            root_args,
            index_args,
            swap_args,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CanisterIds {
    pub governance_id: Principal,
    pub ledger_id: Principal,
    pub root_id: Principal,
    pub index_id: Principal,
    pub swap_id: Principal,
    pub dapp_canisters: HashMap<String, Principal>,
}

use sns_governance_canister::types::{
    governance::SnsMetadata, DefaultFollowees, NervousSystemParameters, NeuronPermissionList,
    VotingRewardsParameters,
};

pub fn generate_sns_init_args(
    neuron_data: &HashMap<usize, Neuron>,
    sns_ledger_canister_id: Principal,
    sns_root_canister_id: Principal,
    sns_swap_canister_id: Principal,
) -> Governance {
    let neuron_data_with_neuron_keys: BTreeMap<String, Neuron> = neuron_data
        .iter() // Iterate over the entries of the original map
        .map(|(key, value)| {
            (
                neuron_id_from_number(key.clone()).to_string(),
                value.clone(),
            )
        }) // Convert usize keys to String
        .collect();

    Governance {
        deployed_version: None,
        neurons: neuron_data_with_neuron_keys,
        proposals: BTreeMap::new(),
        parameters: Some(NervousSystemParameters {
            default_followees: Some(DefaultFollowees {
                followees: BTreeMap::new(),
            }),
            reject_cost_e8s: Some(10000u64),
            neuron_minimum_stake_e8s: Some(20000u64),
            transaction_fee_e8s: Some(10000u64),
            max_proposals_to_keep_per_action: Some(10),
            initial_voting_period_seconds: Some(2591000),
            wait_for_quiet_deadline_increase_seconds: Some(1295500),
            max_number_of_neurons: Some(1000u64),
            neuron_minimum_dissolve_delay_to_vote_seconds: Some(10u64),
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
            maturity_modulation_disabled: Some(true),
        }),
        latest_reward_event: None,
        in_flight_commands: BTreeMap::new(),
        genesis_timestamp_seconds: 1713271942u64,
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
    }

    // NOTE: Real GOLDDAO parameters
    // Governance {
    //     deployed_version: None,
    //     neurons: neuron_data_with_neuron_keys,
    //     proposals: BTreeMap::new(),
    //     parameters: Some(NervousSystemParameters {
    //         default_followees: Some(DefaultFollowees {
    //             followees: BTreeMap::new(),
    //         }),
    //         reject_cost_e8s: Some(100_000_000_000_u64),
    //         neuron_minimum_stake_e8s: Some(10_000_000_000u64),
    //         transaction_fee_e8s: Some(100_000u64),
    //         max_proposals_to_keep_per_action: Some(100),
    //         initial_voting_period_seconds: Some(345_600),
    //         wait_for_quiet_deadline_increase_seconds: Some(86_400),
    //         max_number_of_neurons: Some(200_000u64),
    //         neuron_minimum_dissolve_delay_to_vote_seconds: Some(7_890_048_u64),
    //         max_followees_per_function: Some(15),
    //         max_dissolve_delay_seconds: Some(63_115_200_u64),
    //         max_neuron_age_for_age_bonus: Some(63_115_200),
    //         max_number_of_proposals_with_ballots: Some(700_u64),
    //         neuron_claimer_permissions: Some(NeuronPermissionList {
    //             permissions: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    //         }),
    //         neuron_grantable_permissions: Some(NeuronPermissionList {
    //             permissions: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    //         }),
    //         max_number_of_principals_per_neuron: Some(5),
    //         voting_rewards_parameters: Some(VotingRewardsParameters {
    //             round_duration_seconds: Some(86_400),
    //             reward_rate_transition_duration_seconds: Some(0),
    //             initial_reward_rate_basis_points: Some(10),
    //             final_reward_rate_basis_points: Some(10),
    //         }),
    //         max_dissolve_delay_bonus_percentage: Some(100_u64),
    //         max_age_bonus_percentage: Some(50_u64),
    //         maturity_modulation_disabled: Some(false),
    //     }),
    //     latest_reward_event: None,
    //     in_flight_commands: BTreeMap::new(),
    //     genesis_timestamp_seconds: 1713271942u64,
    //     metrics: None,
    //     ledger_canister_id: Some(sns_ledger_canister_id.clone()),
    //     root_canister_id: Some(sns_root_canister_id.clone()),
    //     id_to_nervous_system_functions: BTreeMap::new(),
    //     mode: 1,
    //     swap_canister_id: Some(sns_swap_canister_id.clone()),
    //     sns_metadata: Some(SnsMetadata {
    //         logo: None,
    //         url: Some("https://simgov.simgov".to_string()),
    //         name: Some("Simulation Gov".to_string()),
    //         description: Some("Simulation Gov desc".to_string()),
    //     }),
    //     sns_initialization_parameters: "".to_string(),
    //     pending_version: None,
    //     is_finalizing_disburse_maturity: None,
    //     maturity_modulation: None,
    // }
}

pub fn neuron_id_from_number(n: usize) -> NeuronId {
    // Hash the random number using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&n.to_be_bytes());
    let hash_result = hasher.finalize();

    // Convert the hash result to hexadecimal string
    let hex_id = hex::encode(hash_result);
    NeuronId::new(&hex_id).unwrap()
}

pub fn generate_neuron_data(
    start_at: usize,
    n: usize,
    maturity_multiplier: u64,
    users: &Vec<Principal>,
) -> (HashMap<usize, Neuron>, HashMap<Principal, usize>) {
    let mut neuron_data = HashMap::new();
    let mut owner_map = HashMap::new();
    let mut index_user = 0;
    for i in start_at..n {
        let neuron_id = neuron_id_from_number(i);
        let user_principal = users.get(index_user).clone();
        let perms = create_neuron_permissions(user_principal);
        let neuron = create_neuron(neuron_id, maturity_multiplier, perms);
        neuron_data.insert(i, neuron);
        if user_principal.is_some() {
            owner_map.insert(user_principal.unwrap().clone(), i);
        }
        if users.len() >= 1 && index_user == users.len() - 1 {
            index_user = 0;
        }
    }

    (neuron_data, owner_map)
}

pub fn create_neuron(
    id: NeuronId,
    maturity_multiplier: u64,
    perms: Vec<NeuronPermission>,
) -> Neuron {
    Neuron {
        id: Some(id),
        permissions: perms,
        cached_neuron_stake_e8s: 3000000000000u64,
        neuron_fees_e8s: 0u64,
        created_timestamp_seconds: 1620329630,
        aging_since_timestamp_seconds: 1620329630,
        followees: BTreeMap::new(),
        maturity_e8s_equivalent: 1 * maturity_multiplier,
        voting_power_percentage_multiplier: 1,
        source_nns_neuron_id: None,
        staked_maturity_e8s_equivalent: Some(10),
        auto_stake_maturity: Some(false),
        vesting_period_seconds: Some(100000),
        disburse_maturity_in_progress: vec![],
        dissolve_state: Some(
            sns_governance_canister::types::neuron::DissolveState::WhenDissolvedTimestampSeconds(
                100000000000,
            ),
        ),
    }
}

use sns_governance_canister::types::NeuronPermission;
pub fn create_neuron_permissions(user_hotkey: Option<&Principal>) -> Vec<NeuronPermission> {
    let mut perms = Vec::new();

    if let Some(hotkey) = user_hotkey {
        // Add the hotkey permissions
        perms.push(NeuronPermission {
            principal: Some(hotkey.clone()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });
    } else {
        // If no user_hotkey, add anonymous permissions
        perms.push(NeuronPermission {
            principal: Some(Principal::anonymous()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });
    }

    perms
}
