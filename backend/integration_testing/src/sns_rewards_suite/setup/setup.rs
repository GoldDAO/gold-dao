use std::{ collections::HashMap, time::SystemTime };

use candid::{ encode_one, Nat, Principal };
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::{ PocketIc, PocketIcBuilder };
use sns_governance_canister::types::Neuron;
use sns_rewards_api_canister::init::InitArgs;
use sns_rewards_api_canister::Args;
use types::BuildVersion;

use crate::{
    client::icrc1::client::transfer,
    sns_rewards_suite::setup::{
        setup_ledger::setup_ledgers,
        setup_sns::{ create_sns_with_data, generate_neuron_data },
    },
    utils::random_principal,
    wasms,
};

use super::{ setup_rewards::setup_rewards_canister, setup_sns::reinstall_sns_with_data };

pub static POCKET_IC_BIN: &str = "./pocket-ic";

pub fn setup_reward_pools(
    mut pic: &mut PocketIc,
    minting_account: &Principal,
    reward_canister_id: &Principal,
    canister_ids: &Vec<Principal>,
    amount: u64
) {
    let reward_account = Account {
        owner: reward_canister_id.clone(),
        subaccount: Some([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ]),
    };

    for canister_id in canister_ids.into_iter() {
        transfer(
            &mut pic,
            minting_account.clone(),
            canister_id.clone(),
            None,
            reward_account,
            amount.into()
        ).unwrap();
    }
}

pub struct RewardsTestEnv {
    pub controller: Principal,
    pub neuron_data: HashMap<usize, Neuron>,
    pub users: Vec<Principal>,
    pub token_ledgers: HashMap<String, Principal>,
    pub rewards_canister_id: Principal,
    pub sns_gov_canister_id: Principal,
    pub pic: PocketIc,
    pub neuron_owners: HashMap<Principal, usize>,
}

impl RewardsTestEnv {
    /// simulate neurons voting by reinstalling the sns gov canister with an increase in maturity
    /// each neuton's initial maturity is multiplied
    pub fn simulate_neuron_voting(&mut self, multiplier: u64) {
        let (neuron_data, _) = generate_neuron_data(
            0,
            self.neuron_data.len(),
            multiplier,
            &self.users
        );
        self.pic.tick();
        reinstall_sns_with_data(
            &mut self.pic,
            &neuron_data,
            &self.sns_gov_canister_id,
            &self.controller
        );
        self.pic.tick();
    }

    pub fn upgrade_rewards_canister(&mut self) {
        let icp_ledger_canister_id = self.token_ledgers
            .get("icp_ledger_canister_id")
            .expect("couldn't find ledger with 'icp_ledger_canister_id'")
            .clone();
        let sns_ledger_canister_id = self.token_ledgers
            .get("gldgov_ledger_canister_id")
            .expect("couldn't find ledger with 'gldgov_ledger_canister_id'")
            .clone();
        let ogy_ledger_canister_id = self.token_ledgers
            .get("ogy_ledger_canister_id")
            .expect("couldn't find ledger with 'ogy_ledger_canister_id'")
            .clone();

        let init_args = Args::Init(InitArgs {
            test_mode: true,
            version: BuildVersion::min(),
            commit_hash: "Test".to_string(),
            icp_ledger_canister_id,
            sns_ledger_canister_id,
            ogy_ledger_canister_id,
            sns_gov_canister_id: self.sns_gov_canister_id.clone(),
        });
        match
            self.pic.upgrade_canister(
                self.rewards_canister_id,
                wasms::REWARDS.clone(),
                encode_one(()).unwrap(),
                Some(self.controller.clone())
            )
        {
            Ok(m) => println!("{}", "upgrade success"),
            Err(m) => println!("{m:?}"),
        }
    }
}

pub struct RewardsTestEnvBuilder {
    controller: Principal,
    users: Vec<Principal>,
    token_symbols: Vec<String>,
    initial_ledger_accounts: Vec<(Account, Nat)>,
    neurons_to_create: usize,
    initial_reward_pool_amount: Nat,
    ledger_fees: HashMap<String, Nat>,
}

impl RewardsTestEnvBuilder {
    pub fn new() -> Self {
        let default_controller = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

        Self {
            controller: random_principal(),
            users: vec![],
            token_symbols: vec![],
            neurons_to_create: 0,
            initial_ledger_accounts: vec![],
            initial_reward_pool_amount: Nat::from(0u64),
            ledger_fees: HashMap::new(),
        }
    }

    /// is the controller of everything - no real need for this but nice to have if you want to be specific
    pub fn add_controller(mut self, principal: Principal) -> Self {
        self.controller = principal;
        self
    }

    /// users for neuron data - they will be added as hotkeys to neurons // each user users get added to neurons.len() / users.len(), repeating every users.len()
    pub fn add_users(mut self, users: Vec<Principal>) -> Self {
        self.users = users;
        self
    }

    pub fn add_token_ledger(
        mut self,
        symbol: &str,
        initial_balances: &mut Vec<(Account, Nat)>,
        transaction_fee: Nat
    ) -> Self {
        self.token_symbols.push(symbol.to_string());
        self.initial_ledger_accounts.append(initial_balances);
        self.ledger_fees.insert(symbol.to_string(), transaction_fee);
        self
    }

    pub fn add_random_neurons(mut self, amount: usize) -> Self {
        self.neurons_to_create = amount;
        self
    }

    pub fn with_reward_pools(mut self, amount: Nat) -> Self {
        self.initial_reward_pool_amount = amount; // Note - this counts as a mint and therefore increases total supply
        self
    }

    pub fn build(self) -> RewardsTestEnv {
        let mut pic = PocketIcBuilder::new().with_sns_subnet().with_application_subnet().build();

        // set the date
        // Wednesday Jun 19, 2024, 7:00:00 AM

        let (neuron_data, neuron_owners) = generate_neuron_data(
            0,
            self.neurons_to_create,
            1,
            &self.users
        );
        let sns_gov_canister_id = create_sns_with_data(&mut pic, &neuron_data, &self.controller);
        let token_ledgers = setup_ledgers(
            &pic,
            sns_gov_canister_id.clone(),
            self.token_symbols,
            self.initial_ledger_accounts,
            self.ledger_fees
        );
        let rewards_canister_id = setup_rewards_canister(
            &mut pic,
            &token_ledgers,
            &sns_gov_canister_id,
            &self.controller
        );
        let token_ledger_ids: Vec<Principal> = token_ledgers
            .iter()
            .map(|(_, id)| id.clone())
            .collect();
        if self.initial_reward_pool_amount > Nat::from(0u64) {
            setup_reward_pools(
                &mut pic,
                &sns_gov_canister_id,
                &rewards_canister_id,
                &token_ledger_ids,
                self.initial_reward_pool_amount.0.try_into().unwrap()
            );
        }
        pic.set_time(SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(1718701200000)); // Tuesday Jun 18, 2024, 9:00:00 AM

        RewardsTestEnv {
            controller: self.controller,
            neuron_data,
            users: self.users,
            token_ledgers,
            rewards_canister_id,
            sns_gov_canister_id,
            pic,
            neuron_owners,
        }
    }
}
