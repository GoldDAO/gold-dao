use crate::sns_neuron_controller_suite::setup::setup_ledger::setup_ledgers;
use crate::sns_neuron_controller_suite::setup::setup_rewards::setup_rewards_canister;
use crate::sns_neuron_controller_suite::setup::setup_sns::create_sns_with_data;
use crate::sns_neuron_controller_suite::setup::setup_sns::generate_neuron_data;
use crate::sns_neuron_controller_suite::setup::setup_sns_neuron_controller::setup_sns_neuron_controller_canister;
use crate::sns_neuron_controller_suite::setup::*;
use crate::utils::random_principal;
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::{PocketIc, PocketIcBuilder};
use sns_governance_canister::types::Neuron;
use sns_neuron_controller_api_canister::Args;
use std::collections::HashMap;
use types::BuildVersion;
use types::CanisterId;

#[derive(CandidType, Deserialize, Debug)]
pub struct RegisterDappCanisterRequest {
    pub canister_id: Option<Principal>,
}

pub struct SNCTestEnv {
    pub controller: Principal,
    pub neuron_data: HashMap<usize, Neuron>,
    pub token_ledgers: HashMap<String, Principal>,
    pub sns_neuron_controller_id: CanisterId,
    pub sns_governance_id: CanisterId,
    pub ogy_rewards_canister_id: CanisterId,
    pub gld_rewards_canister_id: CanisterId, // could be mocked
    pub pic: PocketIc,
}

use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
impl Debug for SNCTestEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SNCTestEnv")
            .field("controller", &self.controller.to_text())
            .field(
                "sns_neuron_controller_id",
                &self.sns_neuron_controller_id.to_text(),
            )
            .field("sns_governance_id", &self.sns_governance_id.to_text())
            .field(
                "ogy_rewards_canister_id",
                &self.ogy_rewards_canister_id.to_text(),
            )
            .field(
                "gld_rewards_canister_id",
                &self.gld_rewards_canister_id.to_text(),
            )
            .finish()
    }
}
pub struct SNCTestEnvBuilder {
    controller: Principal,
    token_symbols: Vec<String>,
    // Canister ids parameters
    sns_neuron_controller_id: CanisterId,
    sns_governance_id: CanisterId,
    ogy_rewards_canister_id: CanisterId,
    gld_rewards_canister_id: CanisterId, // could be mocked
    // Ledger parameters
    initial_ledger_accounts: Vec<(Account, Nat)>,
    ledger_fees: HashMap<String, Nat>,
}

impl Default for SNCTestEnvBuilder {
    fn default() -> Self {
        Self {
            controller: random_principal(),
            sns_neuron_controller_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            sns_governance_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            ogy_rewards_canister_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            gld_rewards_canister_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            token_symbols: vec![],
            initial_ledger_accounts: vec![],
            ledger_fees: HashMap::new(),
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
        let mut pic = PocketIcBuilder::new()
            .with_sns_subnet()
            .with_application_subnet()
            .build();

        let sns_subnet = pic.topology().get_sns().unwrap();
        self.ogy_rewards_canister_id =
            pic.create_canister_on_subnet(Some(self.controller.clone()), None, sns_subnet);
        self.sns_governance_id =
            pic.create_canister_on_subnet(Some(self.controller.clone()), None, sns_subnet);
        self.sns_neuron_controller_id =
            pic.create_canister_on_subnet(Some(self.controller.clone()), None, sns_subnet);
        self.gld_rewards_canister_id =
            pic.create_canister_on_subnet(Some(self.controller.clone()), None, sns_subnet);

        // NOTE: Neuron Permissions should be granted to the controller
        let (neuron_data, _) = generate_neuron_data(0, 1, 1, &vec![self.sns_neuron_controller_id]);
        let sns_gov_canister_id = create_sns_with_data(
            &mut pic,
            self.sns_governance_id,
            &neuron_data,
            &self.controller,
        );
        let token_ledgers = setup_ledgers(
            &pic,
            sns_gov_canister_id.clone(),
            self.token_symbols.clone(),
            self.initial_ledger_accounts.clone(),
            self.ledger_fees.clone(),
        );

        let ogy_sns_rewards_canister_id = setup_rewards_canister(
            &mut pic,
            self.ogy_rewards_canister_id,
            &token_ledgers,
            sns_gov_canister_id,
            &self.controller,
        );

        // let token_ledger_ids: Vec<Principal> =
        //     token_ledgers.iter().map(|(_, id)| id.clone()).collect();

        let ogy_sns_ledger_canister_id =
            token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();

        let snc_init_args = Args::Init(sns_neuron_controller_api_canister::init::InitArgs {
            test_mode: true,
            version: BuildVersion::min(),
            commit_hash: "integration_testing".to_string(),
            authorized_principals: vec![self.sns_governance_id],
            sns_rewards_canister_id: self.gld_rewards_canister_id,
            ogy_sns_governance_canister_id: self.sns_governance_id,
            ogy_sns_ledger_canister_id,
            ogy_sns_rewards_canister_id,
        });

        let snc_canister_id = setup_sns_neuron_controller_canister(
            &mut pic,
            self.sns_neuron_controller_id,
            snc_init_args,
            self.controller,
        );

        SNCTestEnv {
            controller: self.controller,
            neuron_data,
            token_ledgers,
            sns_neuron_controller_id: snc_canister_id,
            sns_governance_id: self.sns_governance_id,
            ogy_rewards_canister_id: ogy_sns_rewards_canister_id,
            gld_rewards_canister_id: self.gld_rewards_canister_id,
            pic,
        }
    }
}
