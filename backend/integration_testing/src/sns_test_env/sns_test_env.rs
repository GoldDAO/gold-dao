use crate::sns_test_env::sns_init_args::generate_sns_init_args;
use crate::sns_test_env::sns_init_args::CanisterIds;
use crate::sns_test_env::sns_init_args::SnsInitArgs;
use crate::utils::tick_n_blocks;
use crate::{client, wasms};
use candid::{encode_one, Principal};
use pocket_ic::PocketIc;
use sns_governance_canister::types::manage_neuron::Command;
use sns_governance_canister::types::proposal::Action;
use sns_governance_canister::types::Motion;
use sns_governance_canister::types::ProposalData;
use sns_governance_canister::types::{Neuron, NeuronId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub use crate::sns_test_env::sns_init_args::*;

pub struct SnsTestEnvBuilder {
    pub pic: Rc<RefCell<PocketIc>>,
    pub controller: Principal,

    pub canister_ids: Option<CanisterIds>,
    pub init_args: Option<SnsInitArgs>,

    pub dapp_canisters: HashMap<String, Principal>,
    pub developer_neuron_id: Option<String>,
}

impl Default for SnsTestEnvBuilder {
    fn default() -> Self {
        SnsTestEnvBuilder {
            pic: Rc::new(RefCell::new(PocketIc::default())),
            controller: Principal::anonymous(),
            canister_ids: None,
            init_args: None,
            dapp_canisters: HashMap::new(),
            developer_neuron_id: None,
        }
    }
}

impl SnsTestEnvBuilder {
    pub fn new(pic: &Rc<RefCell<PocketIc>>, controller: Principal) -> Self {
        SnsTestEnvBuilder {
            pic: pic.clone(),
            controller,
            ..Default::default()
        }
    }

    pub fn generate_ids(&mut self) -> &mut Self {
        let controller = self.controller;

        // Scope the borrow to avoid holding it across the entire method
        let (governance_id, root_id, ledger_id, index_id, swap_id) = {
            let pic = self.pic.borrow();
            let sns_subnet = pic.topology().get_sns().unwrap();

            (
                pic.create_canister_on_subnet(Some(controller), None, sns_subnet),
                pic.create_canister_on_subnet(Some(controller), None, sns_subnet),
                pic.create_canister_on_subnet(Some(controller), None, sns_subnet),
                pic.create_canister_on_subnet(Some(controller), None, sns_subnet),
                pic.create_canister_on_subnet(Some(controller), None, sns_subnet),
            )
        };

        // Add cycles to canisters
        {
            let pic: std::cell::Ref<'_, PocketIc> = self.pic.borrow();
            pic.add_cycles(governance_id, 1_000_000_000_000);
            pic.add_cycles(root_id, 1_000_000_000_000);
            pic.add_cycles(ledger_id, 1_000_000_000_000);
            pic.add_cycles(index_id, 1_000_000_000_000);
            pic.add_cycles(swap_id, 1_000_000_000_000);
        }

        self.canister_ids = Some(CanisterIds {
            governance_id,
            root_id,
            ledger_id,
            index_id,
            swap_id,
            dapp_canisters: HashMap::new(),
        });

        self
    }

    pub fn with_init_args(mut self, init_args: SnsInitArgs) -> Self {
        self.init_args = Some(init_args);
        self
    }

    pub fn with_ogy_init_args(mut self, neuron_data: &HashMap<usize, Neuron>) -> Self {
        let controller = self.controller;
        let canister_ids = self.canister_ids.clone().unwrap();
        let sns_init_args = SnsInitArgs::ogy(&canister_ids, neuron_data, controller);

        self.init_args = Some(sns_init_args);
        self
    }

    pub fn with_goldao_mock_init_args(mut self, neuron_data: &HashMap<usize, Neuron>) -> Self {
        let controller = self.controller;
        let canister_ids = self.canister_ids.clone().unwrap();
        // TODO: replace with default value
        let sns_init_args = SnsInitArgs::ogy(&canister_ids, neuron_data, controller);

        self.init_args = Some(sns_init_args);
        self
    }

    pub fn with_goldao_init_args(mut self, neuron_data: &HashMap<usize, Neuron>) -> Self {
        let controller = self.controller;
        let canister_ids = self.canister_ids.clone().unwrap();
        let sns_init_args = SnsInitArgs::goldao(&canister_ids, neuron_data, controller);

        self.init_args = Some(sns_init_args);
        self
    }

    pub fn with_wtn_init_args(mut self, neuron_data: &HashMap<usize, Neuron>) -> Self {
        let controller = self.controller;
        let canister_ids = self.canister_ids.clone().unwrap();
        let sns_init_args = SnsInitArgs::wtn(&canister_ids, neuron_data, controller);

        self.init_args = Some(sns_init_args);
        self
    }

    pub fn build(self) -> SnsTestEnv {
        let canister_ids = self.canister_ids.clone().unwrap();
        let init_args = self.init_args.clone().unwrap();
        let controller = self.controller;

        // Install the SNS canisters
        {
            let pic = self.pic.borrow();
            pic.install_canister(
                canister_ids.governance_id,
                wasms::SNS_GOVERNANCE.clone(),
                encode_one(init_args.governance_args).unwrap(),
                Some(controller.clone()),
            );
            pic.install_canister(
                canister_ids.root_id,
                wasms::SNS_ROOT.clone(),
                encode_one(init_args.root_args).unwrap(),
                Some(controller.clone()),
            );
            pic.install_canister(
                canister_ids.ledger_id,
                wasms::SNS_LEDGER.clone(),
                encode_one(sns_ledger_canister::types::LedgerArgument::Init(
                    init_args.ledger_args,
                ))
                .unwrap(),
                Some(controller.clone()),
            );
            pic.install_canister(
                canister_ids.index_id,
                wasms::SNS_INDEX.clone(),
                encode_one(sns_index_canister::types::IndexArg::Init(
                    init_args.index_args,
                ))
                .unwrap(),
                Some(controller.clone()),
            );
            pic.install_canister(
                canister_ids.swap_id,
                wasms::SNS_SWAP.clone(),
                encode_one(init_args.swap_args).unwrap(),
                Some(controller.clone()),
            );
        }

        // Clone the Rc<RefCell<PocketIc>> to avoid moving it
        let pic = Rc::clone(&self.pic);
        // Create the SnsTestEnv instance
        SnsTestEnv {
            pic,
            controller: self.controller,
            governance_id: canister_ids.governance_id,
            root_id: canister_ids.root_id,
            ledger_id: canister_ids.ledger_id,
            index_id: canister_ids.index_id,
            swap_id: canister_ids.swap_id,
            dapp_canisters: HashMap::new(),
        }
    }
}

pub struct SnsTestEnv {
    pub pic: Rc<RefCell<PocketIc>>,
    pub controller: Principal,
    pub governance_id: Principal,
    pub root_id: Principal,
    pub ledger_id: Principal,
    pub index_id: Principal,
    pub swap_id: Principal,
    pub dapp_canisters: HashMap<String, Principal>,
}

impl SnsTestEnv {
    pub fn add_dapp_canisters(&mut self, canisters: HashMap<String, Principal>) {
        let mut canister_ids = Vec::new();
        let pic = self.pic.borrow();

        // Insert each canister into the dapp_canisters HashMap
        for (name, canister_id) in canisters {
            self.dapp_canisters.insert(name, canister_id.clone());
            canister_ids.push(canister_id); // Add canister principal to the vector

            pic.set_controllers(
                canister_id,
                Some(self.controller),
                vec![self.controller, self.root_id],
            )
            .unwrap();
        }

        // Register all canisters at once
        crate::client::sns_root_canister::register_dapp_canisters(
            &pic,
            self.controller,
            self.root_id,
            &sns_root_canister::register_dapp_canisters::Args { canister_ids },
        );

        tick_n_blocks(&pic, 10);
    }

    pub fn create_and_register_dapp(
        &mut self,
        name: &str,
        wasm: Vec<u8>,
        init_payload: Vec<u8>,
    ) -> Principal {
        // Get the application subnet
        let dapp_id = {
            let pic = self.pic.borrow();
            let app_subnet = pic.topology().get_app_subnets();

            let dapp_id = pic.create_canister_on_subnet(
                Some(self.controller),
                None,
                *app_subnet.first().unwrap(),
            );
            pic.add_cycles(dapp_id, 100_000_000_000_000_000);

            pic.install_canister(dapp_id, wasm, init_payload, Some(self.controller));

            dapp_id
        };

        self.add_dapp_canisters(HashMap::from([(name.to_string(), dapp_id)]));

        // Log the message that the DApp has been registered
        println!(
            "DApp '{}' with canister ID {:?} has been registered.",
            name, dapp_id
        );

        dapp_id
    }

    pub fn submit_mock_proposal(
        &self,
        neuron_owner: Principal,
        neuron_id: &NeuronId,
    ) -> sns_governance_canister::types::ProposalId {
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
        pic.advance_time(std::time::Duration::from_secs(10));
        tick_n_blocks(&pic, 50);

        match result.command {
            Some(
                sns_governance_canister::types::manage_neuron_response::Command::MakeProposal(
                    proposal,
                ),
            ) => proposal.proposal_id.unwrap(),
            _ => panic!("Failed to submit proposal"),
        }
    }

    pub fn submit_proposal(
        &self,
        neuron_owner: Principal,
        neuron_id: &NeuronId,
        proposal: sns_governance_canister::types::Proposal,
    ) {
        let pic = self.pic.borrow();

        println!(
            "Submitting proposal with neuron {}: {:?}",
            neuron_id, proposal
        );

        let manage_neuron = sns_governance_canister::types::ManageNeuron {
            subaccount: neuron_id.id.clone(),
            command: Some(Command::MakeProposal(proposal)),
        };

        let _ = client::sns_governance::manage_neuron(
            &pic,
            neuron_owner,
            self.governance_id,
            &manage_neuron,
        );

        pic.advance_time(std::time::Duration::from_secs(100));
        tick_n_blocks(&pic, 50);
    }

    pub fn get_proposals(&self) -> Vec<ProposalData> {
        client::sns_governance::list_proposals(
            &self.pic.borrow(),
            self.governance_id,
            self.governance_id,
            &sns_governance_canister::types::ListProposals {
                limit: 100,
                before_proposal: None,
                exclude_type: vec![],
                include_reward_status: vec![],
                include_status: vec![],
            },
        )
        .proposals
    }

    pub fn get_proposal(
        &self,
        proposal_id: sns_governance_canister::types::ProposalId,
    ) -> sns_governance_canister::types::GetProposalResponse {
        client::sns_governance::get_proposal(
            &self.pic.borrow(),
            self.governance_id,
            self.governance_id,
            &sns_governance_canister::types::GetProposal {
                proposal_id: Some(proposal_id),
            },
        )
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
