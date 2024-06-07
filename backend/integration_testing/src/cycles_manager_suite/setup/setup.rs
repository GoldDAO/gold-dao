use super::setup_burner::setup_burner_canister;
use crate::cycles_manager_suite::setup::setup_cycles_manager::setup_cycle_manager_canister;
use crate::cycles_manager_suite::setup::setup_sns_root::setup_root_canister;
use crate::utils::random_principal;
use candid::Principal;
use pocket_ic::{PocketIc, PocketIcBuilder};
use types::Cycles;

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
    pub fn _new_with_controller(principal: Principal) -> Self {
        Self {
            controller: principal,
        }
    }

    pub fn build(self) -> CyclesManagerEnv {
        let mut pic = PocketIcBuilder::new()
            .with_sns_subnet()
            .with_application_subnet()
            .build();

        // Define initialization arguments for burner canister
        let burner_canister_init_args =
            crate::cycles_manager_suite::setup::setup_burner::InitArgs {
                interval_between_timers_in_seconds: 2 * 60 * 60, // Burn once in 2 hours
                burn_amount: 500_000_000_000,
            };

        let burner_canister_id =
            setup_burner_canister(&mut pic, &self.controller, burner_canister_init_args);
        pic.tick();

        // Define initialization arguments for root canister
        let root_init_args = crate::cycles_manager_suite::setup::setup_sns_root::Args {
            dapp_canister_ids: vec![],
            testflight: true,
            latest_ledger_archive_poll_timestamp_seconds: None,
            archive_canister_ids: vec![],
            governance_canister_id: Some(self.controller),
            index_canister_id: Some(Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 2])),
            swap_canister_id: Some(Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 3])),
            ledger_canister_id: Some(Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 4])),
        };

        let sns_root_canister_id = setup_root_canister(&mut pic, &self.controller, root_init_args);
        pic.tick();

        // Define initialization arguments for cycles manager canister
        let cycles_manager_init_args = cycles_manager_api_canister::init::InitArgs {
            test_mode: true,
            authorized_principals: vec![self.controller],
            canisters: vec![burner_canister_id, sns_root_canister_id],
            sns_root_canister: sns_root_canister_id,
            max_top_up_amount: 20 * T,
            min_cycles_balance: 10 * T,
        };

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
