use std::collections::HashMap;

use crate::cycles_manager_suite::setup::setup_cycles_manager::setup_cycle_manager_canister;
use candid::Principal;
use pocket_ic::{PocketIc, PocketIcBuilder};
use sns_governance_canister::types::Neuron;
use types::BuildVersion;
use types::Cycles;

use crate::cycles_manager_suite::setup::setup_root::setup_root_canister;
use crate::utils::random_principal;

use super::setup_burner::setup_burner_canister;

const T: Cycles = 1_000_000_000_000;

pub struct CyclesManagerEnv {
    pub controller: Principal,
    pub cycles_manager_id: Principal,
    pub burner_canister_id: Principal,
    pub sns_root_canister_id: Principal,
    pub pic: PocketIc,
}

impl CyclesManagerEnv {}

pub struct CyclesManagerTestEnvBuilder {
    controller: Principal,
}

impl CyclesManagerTestEnvBuilder {
    pub fn new() -> Self {
        Self {
            controller: random_principal(),
        }
    }

    /// is the controller of everything - no real need for this but nice to have if you want to be specific
    pub fn add_controller(mut self, principal: Principal) -> Self {
        self.controller = principal;
        self
    }

    pub fn build(self) -> CyclesManagerEnv {
        let mut neuron_data = HashMap::new();
        neuron_data.insert(1, Neuron::default());
        let mut pic = PocketIcBuilder::new()
            .with_sns_subnet()
            .with_application_subnet()
            .build();

        let burner_canister_id = setup_burner_canister(&mut pic, &self.controller);
        // println!("Burner canister: {}", burner_canister_id);

        let sns_root_canister_id = setup_root_canister(&mut pic, &self.controller);
        println!("SNS root canister: {}", sns_root_canister_id);
        pic.tick();
        // Args
        let cycles_manager_init_args = cycles_manager_canister::init::InitArgs {
            test_mode: true,
            authorized_principals: vec![self.controller],
            canisters: vec![burner_canister_id, sns_root_canister_id],
            sns_root_canister: Some(sns_root_canister_id),
            max_top_up_amount: 2000 * T,
            min_interval: 60,
            min_cycles_balance: 200 * T,
            wasm_version: BuildVersion::min(),
        };

        // Setup cycle manager canister
        let cycles_manager_id: Principal =
            setup_cycle_manager_canister(&mut pic, &self.controller, cycles_manager_init_args);

        CyclesManagerEnv {
            controller: self.controller,
            cycles_manager_id,
            burner_canister_id,
            sns_root_canister_id,
            pic,
        }
    }
}
