use super::setup_burner::setup_burner_canister;
use super::setup_cycles_minting::setup_cycles_minting;
use crate::cycles_manager_suite::setup::setup_cycles_manager::setup_cycle_manager_canister;
use crate::cycles_manager_suite::setup::setup_icp_ledger::setup_icp_ledger;
use crate::cycles_manager_suite::setup::setup_sns_root::setup_root_canister;
use crate::utils::random_principal;
use candid::encode_one;
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;
use ic_ledger_types::AccountIdentifier;
use ic_ledger_types::Subaccount;
use ic_ledger_types::Tokens;
use pocket_ic::{PocketIc, PocketIcBuilder};
use std::collections::HashMap;
use std::collections::HashSet;
use types::CanisterId;
use types::Cycles;

pub const DEFAULT_SUBACCOUNT: Subaccount = Subaccount([0; 32]);

#[derive(CandidType, Deserialize, Debug)]
pub struct RegisterDappCanisterRequest {
    pub canister_id: Option<Principal>,
}

pub struct CyclesManagerEnv {
    pub controller: Principal,
    pub cycles_manager_id: Principal,
    pub burner_canister_id: Principal,
    pub sns_root_canister_id: Principal,
    pub icp_ledger_canister_id: CanisterId,
    pub cycles_minting_canister_id: CanisterId,
    pub pic: PocketIc,
}

impl CyclesManagerEnv {}

pub struct CyclesManagerTestEnvBuilder {
    controller: Principal,
    pub max_top_up_amount: Cycles,
    pub min_cycles_balance: Cycles,
    pub icp_burn_amount: Tokens,
}

impl Default for CyclesManagerTestEnvBuilder {
    fn default() -> Self {
        Self {
            controller: random_principal(),
            // max_top_up_amount: 20_000_000_000_000,
            max_top_up_amount: 200_000_000_000_000,
            min_cycles_balance: 200_000_000_000_000,
            // icp_burn_amount: Tokens::from_e8s(10_000_000_000),
            icp_burn_amount: Tokens::from_e8s(0),
        }
    }
}

impl CyclesManagerTestEnvBuilder {
    pub fn new() -> Self {
        CyclesManagerTestEnvBuilder::default()
    }
    pub fn with_controller(mut self, principal: Principal) -> Self {
        self.controller = principal;
        self
    }
    pub fn with_max_top_up_amount(mut self, max_top_up_amount: Cycles) -> Self {
        self.max_top_up_amount = max_top_up_amount;
        self
    }
    pub fn with_min_cycles_balance(mut self, min_cycles_balance: Cycles) -> Self {
        self.min_cycles_balance = min_cycles_balance;
        self
    }
    pub fn with_icp_burn_amount(mut self, icp_burn_amount: u64) -> Self {
        self.icp_burn_amount = Tokens::from_e8s(icp_burn_amount);
        self
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

        // Arguments to register dapp in the sns_root_canister
        let register_canister_args = RegisterDappCanisterRequest {
            canister_id: Some(burner_canister_id),
        };

        let _ = pic
            .update_call(
                sns_root_canister_id,
                self.controller,
                "register_dapp_canister",
                encode_one(register_canister_args).unwrap(),
            )
            .unwrap();

        let minting_account = AccountIdentifier::new(&self.controller, &DEFAULT_SUBACCOUNT);
        let icp_ledger_init_args = crate::cycles_manager_suite::setup::setup_icp_ledger::Args {
            minting_account: minting_account.to_string(),
            initial_values: HashMap::new(),
            send_whitelist: HashSet::new(),
            transfer_fee: Some(Tokens::from_e8s(10_000)),
        };
        let icp_ledger_canister_id =
            setup_icp_ledger(&mut pic, &self.controller, icp_ledger_init_args);

        let cycles_minting_init_args =
            crate::cycles_manager_suite::setup::setup_cycles_minting::Args {
                ledger_canister_id: icp_ledger_canister_id,
                governance_canister_id: CanisterId::anonymous(),
                minting_account_id: Some(minting_account.to_string()),
                last_purged_notification: Some(0),
            };

        let cycles_minting_canister_id =
            setup_cycles_minting(&mut pic, &self.controller, cycles_minting_init_args);
        pic.tick();

        // Define initialization arguments for cycles manager canister
        let cycles_manager_init_args = cycles_manager_api_canister::init::InitArgs {
            test_mode: true,
            authorized_principals: vec![self.controller],
            canisters: vec![burner_canister_id, sns_root_canister_id],
            sns_root_canister: sns_root_canister_id,
            max_top_up_amount: self.max_top_up_amount,
            min_cycles_balance: self.min_cycles_balance,
            icp_burn_amount: self.icp_burn_amount,
            icp_ledger_canister: icp_ledger_canister_id,
            cycles_minting_canister: cycles_minting_canister_id,
        };

        let cycles_manager_id: Principal =
            setup_cycle_manager_canister(&mut pic, &self.controller, cycles_manager_init_args);

        CyclesManagerEnv {
            controller: self.controller,
            cycles_manager_id,
            burner_canister_id,
            sns_root_canister_id,
            icp_ledger_canister_id,
            cycles_minting_canister_id,
            pic,
        }
    }
}
