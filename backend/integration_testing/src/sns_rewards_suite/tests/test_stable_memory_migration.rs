use crate::client::rewards::_insert_mock_neuron_info;
use crate::client::rewards::get_neuron_by_id;
use crate::sns_rewards_suite::setup::setup_rewards::upgrade_rewards_canister;
use crate::wasms;
use crate::{sns_rewards_suite::setup::default_test_setup, utils::tick_n_blocks};
use candid::{encode_one, Principal};
use canister_time::{DAY_IN_MS, HOUR_IN_MS};
use pocket_ic::PocketIc;
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::init::InitArgs;
use sns_rewards_api_canister::Args;
use std::collections::HashMap;
use std::time::Duration;
use types::BuildVersion;
use types::NeuronInfo;

#[test]
fn test_migration_happy_path() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let sns_rewards_id = test_env.rewards_canister_id;
    setup_old_rewards_canister(
        &pic,
        sns_rewards_id,
        &test_env.token_ledgers,
        test_env.sns_gov_canister_id,
        &test_env.controller,
    );

    // ********************************
    // 1. Distribute rewards
    // ********************************
    let n = pic.get_time();
    println!("now is : {n:?}");
    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    tick_n_blocks(&pic, 20);
    pic.advance_time(Duration::from_millis(DAY_IN_MS)); // 9:00am Wednesday 19th June

    tick_n_blocks(&pic, 100);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 5)); // 14:00
    tick_n_blocks(&pic, 40);

    let _ = insert_mock_neurons(&pic, test_env.controller, sns_rewards_id, 100);

    upgrade_rewards_canister(&pic, sns_rewards_id, &test_env.controller).unwrap();

    let neurons = get_mock_neurons(&pic, test_env.controller, sns_rewards_id, 100);

    for neuron in neurons {
        assert!(neuron.is_some(), "Neuron should exist after migration");
        let n = neuron.unwrap();
    }
}

fn insert_mock_neurons(
    pic: &PocketIc,
    controller: Principal,
    sns_rewards_id: Principal,
    amount: u64,
) {
    for i in 1..=amount {
        // Generate neuron ID from index
        let hex_id = format!("{:064x}", i);
        let neuron_id = NeuronId::new(&hex_id).unwrap();

        let args = sns_rewards_api_canister::_insert_mock_neuron_info::Args {
            neuron_id,
            neuron_info: NeuronInfo {
                ..Default::default()
            },
        };

        let _ = _insert_mock_neuron_info(pic, controller, sns_rewards_id, &args);
    }
}

fn get_mock_neurons(
    pic: &PocketIc,
    controller: Principal,
    sns_rewards_id: Principal,
    amount: u64,
) -> Vec<Option<NeuronInfo>> {
    let mut neurons = Vec::with_capacity(amount as usize);

    for i in 1..=amount {
        // Generate neuron ID from index (64-char hex string)
        let hex_id = format!("{:064x}", i);
        let neuron_id = NeuronId::new(&hex_id).unwrap();

        // Fetch neuron info and push to list
        let neuron = get_neuron_by_id(pic, controller, sns_rewards_id, &neuron_id);
        neurons.push(neuron);
    }

    neurons
}

pub fn setup_old_rewards_canister(
    pic: &PocketIc,
    sns_rewards_id: Principal,
    token_ledgers: &HashMap<String, Principal>,
    sns_canister_id: Principal,
    controller: &Principal,
) -> Principal {
    let rewards_wasm = wasms::REWARDS_OLD.clone();
    pic.add_cycles(sns_rewards_id, 100_000_000_000_000_000);
    pic.set_controllers(
        sns_rewards_id,
        Some(controller.clone()),
        vec![controller.clone()],
    )
    .unwrap();
    pic.tick();

    let icp_ledger_canister_id = token_ledgers
        .get("icp_ledger_canister_id")
        .expect("couldn't find ledger with 'icp_ledger_canister_id'")
        .clone();
    let sns_ledger_canister_id = token_ledgers
        .get("goldao_ledger_canister_id")
        .expect("couldn't find ledger with 'goldao_ledger_canister_id'")
        .clone();
    let ogy_ledger_canister_id = token_ledgers
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
        sns_gov_canister_id: sns_canister_id.clone(),
    });
    let _ = pic
        .reinstall_canister(
            sns_rewards_id,
            rewards_wasm,
            encode_one(init_args).unwrap(),
            Some(controller.clone()),
        )
        .unwrap();
    sns_rewards_id
}
