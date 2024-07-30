use std::collections::{BTreeMap, HashMap};

use candid::{encode_one, Principal};
use pocket_ic::PocketIc;
use sha2::{Digest, Sha256};
use sns_governance_canister::types::{
    governance::SnsMetadata, DefaultFollowees, Governance, NervousSystemParameters, Neuron,
    NeuronId, NeuronPermission, NeuronPermissionList, VotingRewardsParameters,
};

use crate::wasms;

// generates random neurons
pub fn generate_neuron_data(
    start_at: usize,
    n: usize,
    maturity_multiplier: u64,
    users: &Vec<Principal>,
) -> (HashMap<usize, Neuron>, HashMap<Principal, usize>) {
    let mut neuron_data = HashMap::new();
    let mut owner_map = HashMap::new();
    let mut index_user = 0;
    for i in start_at..n {
        let neuron_id = neuron_id_from_number(i);
        let user_principal = users.get(index_user).clone();
        let perms = create_neuron_permissions(user_principal);
        let neuron = create_neuron(neuron_id, maturity_multiplier, perms);
        neuron_data.insert(i, neuron);
        if user_principal.is_some() {
            owner_map.insert(user_principal.unwrap().clone(), i);
        }
        if users.len() >= 1 && index_user == users.len() - 1 {
            index_user = 0;
        }
    }

    (neuron_data, owner_map)
}

pub fn create_neuron(
    id: NeuronId,
    maturity_multiplier: u64,
    perms: Vec<NeuronPermission>,
) -> Neuron {
    Neuron {
        id: Some(id),
        permissions: perms,
        cached_neuron_stake_e8s: 20000u64,
        neuron_fees_e8s: 0u64,
        created_timestamp_seconds: 1713271942,
        aging_since_timestamp_seconds: 1713271942,
        followees: BTreeMap::new(),
        maturity_e8s_equivalent: 100_000 * maturity_multiplier,
        voting_power_percentage_multiplier: 1,
        source_nns_neuron_id: None,
        staked_maturity_e8s_equivalent: Some(100000),
        auto_stake_maturity: Some(false),
        vesting_period_seconds: Some(100000),
        disburse_maturity_in_progress: vec![],
        dissolve_state: Some(
            sns_governance_canister::types::neuron::DissolveState::WhenDissolvedTimestampSeconds(
                100000000000,
            ),
        ),
    }
}

pub fn create_neuron_permissions(user_hotkey: Option<&Principal>) -> Vec<NeuronPermission> {
    let mut perms = vec![NeuronPermission {
        principal: Some(Principal::anonymous()),
        permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    }];
    if user_hotkey.is_some() {
        perms.push(NeuronPermission {
            principal: Some(user_hotkey.unwrap().clone()),
            permission_type: vec![3, 4],
        });
    }
    perms
}

pub fn neuron_id_from_number(n: usize) -> NeuronId {
    // Hash the random number using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&n.to_be_bytes());
    let hash_result = hasher.finalize();

    // Convert the hash result to hexadecimal string
    let hex_id = hex::encode(hash_result);
    NeuronId::new(&hex_id).unwrap()
}

pub fn create_sns_with_data(
    pic: &mut PocketIc,
    sns_gov_id: Principal,
    neuron_data: &HashMap<usize, Neuron>,
    controller: &Principal,
) -> Principal {
    let sns_init_args = generate_sns_init_args(neuron_data);
    // let sns_subnet_id = pic.topology().get_sns().unwrap();
    // let sns_gov_id = pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet_id);

    // let sns_gov_id = pic
    //     .create_canister_with_id(Some(controller.clone()), None, sns_gov_id)
    //     .unwrap();

    pic.add_cycles(sns_gov_id, 100_000_000_000_000_000);
    pic.set_controllers(
        sns_gov_id,
        Some(controller.clone()),
        vec![controller.clone(), sns_gov_id.clone()],
    )
    .unwrap();

    pic.tick();
    let sns_gov_wasm = wasms::SNS_GOVERNANCE.clone();
    pic.install_canister(
        sns_gov_id,
        sns_gov_wasm,
        encode_one(sns_init_args.clone()).unwrap(),
        Some(controller.clone()),
    );

    sns_gov_id
}

// pub fn reinstall_sns_with_data(
//     pic: &mut PocketIc,
//     neuron_data: &HashMap<usize, Neuron>,
//     sns_gov_canister_id: &Principal,
//     controller: &Principal,
// ) {
//     let sns_init_args = generate_sns_init_args(neuron_data);

//     let sns_gov_wasm = wasms::SNS_GOVERNANCE.clone();
//     pic.stop_canister(sns_gov_canister_id.clone(), Some(controller.clone()))
//         .unwrap();
//     pic.tick();
//     pic.reinstall_canister(
//         sns_gov_canister_id.clone(),
//         sns_gov_wasm,
//         encode_one(sns_init_args.clone()).unwrap(),
//         Some(controller.clone()),
//     )
//     .unwrap();
//     pic.tick();
//     pic.start_canister(sns_gov_canister_id.clone(), Some(controller.clone()))
//         .unwrap();

//     pic.tick();
// }

pub fn generate_sns_init_args(neuron_data: &HashMap<usize, Neuron>) -> Governance {
    let sns_root_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    let sns_ledger_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 3]);
    let sns_swap_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 4]);

    let neuron_data_with_neuron_keys: BTreeMap<String, Neuron> = neuron_data
        .iter() // Iterate over the entries of the original map
        .map(|(key, value)| {
            (
                neuron_id_from_number(key.clone()).to_string(),
                value.clone(),
            )
        }) // Convert usize keys to String
        .collect();

    Governance {
        deployed_version: None,
        neurons: neuron_data_with_neuron_keys,
        proposals: BTreeMap::new(),
        parameters: Some(NervousSystemParameters {
            default_followees: Some(DefaultFollowees {
                followees: BTreeMap::new(),
            }),
            reject_cost_e8s: Some(10000u64),
            neuron_minimum_stake_e8s: Some(20000u64),
            transaction_fee_e8s: Some(10000u64),
            max_proposals_to_keep_per_action: Some(10),
            initial_voting_period_seconds: Some(2591000),
            wait_for_quiet_deadline_increase_seconds: Some(60 * 60),
            max_number_of_neurons: Some(1000u64),
            neuron_minimum_dissolve_delay_to_vote_seconds: Some(1u64),
            max_followees_per_function: Some(10),
            max_dissolve_delay_seconds: Some(10000000u64),
            max_neuron_age_for_age_bonus: Some(10000000),
            max_number_of_proposals_with_ballots: Some(100u64),
            neuron_claimer_permissions: Some(NeuronPermissionList {
                permissions: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            }),
            neuron_grantable_permissions: Some(NeuronPermissionList {
                permissions: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            }),
            max_number_of_principals_per_neuron: Some(10),
            voting_rewards_parameters: Some(VotingRewardsParameters {
                round_duration_seconds: Some(1000),
                reward_rate_transition_duration_seconds: Some(100),
                initial_reward_rate_basis_points: Some(5),
                final_reward_rate_basis_points: Some(5),
            }),
            max_dissolve_delay_bonus_percentage: Some(10u64),
            max_age_bonus_percentage: Some(10u64),
            maturity_modulation_disabled: Some(false),
        }),
        latest_reward_event: None,
        in_flight_commands: BTreeMap::new(),
        genesis_timestamp_seconds: 1713271942u64,
        metrics: None,
        ledger_canister_id: Some(sns_ledger_canister_id.clone()),
        root_canister_id: Some(sns_root_canister_id.clone()),
        id_to_nervous_system_functions: BTreeMap::new(),
        mode: 2,
        swap_canister_id: Some(sns_swap_canister_id.clone()),
        sns_metadata: Some(SnsMetadata {
            logo: None,
            url: Some("https://simgov.simgov".to_string()),
            name: Some("Simulation Gov".to_string()),
            description: Some("Simulation Gov desc".to_string()),
        }),
        sns_initialization_parameters: "".to_string(),
        pending_version: None,
        is_finalizing_disburse_maturity: None,
        maturity_modulation: None,
    }
}
