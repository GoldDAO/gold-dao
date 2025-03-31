use crate::utils::tick_n_blocks;
use crate::{client, wasms};
use candid::Nat;
use candid::{encode_one, Principal};
use pocket_ic::PocketIc;
use sha2::{Digest, Sha256};
use sns_governance_canister::types::manage_neuron::Command;
use sns_governance_canister::types::proposal::Action;
use sns_governance_canister::types::Motion;
use sns_governance_canister::types::{
    governance::SnsMetadata, DefaultFollowees, Governance, NervousSystemParameters, Neuron,
    NeuronId, NeuronPermissionList, VotingRewardsParameters,
};
use sns_ledger_canister::types::Account;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

pub struct SnsTestEnv {
    pub pic: Rc<RefCell<PocketIc>>,
    pub controller: Principal,
    pub governance_id: Principal,
    pub ledger_id: Principal,
    pub root_id: Principal,
    pub index_id: Principal,
    pub swap_id: Principal,
    pub dapp_canisters: HashMap<String, Principal>,
    pub developer_neuron_id: Option<String>,
}

use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
impl Debug for SnsTestEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SnsTestEnv")
            .field("controller", &self.controller.to_text())
            .field("governance_id", &self.governance_id.to_text())
            .field("ledger_id", &self.ledger_id.to_text())
            .field("root_id", &self.root_id.to_text())
            .field("index_id", &self.index_id.to_text())
            .field("swap_id", &self.swap_id.to_text())
            .field("dapp_canisters", &self.dapp_canisters)
            .finish()
    }
}

impl SnsTestEnv {
    // Creates a new SNS test environment and deploys SNS canisters
    pub fn new(
        pic_pointer: &Rc<RefCell<PocketIc>>, // Take Rc<RefCell<PocketIc>> as parameter
        controller: Principal,
        neuron_data: &HashMap<usize, Neuron>,
    ) -> Self {
        let pic = pic_pointer.borrow();
        let sns_subnet = pic.topology().get_sns().unwrap();

        // Create canisters
        let sns_gov = pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet);
        let sns_root = pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet);
        let sns_ledger = pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet);
        let sns_index = pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet);
        let sns_swap = pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet);

        // Add cycles to canisters
        pic.add_cycles(sns_gov, 1_000_000_000_000);
        pic.add_cycles(sns_root, 1_000_000_000_000);
        pic.add_cycles(sns_ledger, 1_000_000_000_000);
        pic.add_cycles(sns_index, 1_000_000_000_000);
        pic.add_cycles(sns_swap, 1_000_000_000_000);

        // println!("neuron_data: {:?}", neuron_data);

        let sns_gov_init_args = generate_sns_init_args(neuron_data, sns_ledger, sns_root, sns_swap);

        let sns_root_init_args = sns_root_canister::types::SnsRootCanister {
            dapp_canister_ids: vec![],
            timers: None,
            testflight: false,
            archive_canister_ids: vec![],
            governance_canister_id: Some(sns_gov),
            index_canister_id: Some(sns_index),
            swap_canister_id: Some(sns_swap),
            ledger_canister_id: Some(sns_ledger),
        };

        // FIXME: replace with better args
        let sns_ledger_init_args = sns_ledger_canister::types::LedgerArgument::Init(
            sns_ledger_canister::types::InitArgs {
                decimals: Some(8),
                token_symbol: "GOLD_TEST".to_string(),
                transfer_fee: Nat::from(200_000_u64),
                metadata: vec![],
                minting_account: Account {
                    owner: sns_gov.clone(),
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
            },
        );

        let sns_index_init_args =
            sns_index_canister::types::IndexArg::Init(sns_index_canister::types::InitArg {
                ledger_id: sns_ledger,
                retrieve_blocks_from_ledger_interval_seconds: Some(60),
            });

        let sns_swap_init_args = sns_swap_canister::types::Init {
            neuron_basket_construction_parameters: Some(
                sns_swap_canister::types::NeuronBasketConstructionParameters {
                    count: 3,
                    dissolve_delay_interval_seconds: 15_778_800, // 6 months in seconds
                },
            ),
            nns_proposal_id: Some(1),
            nns_governance_canister_id: controller.to_string(),
            sns_governance_canister_id: sns_gov.to_text(),
            sns_ledger_canister_id: sns_ledger.to_text(),
            icp_ledger_canister_id: sns_ledger.to_text(),
            sns_root_canister_id: sns_root.to_text(),
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

        pic.install_canister(
            sns_gov,
            wasms::SNS_GOVERNANCE.clone(),
            encode_one(sns_gov_init_args).unwrap(),
            Some(controller.clone()),
        );
        pic.install_canister(
            sns_root,
            wasms::SNS_ROOT.clone(),
            encode_one(sns_root_init_args).unwrap(),
            Some(controller.clone()),
        );
        pic.install_canister(
            sns_ledger,
            wasms::SNS_LEDGER.clone(),
            encode_one(sns_ledger_init_args).unwrap(),
            Some(controller.clone()),
        );
        pic.install_canister(
            sns_index,
            wasms::SNS_INDEX.clone(),
            encode_one(sns_index_init_args).unwrap(),
            Some(controller.clone()),
        );
        pic.install_canister(
            sns_swap,
            wasms::SNS_SWAP.clone(),
            encode_one(sns_swap_init_args).unwrap(),
            Some(controller.clone()),
        );

        SnsTestEnv {
            pic: pic_pointer.clone(),
            controller,
            governance_id: sns_gov,
            ledger_id: sns_ledger,
            root_id: sns_root,
            index_id: sns_index,
            swap_id: sns_swap,
            dapp_canisters: HashMap::new(),
            developer_neuron_id: None,
        }
    }

    pub fn add_dapp_canister(&mut self, name: &str, canister_id: Principal) {
        self.dapp_canisters.insert(name.to_string(), canister_id);
    }

    // pub fn create_and_register_dapp(
    //     &mut self,
    //     name: &str,
    //     wasm: Vec<u8>,
    //     init_payload: Vec<u8>,
    // ) -> Principal {
    //     // Get the application subnet
    //     let app_subnet = *self.pic.topology().get_app_subnets().first().unwrap();

    //     // Create the dapp canister
    //     let dapp_id = self
    //         .pic
    //         .create_canister_on_subnet(Some(self.controller), None, app_subnet);
    //     self.pic.add_cycles(dapp_id, 100_000_000_000_000_000);

    //     // Install the dapp canister
    //     self.pic
    //         .install_canister(dapp_id, wasm, init_payload, Some(self.controller));

    //     // Add SNS root as a controller of the dapp
    //     self.pic
    //         .set_controllers(
    //             dapp_id,
    //             Some(self.controller),
    //             vec![self.controller, self.root_id, dapp_id],
    //         )
    //         .unwrap();

    //     // Register the dapp with the SNS root
    //     if let Some(neuron_id) = &self.developer_neuron_id {
    //         // Create and submit a proposal to register the dapp
    //         let proposal = format!(
    //             r#"(
    //                 record {{
    //                     title = "Register dapp canister";
    //                     url = "https://example.com/";
    //                     summary = "This proposal registers a dapp canister with the SNS.";
    //                     action = opt variant {{
    //                         RegisterDappCanisters = record {{
    //                             canister_ids = vec {{ principal "{}" }}
    //                         }}
    //                     }};
    //                 }}
    //             )"#,
    //             dapp_id.to_text()
    //         );

    //         // Submit the proposal using the developer neuron
    //         self.submit_proposal_raw(neuron_id, &proposal);
    //     }

    //     // Add the dapp to our list
    //     self.add_dapp_canister(name, dapp_id);

    //     dapp_id
    // }

    // pub fn submit_proposal_raw(&self, neuron_id: &str, proposal: &str) {
    //     // Implementation would call the make_proposal method on the governance canister
    //     // This is a simplified version - in a real implementation, you would encode the proposal
    //     // and call the governance canister
    //     println!(
    //         "Submitting proposal with neuron {}: {}",
    //         neuron_id, proposal
    //     );

    //     // Advance time to simulate proposal processing
    //     self.pic.advance_time(std::time::Duration::from_secs(60));
    //     self.pic.tick();
    // }

    pub fn submit_proposal(&self, neuron_owner: Principal, neuron_id: &NeuronId) {
        let pic = self.pic.borrow();

        let proposal = sns_governance_canister::types::Proposal {
            title: "Test Proposal".to_string(),
            summary: "This is a test proposal".to_string(),
            url: "https://example.com".to_string(),
            action: Some(Action::Motion(Motion {
                motion_text: "Test Motion".to_string(),
            })),
        };

        println!(
            "Submitting proposal with neuron {}: {:?}",
            neuron_id, proposal
        );

        let manage_neuron = sns_governance_canister::types::ManageNeuron {
            subaccount: neuron_id.id.clone(),
            command: Some(Command::MakeProposal(proposal)),
        };

        let result = client::sns_governance::manage_neuron(
            &pic,
            neuron_owner,
            self.governance_id,
            &manage_neuron,
        );

        println!("Proposal result: {result:?}");

        // Advance time to simulate proposal processing
        pic.advance_time(std::time::Duration::from_secs(100));
        tick_n_blocks(&pic, 50);
    }

    pub fn vote_on_proposal(
        &self,
        neuron_owner: Principal,
        neuron_id: &NeuronId,
        proposal_id: u64,
        vote: bool,
    ) {
        let pic = self.pic.borrow();
        // Create the proposal ID struct
        let proposal_id_struct = sns_governance_canister::types::ProposalId { id: proposal_id };

        // Convert the boolean vote to the expected integer format
        // According to the IC documentation, 1 = yes (adopt), 2 = no (reject)
        let vote_value: i32 = if vote { 1 } else { 2 };

        println!(
            "Voting {} on proposal {} with neuron {}",
            if vote { "yes" } else { "no" },
            proposal_id,
            &neuron_id
        );

        // Call the register_vote method on the governance canister
        let manage_neuron = sns_governance_canister::types::ManageNeuron {
            subaccount: neuron_id.id.clone(),
            command: Some(
                sns_governance_canister::types::manage_neuron::Command::RegisterVote(
                    sns_governance_canister::types::manage_neuron::RegisterVote {
                        proposal: Some(proposal_id_struct),
                        vote: vote_value,
                    },
                ),
            ),
        };

        let result = client::sns_governance::manage_neuron(
            &pic,
            neuron_owner,
            self.governance_id,
            &manage_neuron,
        );

        println!("Vote result: {result:?}");

        // Advance time to simulate vote processing
        // This is important because SNS voting may use the "wait for quiet" mechanism
        // which extends voting periods for controversial proposals
        pic.advance_time(std::time::Duration::from_secs(100));
        tick_n_blocks(&pic, 50);
    }

    /// Simulates a decentralization swap
    pub fn simulate_decentralization_swap(
        &mut self,
        participants: &[Principal],
        min_participants: u64,
    ) {
        let pic = self.pic.borrow();
        println!(
            "Starting decentralization swap simulation with {} participants",
            participants.len()
        );

        // Initialize the swap canister with parameters
        // This would be a call to the swap canister in a real implementation
        println!(
            "Initializing swap with min_participants={}",
            min_participants
        );

        // Simulate participants joining the swap
        for (i, participant) in participants.iter().enumerate() {
            println!("Participant {} joining swap", i + 1);
            // This would be a call to the swap canister in a real implementation
        }

        // Advance time to end of swap
        pic.advance_time(std::time::Duration::from_secs(3601));
        pic.tick();

        // Finalize the swap
        println!("Finalizing swap");
        // This would be a call to the swap canister in a real implementation

        // Advance time again to complete finalization
        pic.advance_time(std::time::Duration::from_secs(60));
        pic.tick();
    }

    pub fn reinstall_sns_with_data(&self, neuron_data: &HashMap<usize, Neuron>) {
        let pic = self.pic.borrow();
        let sns_init_args =
            generate_sns_init_args(neuron_data, self.ledger_id, self.root_id, self.swap_id);

        let sns_gov_wasm = wasms::SNS_GOVERNANCE.clone();
        pic.stop_canister(self.governance_id.clone(), Some(self.controller.clone()))
            .unwrap();
        pic.tick();
        pic.reinstall_canister(
            self.governance_id.clone(),
            sns_gov_wasm,
            encode_one(sns_init_args.clone()).unwrap(),
            Some(self.controller.clone()),
        )
        .unwrap();
        pic.tick();
        pic.start_canister(self.governance_id.clone(), Some(self.controller.clone()))
            .unwrap();

        pic.tick();
    }
}

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

    // println!("neuron_data_with_neuron_keys: {neuron_data_with_neuron_keys:?}");

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
