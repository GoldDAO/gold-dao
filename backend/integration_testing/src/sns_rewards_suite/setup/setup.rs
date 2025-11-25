use super::setup_rewards::setup_rewards_canister;
use crate::sns_test_env::sns_test_env::SnsTestEnv;
use crate::sns_test_env::utils::generate_neuron_data;
use crate::{
    client::icrc1::client::transfer, sns_rewards_suite::setup::setup_ledger::setup_ledgers,
    utils::random_principal, wasms,
};
use candid::{encode_one, Nat, Principal};
use bity_ic_canister_time::HOUR_IN_MS;
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::{PocketIc, PocketIcBuilder};
use sns_governance_canister::types::Neuron;
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

pub fn setup_reward_pools(
    mut pic: &PocketIc,
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

pub struct RewardsTestEnv {
    pub pic: Rc<RefCell<PocketIc>>,
    pub controller: Principal,
    pub gld_sns_test_env: SnsTestEnv,
    pub neuron_data: HashMap<usize, Neuron>,
    pub users: Vec<Principal>,
    pub token_ledgers: HashMap<String, Principal>,
    pub rewards_canister_id: Principal,
    pub sns_gov_canister_id: Principal,
    pub neuron_owners: HashMap<Principal, usize>,
}

impl RewardsTestEnv {
    /// simulate neurons voting by reinstalling the sns gov canister with an increase in maturity
    /// each neuron's initial maturity is multiplied
    pub fn simulate_neuron_voting(&self, multiplier: u64) {
        let pic = self.pic.borrow();
        let (neuron_data, _) =
            generate_neuron_data(0, self.neuron_data.len(), multiplier, &self.users);
        pic.tick();

        self.gld_sns_test_env
            .reinstall_governance_with_neuron_data(&neuron_data);
        pic.tick();
    }

    pub fn upgrade_rewards_canister(&self) {
        let pic = self.pic.borrow();

        match pic.upgrade_canister(
            self.rewards_canister_id,
            wasms::REWARDS.clone(),
            encode_one(()).unwrap(),
            Some(self.controller.clone()),
        ) {
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

    pub fn build(self) -> RewardsTestEnv {
        let pic_ref = Rc::new(RefCell::new(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_sns_subnet()
                .with_application_subnet()
                .build(),
        ));
        let pic = pic_ref.borrow();
        pic.set_time(
            (SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(1718697600000)).into(),
        ); // Tue Jun 18 2024 08:00:00 GMT

        let (gld_neuron_data, neuron_owners) =
            generate_neuron_data(0, self.neurons_to_create, 1, &self.users);
        let initial_ledger_accounts = vec![(
            sns_ledger_canister::types::Account {
                owner: self.controller,
                subaccount: None,
            },
            Nat::from(1_000_000_000_000_000u64),
        )];
        let gld_sns_test_env = SnsTestEnv::goldao(
            &pic_ref,
            self.controller,
            &gld_neuron_data,
            Some(initial_ledger_accounts),
        );
        let sns_gov_canister_id = gld_sns_test_env.governance_id;

        let mut token_ledgers = setup_ledgers(
            &pic,
            sns_gov_canister_id.clone(),
            self.token_symbols,
            self.initial_ledger_accounts,
            self.ledger_fees,
        );
        token_ledgers.insert(
            "goldao_ledger_canister_id".to_string(),
            gld_sns_test_env.ledger_id,
        );
        let rewards_canister_id =
            setup_rewards_canister(&pic, &token_ledgers, &sns_gov_canister_id, &self.controller);
        let token_ledger_ids: Vec<Principal> =
            token_ledgers.iter().map(|(_, id)| id.clone()).collect();
        if self.initial_reward_pool_amount > Nat::from(0u64) {
            setup_reward_pools(
                &pic,
                &sns_gov_canister_id,
                &rewards_canister_id,
                &token_ledger_ids,
                self.initial_reward_pool_amount.0.try_into().unwrap(),
            );
        }
        // Tuesday Jun 18, 2024, 9:00:00 AM
        pic.advance_time(Duration::from_millis(HOUR_IN_MS));
        pic.tick();
        pic.tick();
        pic.tick();
        pic.tick();
        RewardsTestEnv {
            controller: self.controller,
            gld_sns_test_env: gld_sns_test_env,
            neuron_data: gld_neuron_data,
            users: self.users,
            token_ledgers,
            rewards_canister_id,
            sns_gov_canister_id,
            pic: Rc::clone(&pic_ref),
            neuron_owners,
        }
    }
}
