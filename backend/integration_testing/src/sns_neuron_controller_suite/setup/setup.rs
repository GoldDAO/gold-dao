use crate::sns_neuron_controller_suite::setup::setup_ledger::setup_ledgers;
use crate::sns_neuron_controller_suite::setup::setup_rewards::setup_rewards_canister;
use crate::sns_neuron_controller_suite::setup::setup_sns_neuron_controller::setup_sns_neuron_controller_canister;
use crate::sns_neuron_controller_suite::setup::*;
use crate::sns_test_env::sns_test_env::generate_neuron_data;
use crate::sns_test_env::sns_test_env::SnsTestEnv;
use crate::sns_test_env::sns_test_env::SnsTestEnvBuilder;
use crate::utils::random_principal;
use crate::utils::tick_n_blocks;
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::PocketIc;
use pocket_ic::PocketIcBuilder;
use sns_governance_canister::types::Neuron;
use sns_neuron_controller_api_canister::init::OgyManagerConfig;
use sns_neuron_controller_api_canister::init::WtnManagerConfig;
use sns_neuron_controller_api_canister::Args;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;
use types::BuildVersion;
use types::CanisterId;

#[derive(CandidType, Deserialize, Debug)]
pub struct RegisterDappCanisterRequest {
    pub canister_id: Option<Principal>,
}

pub struct SNCTestEnv {
    // NOTE: Pic is stored inside this struct
    pub pic: Rc<RefCell<PocketIc>>,
    pub ogy_sns_test_env: SnsTestEnv,
    pub wtn_sns_test_env: SnsTestEnv,
    pub controller: Principal,
    pub wtn_neuron_data: HashMap<usize, Neuron>,
    pub ogy_neuron_data: HashMap<usize, Neuron>,
    pub token_ledgers: HashMap<String, Principal>,
    pub sns_neuron_controller_id: CanisterId,
    pub ogy_rewards_canister_id: CanisterId,
    pub gld_rewards_canister_id: CanisterId, // could be mocked
}

impl SNCTestEnv {
    pub fn get_pic(&self) -> std::cell::Ref<PocketIc> {
        self.pic.borrow()
    }
}

pub struct SNCTestEnvBuilder {
    pub controller: Principal,
    token_symbols: Vec<String>,
    // Canister ids parameters
    sns_neuron_controller_id: CanisterId,
    ogy_rewards_canister_id: CanisterId,
    gld_rewards_canister_id: CanisterId, // could be mocked
    // Ledger parameters
    initial_ledger_accounts: Vec<(Account, Nat)>,
    ledger_fees: HashMap<String, Nat>,
    // Marker to check whether the neuron data needs to be pre-generated
    with_neuron_data: bool,
    with_other_user_neuron_data: bool,
}

impl Default for SNCTestEnvBuilder {
    fn default() -> Self {
        Self {
            controller: random_principal(),
            sns_neuron_controller_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ogy_rewards_canister_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 3]),
            gld_rewards_canister_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 4]),
            token_symbols: vec![],
            initial_ledger_accounts: vec![],
            ledger_fees: HashMap::new(),
            with_neuron_data: false,
            with_other_user_neuron_data: false,
        }
    }
}

impl SNCTestEnvBuilder {
    pub fn new() -> Self {
        SNCTestEnvBuilder::default()
    }

    pub fn with_controller(mut self, principal: Principal) -> Self {
        self.controller = principal;
        self
    }

    pub fn with_neuron_data(mut self) -> Self {
        self.with_neuron_data = true;
        self
    }

    pub fn with_other_user_neuron_data(mut self) -> Self {
        self.with_other_user_neuron_data = true;
        self
    }

    pub fn add_token_ledger(
        mut self,
        symbol: &str,
        initial_balances: &mut Vec<(Account, Nat)>,
        transaction_fee: Nat,
    ) -> Self {
        self.token_symbols.push(symbol.to_string());
        self.initial_ledger_accounts.append(initial_balances);
        self.ledger_fees.insert(symbol.to_string(), transaction_fee);
        self
    }

    pub fn build(&mut self) -> SNCTestEnv {
        let pic_ref = Rc::new(RefCell::new(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_sns_subnet()
                .with_application_subnet()
                .build(),
        ));
        let pic = pic_ref.borrow();
        let sns_subnet = pic.topology().get_sns().unwrap();

        // TODO: impl with method in sns_test_env
        self.ogy_rewards_canister_id =
            pic.create_canister_on_subnet(Some(self.controller.clone()), None, sns_subnet);
        self.sns_neuron_controller_id =
            pic.create_canister_on_subnet(Some(self.controller.clone()), None, sns_subnet);
        self.gld_rewards_canister_id =
            pic.create_canister_on_subnet(Some(self.controller.clone()), None, sns_subnet);

        let mut ogy_neuron_data = HashMap::new();
        if self.with_neuron_data == true {
            (ogy_neuron_data, _) =
                generate_neuron_data(0, 1, 1, &vec![self.sns_neuron_controller_id]);
        }

        let mut wtn_neuron_data = HashMap::new();
        if self.with_other_user_neuron_data == true {
            // (neuron_data, _) = generate_neuron_data(0, 20, 1, &vec![self.controller]);

            (wtn_neuron_data, _) =
                generate_neuron_data(0, 1, 1000000, &vec![self.sns_neuron_controller_id]);
            pic.advance_time(Duration::from_secs(100));
            tick_n_blocks(&pic, 50);
        }

        let mut ogy_sns_test_env_builder = SnsTestEnvBuilder::new(&pic_ref, self.controller);
        ogy_sns_test_env_builder.generate_ids();
        let ogy_sns_test_env = ogy_sns_test_env_builder
            .with_ogy_init_args(&ogy_neuron_data)
            .build();

        let mut wtn_sns_test_env_builder = SnsTestEnvBuilder::new(&pic_ref, self.controller);
        wtn_sns_test_env_builder.generate_ids();
        let wtn_sns_test_env = wtn_sns_test_env_builder
            .with_wtn_init_args(&wtn_neuron_data)
            .build();

        let ogy_sns_ledger_canister_id = ogy_sns_test_env.ledger_id;

        let mut token_ledgers = setup_ledgers(
            &pic,
            self.controller.clone(),
            self.token_symbols.clone(),
            self.initial_ledger_accounts.clone(),
            self.ledger_fees.clone(),
        );
        token_ledgers.insert(
            "ogy_ledger_canister_id".to_string(),
            ogy_sns_test_env.ledger_id,
        );
        token_ledgers.insert(
            "wtn_ledger_canister_id".to_string(),
            wtn_sns_test_env.ledger_id,
        );

        let ogy_sns_rewards_canister_id = setup_rewards_canister(
            &pic_ref.borrow(),
            self.ogy_rewards_canister_id,
            &token_ledgers,
            ogy_sns_test_env.governance_id,
            &self.controller,
        );

        let snc_init_args = Args::Init(sns_neuron_controller_api_canister::init::InitArgs {
            test_mode: false,
            version: BuildVersion::min(),
            commit_hash: "integration_testing".to_string(),
            authorized_principals: vec![
                self.controller,
                ogy_sns_test_env.governance_id,
                wtn_sns_test_env.governance_id,
            ],
            ogy_manager_config: OgyManagerConfig {
                ogy_sns_governance_canister_id: ogy_sns_test_env.governance_id,
                ogy_sns_ledger_canister_id,
                ogy_sns_rewards_canister_id,
                ogy_rewards_threshold: Nat::from(100_000_000_u64) * Nat::from(1_000_000_u64),
            },
            wtn_manager_config: WtnManagerConfig {
                wtn_sns_governance_canister_id: wtn_sns_test_env.governance_id,
                wtn_sns_ledger_canister_id: wtn_sns_test_env.ledger_id,
                icp_ledger: token_ledgers.get("icp_ledger_canister_id").unwrap().clone(),
                icp_rewards_threshold: Nat::from(10_000_u64),
                wtn_rewards_threshold: Nat::from(10_000_u64),
            },
            sns_rewards_canister_id: self.gld_rewards_canister_id,
        });

        let snc_canister_id = setup_sns_neuron_controller_canister(
            &pic_ref.borrow(),
            self.sns_neuron_controller_id,
            snc_init_args,
            vec![
                self.controller,
                ogy_sns_test_env.governance_id,
                wtn_sns_test_env.governance_id,
            ],
        );

        println!("snc_canister_id: {}", snc_canister_id);

        pic.advance_time(Duration::from_secs(100));
        tick_n_blocks(&pic, 50);

        SNCTestEnv {
            pic: Rc::clone(&pic_ref),
            controller: self.controller,
            token_ledgers,
            sns_neuron_controller_id: snc_canister_id,
            wtn_neuron_data,
            ogy_neuron_data,
            ogy_sns_test_env: ogy_sns_test_env,
            wtn_sns_test_env: wtn_sns_test_env,
            ogy_rewards_canister_id: ogy_sns_rewards_canister_id,
            gld_rewards_canister_id: self.gld_rewards_canister_id,
        }
    }
}
