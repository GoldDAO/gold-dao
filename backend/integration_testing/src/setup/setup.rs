use std::collections::HashMap;

use candid::{ Nat, Principal };
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::{ PocketIc, PocketIcBuilder };
use sns_governance_canister::types::Neuron;

use crate::{
    client::icrc1::client::transfer,
    setup::{
        setup_ledger::setup_ledgers,
        setup_sns::{ create_sns_with_data, generate_neuron_data },
    },
    utils::random_principal,
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
}

pub struct RewardsTestEnvBuilder {
    controller: Principal,
    users: Vec<Principal>,
    token_symbols: Vec<String>,
    initial_ledger_accounts: Vec<(Account, Nat)>,
    neurons_to_create: usize,
}

impl RewardsTestEnvBuilder {
    pub fn new() -> Self {
        let default_controller = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        let default_ledger_accounts = vec![(
            Account::from(default_controller),
            Nat::from(1_000_000_000_000_000u64),
        )];
        Self {
            controller: random_principal(),
            users: vec![],
            token_symbols: vec![],
            neurons_to_create: 0,
            initial_ledger_accounts: default_ledger_accounts.clone(),
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
        initial_balances: &mut Vec<(Account, Nat)>
    ) -> Self {
        self.token_symbols.push(symbol.to_string());
        self.initial_ledger_accounts.append(initial_balances);
        self
    }

    pub fn add_random_neurons(mut self, amount: usize) -> Self {
        self.neurons_to_create = amount;
        self
    }

    pub fn build(self) -> RewardsTestEnv {
        let mut pic = PocketIcBuilder::new().with_sns_subnet().with_application_subnet().build();
        let token_ledgers = setup_ledgers(
            &pic,
            self.controller,
            self.token_symbols,
            self.initial_ledger_accounts
        );
        let (neuron_data, neuron_owners) = generate_neuron_data(
            0,
            self.neurons_to_create,
            1,
            &self.users
        );
        let sns_gov_canister_id = create_sns_with_data(&mut pic, &neuron_data, &self.controller);
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
        setup_reward_pools(
            &mut pic,
            &self.controller,
            &rewards_canister_id,
            &token_ledger_ids,
            100_000_000_000
        );
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
