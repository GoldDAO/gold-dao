use std::collections::HashMap;

use crate::cycles_manager_suite::setup::setup_cycles_manager::setup_cycle_manager_canister;
use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::{PocketIc, PocketIcBuilder};
use sns_governance_canister::types::Neuron;
use types::BuildVersion;
use types::Cycles;

use crate::{
    client::icrc1::client::transfer,
    cycles_manager_suite::setup::{
        setup_ledger::setup_ledgers,
        setup_sns::{create_sns_with_data, generate_neuron_data},
    },
    utils::random_principal,
};

use super::{setup_rewards::setup_rewards_canister, setup_sns::reinstall_sns_with_data};

const T: Cycles = 1_000_000_000_000;

pub fn setup_reward_pools(
    mut pic: &mut PocketIc,
    minting_account: &Principal,
    reward_canister_id: &Principal,
    canister_ids: &Vec<Principal>,
    amount: u64,
) {
    let reward_account = Account {
        owner: reward_canister_id.clone(),
        subaccount: Some([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]),
    };

    for canister_id in canister_ids.into_iter() {
        transfer(
            &mut pic,
            minting_account.clone(),
            canister_id.clone(),
            None,
            reward_account,
            amount.into(),
        )
        .unwrap();
    }
}

pub struct CyclesManagerEnv {
    pub controller: Principal,
    pub neuron_data: HashMap<usize, Neuron>,
    pub users: Vec<Principal>,
    pub token_ledgers: HashMap<String, Principal>,
    pub cycles_manager_id: Principal,
    pub rewards_canister_id: Principal,
    pub sns_gov_canister_id: Principal,
    pub pic: PocketIc,
    pub neuron_owners: HashMap<Principal, usize>,
}

impl CyclesManagerEnv {}

pub struct CyclesManagerTestEnvBuilder {
    controller: Principal,
    users: Vec<Principal>,
    token_symbols: Vec<String>,
    initial_ledger_accounts: Vec<(Account, Nat)>,
    neurons_to_create: usize,
    initial_reward_pool_amount: Nat,
    ledger_fees: HashMap<String, Nat>,
}

impl CyclesManagerTestEnvBuilder {
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
        transaction_fee: Nat,
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

    pub fn build(self) -> CyclesManagerEnv {
        let mut pic = PocketIcBuilder::new()
            .with_sns_subnet()
            .with_application_subnet()
            .build();

        let (neuron_data, neuron_owners) =
            generate_neuron_data(0, self.neurons_to_create, 1, &self.users);
        let sns_gov_canister_id = create_sns_with_data(&mut pic, &neuron_data, &self.controller);
        let token_ledgers = setup_ledgers(
            &pic,
            sns_gov_canister_id.clone(),
            self.token_symbols,
            self.initial_ledger_accounts,
            self.ledger_fees,
        );
        let rewards_canister_id = setup_rewards_canister(
            &mut pic,
            &token_ledgers,
            &sns_gov_canister_id,
            &self.controller,
        );

        let token_ledger_ids: Vec<Principal> =
            token_ledgers.iter().map(|(_, id)| id.clone()).collect();
        if self.initial_reward_pool_amount > Nat::from(0u64) {
            setup_reward_pools(
                &mut pic,
                &sns_gov_canister_id,
                &rewards_canister_id,
                &token_ledger_ids,
                self.initial_reward_pool_amount.0.try_into().unwrap(),
            );
        }

        // Args
        let cycles_dispenser_init_args = cycles_manager_canister::init::Args {
            test_mode: true,
            governance_principals: vec![self.controller],
            canisters: vec![rewards_canister_id],
            max_top_up_amount: 1 * T,
            min_interval: 5 * 60 * 1000,             // 5 minutes
            min_cycles_balance: 200_000_000_000_000, // 200_000_000_000_000
            wasm_version: BuildVersion::min(),
        };

        // Make setup
        let cycles_manager_id: Principal =
            setup_cycle_manager_canister(&mut pic, &self.controller, cycles_dispenser_init_args);

        CyclesManagerEnv {
            controller: self.controller,
            neuron_data,
            users: self.users,
            token_ledgers,
            cycles_manager_id,
            rewards_canister_id,
            sns_gov_canister_id,
            pic,
            neuron_owners,
        }
    }
}
