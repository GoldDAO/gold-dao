use crate::gldt_stake_suite::setup::setup_gldt_stake::setup_gldt_stake_canister;
use crate::gldt_stake_suite::setup::setup_ledger::setup_ledgers;
use crate::gldt_stake_suite::setup::setup_rewards::setup_rewards_canister;
use crate::gldt_stake_suite::setup::setup_sns::create_sns_with_data;
use crate::gldt_stake_suite::setup::setup_sns::generate_neuron_data;
use crate::gldt_stake_suite::setup::*;
use crate::utils::random_principal;
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;
use canister_time::DAY_IN_MS;
use gldt_stake_api_canister::Args;
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::{PocketIc, PocketIcBuilder};
use sns_governance_canister::types::Neuron;
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;
use types::BuildVersion;
use types::CanisterId;

#[derive(CandidType, Deserialize, Debug)]
pub struct RegisterDappCanisterRequest {
    pub canister_id: Option<Principal>,
}

pub struct GldtStakeTestEnv {
    pub controller: Principal,
    pub neuron_data: HashMap<usize, Neuron>,
    pub token_ledgers: HashMap<String, Principal>,
    pub gldt_stake_canister_id: CanisterId,
    pub sns_governance_id: CanisterId,
    pub gld_rewards_canister_id: CanisterId, // could be mocked
    pub pic: PocketIc,
    pub ledger_fees: HashMap<String, Nat>,
}

use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
impl Debug for GldtStakeTestEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("GldtStakeTestEnv")
            .field("controller", &self.controller.to_text())
            .field(
                "gldt_stake_canister_id",
                &self.gldt_stake_canister_id.to_text(),
            )
            .field("sns_governance_id", &self.sns_governance_id.to_text())
            .field(
                "gld_rewards_canister_id",
                &self.gld_rewards_canister_id.to_text(),
            )
            .finish()
    }
}
pub struct GldtStakeTestEnvBuilder {
    controller: Principal,
    token_symbols: Vec<String>,
    // Canister ids parameters
    sns_neuron_controller_id: CanisterId,
    sns_governance_id: CanisterId,
    gld_rewards_canister_id: CanisterId, // could be mocked
    // Ledger parameters
    initial_ledger_accounts: Vec<(Account, Nat)>,
    ledger_fees: HashMap<String, Nat>,
}

impl Default for GldtStakeTestEnvBuilder {
    fn default() -> Self {
        Self {
            controller: random_principal(),
            sns_neuron_controller_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            sns_governance_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            gld_rewards_canister_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            token_symbols: vec![],
            initial_ledger_accounts: vec![],
            ledger_fees: HashMap::new(),
        }
    }
}

impl GldtStakeTestEnvBuilder {
    pub fn new() -> Self {
        GldtStakeTestEnvBuilder::default()
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

    pub fn build(&mut self) -> GldtStakeTestEnv {
        let mut pic = PocketIcBuilder::new()
            .with_sns_subnet()
            .with_application_subnet()
            .with_fiduciary_subnet()
            .with_nns_subnet()
            .with_system_subnet()
            .build();

        let sns_subnet = pic.topology().get_sns().unwrap();
        self.sns_governance_id =
            pic.create_canister_on_subnet(Some(self.controller.clone()), None, sns_subnet);
        self.gld_rewards_canister_id =
            pic.create_canister_on_subnet(Some(self.controller.clone()), None, sns_subnet);
        let gldt_stake_canister_id =
            pic.create_canister_on_subnet(Some(self.controller.clone()), None, sns_subnet);

        // NOTE: Neuron Permissions should be granted to the controller
        let (neuron_data, _) = generate_neuron_data(0, 2, 1, &vec![gldt_stake_canister_id]);
        let sns_gov_canister_id = create_sns_with_data(
            &mut pic,
            self.sns_governance_id,
            &neuron_data,
            &self.controller,
        );
        let token_ledgers = setup_ledgers(
            &pic,
            self.controller.clone(),
            self.token_symbols.clone(),
            self.initial_ledger_accounts.clone(),
            self.ledger_fees.clone(),
        );

        let gld_sns_rewards_canister_id = setup_rewards_canister(
            &mut pic,
            self.gld_rewards_canister_id,
            &token_ledgers,
            sns_gov_canister_id,
            &self.controller,
        );

        pic.advance_time(Duration::from_millis(DAY_IN_MS * 30));

        // let token_ledger_ids: Vec<Principal> =
        //     token_ledgers.iter().map(|(_, id)| id.clone()).collect();
        let mut reward_types = HashMap::new();
        reward_types.insert(
            "GOLDAO".to_string(),
            (
                token_ledgers
                    .get("goldao_ledger_canister_id")
                    .unwrap()
                    .clone(),
                self.ledger_fees.get("GOLDAO").unwrap().clone(),
            ),
        );
        reward_types.insert(
            "OGY".to_string(),
            (
                token_ledgers.get("ogy_ledger_canister_id").unwrap().clone(),
                self.ledger_fees.get("OGY").unwrap().clone(),
            ),
        );
        reward_types.insert(
            "ICP".to_string(),
            (
                token_ledgers.get("icp_ledger_canister_id").unwrap().clone(),
                self.ledger_fees.get("ICP").unwrap().clone(),
            ),
        );

        let gldt_stake_init_args = Args::Init(gldt_stake_api_canister::init::InitArgs {
            test_mode: true,
            version: BuildVersion::min(),
            commit_hash: "integration_testing".to_string(),
            authorized_principals: vec![self.controller],
            gld_sns_rewards_canister_id: gld_sns_rewards_canister_id,
            gld_sns_governance_canister_id: self.sns_governance_id,
            goldao_ledger_id: token_ledgers
                .get("goldao_ledger_canister_id")
                .unwrap()
                .clone(),
            gldt_ledger_id: token_ledgers
                .get("gldt_ledger_canister_id")
                .unwrap()
                .clone(),
            reward_types: reward_types,
        });

        setup_gldt_stake_canister(
            &mut pic,
            gldt_stake_canister_id,
            gldt_stake_init_args,
            self.controller,
        );

        // pic.set_time(SystemTime::now());
        pic.set_time(SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(1733486460000)); // Friday 6 Dec 2024, 12:01:00

        GldtStakeTestEnv {
            controller: self.controller,
            neuron_data,
            token_ledgers,
            gldt_stake_canister_id: gldt_stake_canister_id,
            sns_governance_id: self.sns_governance_id,
            gld_rewards_canister_id: self.gld_rewards_canister_id,
            pic,
            ledger_fees: self.ledger_fees.clone(),
        }
    }
}
