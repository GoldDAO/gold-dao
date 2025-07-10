use crate::sns_test_env::utils::neuron_id_from_number;
use candid::{Nat, Principal};
use sns_governance_canister::types::governance::Version;
use sns_governance_canister::types::{Governance, Neuron};
use sns_ledger_canister::types::Account;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Copy)]
pub enum SnsProject {
    Ogy,
    GoldDao,
    Wtn,
}

use std::fmt;
impl fmt::Display for SnsProject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl SnsProject {
    pub fn default_init_args(
        &self,
        controller: Principal,
        canister_ids: &CanisterIds,
        neuron_data: &HashMap<usize, Neuron>,
        initial_balances: Option<Vec<(Account, Nat)>>,
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
        let mut sns_init_args = match self {
            SnsProject::Ogy => SnsInitArgs::ogy(canister_ids, controller),
            SnsProject::GoldDao => SnsInitArgs::goldao(canister_ids, controller),
            SnsProject::Wtn => SnsInitArgs::wtn(canister_ids, controller),
        };
        sns_init_args.governance_args.neurons = neuron_data_with_neuron_keys;
        sns_init_args.ledger_args.archive_options.controller_id = controller;
        sns_init_args.ledger_args.initial_balances = initial_balances.unwrap_or_default();
        sns_init_args
    }
}

#[derive(Default)]
pub struct SnsInitArgsBuilder {
    project: Option<SnsProject>,
    canister_ids: Option<CanisterIds>,
    neuron_data: Option<HashMap<usize, Neuron>>,
    controller: Option<Principal>,
    initial_balances: Option<Vec<(Account, Nat)>>,
}

impl SnsInitArgsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn project(mut self, project: SnsProject) -> Self {
        self.project = Some(project);
        self
    }

    pub fn canister_ids(mut self, ids: CanisterIds) -> Self {
        self.canister_ids = Some(ids);
        self
    }

    pub fn neuron_data(mut self, data: HashMap<usize, Neuron>) -> Self {
        self.neuron_data = Some(data);
        self
    }

    pub fn controller(mut self, controller: Principal) -> Self {
        self.controller = Some(controller);
        self
    }

    pub fn initial_balances(mut self, balances: Vec<(Account, Nat)>) -> Self {
        self.initial_balances = Some(balances);
        self
    }

    pub fn build(self) -> SnsInitArgs {
        SnsInitArgs::new(
            self.project.expect("project required"),
            &self.canister_ids.expect("canister_ids required"),
            &self.neuron_data.expect("neuron_data required"),
            self.controller.expect("controller required"),
            self.initial_balances,
        )
    }
}

#[derive(Clone)]
pub struct SnsInitArgs {
    pub governance_args: Governance,
    pub ledger_args: sns_ledger_canister::types::InitArgs,
    pub root_args: sns_root_canister::types::SnsRootCanister,
    pub index_args: sns_index_canister::types::InitArg,
    pub swap_args: sns_swap_canister::types::Init,
}

impl SnsInitArgs {
    pub fn new(
        project: SnsProject,
        canister_ids: &CanisterIds,
        neuron_data: &HashMap<usize, Neuron>,
        controller: Principal,
        initial_balances: Option<Vec<(Account, Nat)>>,
    ) -> SnsInitArgs {
        project.default_init_args(controller, canister_ids, neuron_data, initial_balances)
    }

    pub fn ogy(canister_ids: &CanisterIds, controller: Principal) -> SnsInitArgs {
        let governance_args = Governance {
        deployed_version: Some(Version {
            root_wasm_hash: vec![],
            governance_wasm_hash: vec![],
            ledger_wasm_hash: vec![],
            swap_wasm_hash: vec![],
            archive_wasm_hash: vec![],
            index_wasm_hash: vec![],
        }),
            neurons: BTreeMap::new(),
            proposals: BTreeMap::new(),
            parameters: Some(NervousSystemParameters {
                default_followees: Some(DefaultFollowees {
                                followees: BTreeMap::new(),
                            }),
                reject_cost_e8s: Some(100_000_000_000),
                neuron_minimum_stake_e8s: Some(88_800_000_000),
                transaction_fee_e8s: Some(200_000),
                max_proposals_to_keep_per_action: Some(100),
                initial_voting_period_seconds: Some(345_600),
                wait_for_quiet_deadline_increase_seconds: Some(86_400),
                max_number_of_neurons: Some(200_000),
                neuron_minimum_dissolve_delay_to_vote_seconds: Some(31_560_192),
                max_followees_per_function: Some(15),
                max_dissolve_delay_seconds: Some(157_788_000),
                max_neuron_age_for_age_bonus: Some(31_557_600),
                max_number_of_proposals_with_ballots: Some(700),
                neuron_claimer_permissions: Some(NeuronPermissionList {
                    permissions: (0..=10).collect(),
                }),
                neuron_grantable_permissions: Some(NeuronPermissionList {
                    permissions: (0..=10).collect(),
                }),
                max_number_of_principals_per_neuron: Some(5),
                voting_rewards_parameters: Some(VotingRewardsParameters {
                    round_duration_seconds: Some(86_400),
                    reward_rate_transition_duration_seconds: Some(86_400),
                    initial_reward_rate_basis_points: Some(1),
                    final_reward_rate_basis_points: Some(1),
                }),
                max_dissolve_delay_bonus_percentage: Some(100),
                max_age_bonus_percentage: Some(0),
                maturity_modulation_disabled: Some(false),
            }),
            latest_reward_event: None,
            in_flight_commands: BTreeMap::new(),
            genesis_timestamp_seconds: 1713271942,
            metrics: None,
            ledger_canister_id: Some(canister_ids.ledger_id.clone()),
            root_canister_id: Some(canister_ids.root_id.clone()),
            id_to_nervous_system_functions: BTreeMap::new(),
            mode: 1,
            swap_canister_id: Some(canister_ids.swap_id.clone()),
            sns_metadata: Some(SnsMetadata {
                logo: None,
                url: Some("https://www.origyn.com/".to_string()),
                name: Some("ORIGYN".to_string()),
                description: Some("The ORIGYN Protocol has been made possible thanks to the major contribution of the ORIGYN Foundation and embodies its vision of becoming the universal digital standard for certification, traceability, and authentication.".to_string()),
            }),
            sns_initialization_parameters: "".to_string(),
            pending_version: None,
            is_finalizing_disburse_maturity: None,
            maturity_modulation: None,
        };

        let ledger_args = sns_ledger_canister::types::InitArgs {
            decimals: Some(8),
            token_symbol: "OGY".to_string(),
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
                max_transactions_per_response: None,
                trigger_threshold: 1_000,
                more_controller_ids: None,
                max_message_size_bytes: None,
                cycles_for_archive_creation: None,
                node_max_memory_size_bytes: None,
                controller_id: canister_ids.governance_id,
            },
            max_memo_length: Some(32),
            token_name: "ORIGYN".to_string(),
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

    pub fn goldao(canister_ids: &CanisterIds, controller: Principal) -> SnsInitArgs {
        let governance_args = Governance {
        deployed_version: Some(Version {
            root_wasm_hash: vec![],
            governance_wasm_hash: vec![],
            ledger_wasm_hash: vec![],
            swap_wasm_hash: vec![],
            archive_wasm_hash: vec![],
            index_wasm_hash: vec![],
        }),
        neurons: BTreeMap::new(),
        proposals: BTreeMap::new(),
        parameters: Some(NervousSystemParameters {
            default_followees: Some(DefaultFollowees {
                followees: BTreeMap::new(),
            }),
            reject_cost_e8s: Some(100_000_000_000_u64),
            neuron_minimum_stake_e8s: Some(10_000_000_000u64),
            transaction_fee_e8s: Some(100_000u64),
            max_proposals_to_keep_per_action: Some(100),

            // NOTE: those ones are mocked
              initial_voting_period_seconds: Some(2591000),
              wait_for_quiet_deadline_increase_seconds: Some(1295500),

            // initial_voting_period_seconds: Some(345_600),
            // wait_for_quiet_deadline_increase_seconds: Some(86_400),

            max_number_of_neurons: Some(200_000u64),
            neuron_minimum_dissolve_delay_to_vote_seconds: Some(7_890_048_u64),
            max_followees_per_function: Some(15),
            max_dissolve_delay_seconds: Some(63_115_200_u64),
            max_neuron_age_for_age_bonus: Some(63_115_200),
            max_number_of_proposals_with_ballots: Some(700_u64),
            neuron_claimer_permissions: Some(NeuronPermissionList {
                permissions: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            }),
            neuron_grantable_permissions: Some(NeuronPermissionList {
                permissions: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            }),
            max_number_of_principals_per_neuron: Some(5),
            voting_rewards_parameters: Some(VotingRewardsParameters {
                            round_duration_seconds: Some(86_400),
                            reward_rate_transition_duration_seconds: Some(0),
                            initial_reward_rate_basis_points: Some(10),
                            final_reward_rate_basis_points: Some(10),
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
            url: Some("https://www.gold-dao.org".to_string()),
            name: Some("Gold DAO".to_string()),
            description: Some("The Gold DAO project represents a groundbreaking fusion of traditional gold and modern blockchain technology, allowing anyone in the world to access  physical gold instantaneously, with no dependence upon the banking system.".to_string()),
        }),
        sns_initialization_parameters: "".to_string(),
        pending_version: None,
        is_finalizing_disburse_maturity: None,
        maturity_modulation: None,
    };

        let ledger_args = sns_ledger_canister::types::InitArgs {
            decimals: Some(8),
            token_symbol: "GOLDAO".to_string(),
            transfer_fee: Nat::from(100_000_u64),
            metadata: vec![],
            minting_account: Account {
                owner: canister_ids.governance_id.clone(),
                subaccount: None,
            },
            initial_balances: Vec::new(),
            fee_collector_account: None, // where the fees are going?
            archive_options: sns_ledger_canister::types::ArchiveOptions {
                num_blocks_to_archive: 1000,
                max_transactions_per_response: None,
                trigger_threshold: 1_000,
                more_controller_ids: None,
                max_message_size_bytes: None,
                cycles_for_archive_creation: None,
                node_max_memory_size_bytes: None,
                controller_id: canister_ids.governance_id,
            },
            max_memo_length: Some(32),
            token_name: "GOLDAO".to_string(),
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

    pub fn wtn(canister_ids: &CanisterIds, controller: Principal) -> SnsInitArgs {
        // NOTE: Old params
        let governance_args = Governance {
        deployed_version: Some(Version {
            root_wasm_hash: vec![],
            governance_wasm_hash: vec![],
            ledger_wasm_hash: vec![],
            swap_wasm_hash: vec![],
            archive_wasm_hash: vec![],
            index_wasm_hash: vec![],
        }),
            neurons: BTreeMap::default(),
            proposals: BTreeMap::new(),
            parameters: Some(NervousSystemParameters {
                default_followees: Some(DefaultFollowees {
                    followees: BTreeMap::new(),
                }),
                reject_cost_e8s: Some(10_000_000_000u64),
                neuron_minimum_stake_e8s: Some(10_000_000_000u64),
                transaction_fee_e8s: Some(1_000_000u64),
                max_proposals_to_keep_per_action: Some(100),
                initial_voting_period_seconds: Some(345_600),
                wait_for_quiet_deadline_increase_seconds: Some(86_400),
                max_number_of_neurons: Some(200_000u64),
                neuron_minimum_dissolve_delay_to_vote_seconds: Some(7_890_048u64),
                max_followees_per_function: Some(15),
                max_dissolve_delay_seconds: Some(94_672_800u64),
                max_neuron_age_for_age_bonus: Some(94_672_800),
                max_number_of_proposals_with_ballots: Some(700u64),
                neuron_claimer_permissions: Some(NeuronPermissionList {
                    permissions: (0..=10).collect(),
                }),
                neuron_grantable_permissions: Some(NeuronPermissionList {
                    permissions: (0..=10).collect(),
                }),
                max_number_of_principals_per_neuron: Some(5),
                    voting_rewards_parameters: Some(VotingRewardsParameters {
                    round_duration_seconds: Some(86_400),
                    reward_rate_transition_duration_seconds: Some(31_557_600),
                    initial_reward_rate_basis_points: Some(300),
                    final_reward_rate_basis_points: Some(300),
                }),
                max_dissolve_delay_bonus_percentage: Some(100u64),
                max_age_bonus_percentage: Some(100u64),
                maturity_modulation_disabled: Some(true), // NOTE: in real WTN - the modulation is used
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
            url: Some("https://waterneuron.fi/".to_string()),
            name: Some("WaterNeuron".to_string()),
            description: Some("WaterNeuron is a liquid staking protocol designed for the Internet Computer network.".to_string()),
        }),
            sns_initialization_parameters: "".to_string(),
            pending_version: None,
            is_finalizing_disburse_maturity: None,
            maturity_modulation: None,
        };

        let ledger_args = sns_ledger_canister::types::InitArgs {
            decimals: Some(8),
            token_symbol: "WTN".to_string(),
            transfer_fee: Nat::from(1_000_000_u64),
            metadata: vec![],
            minting_account: Account {
                owner: canister_ids.governance_id.clone(),
                subaccount: None,
            },
            initial_balances: Vec::new(),
            fee_collector_account: None,
            archive_options: sns_ledger_canister::types::ArchiveOptions {
                num_blocks_to_archive: 1000,
                max_transactions_per_response: None,
                trigger_threshold: 1_000,
                more_controller_ids: None,
                max_message_size_bytes: None,
                cycles_for_archive_creation: None,
                node_max_memory_size_bytes: None,
                controller_id: canister_ids.governance_id,
            },
            max_memo_length: Some(32),
            token_name: "WaterNeuron".to_string(),
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
