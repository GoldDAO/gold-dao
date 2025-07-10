use candid::{Nat, Principal};
use sha2::{Digest, Sha256};
use sns_governance_canister::types::governance::Version;
use sns_governance_canister::types::{Governance, Neuron, NeuronId};
use sns_ledger_canister::types::Account;
use std::collections::{BTreeMap, HashMap};

pub fn neuron_id_from_number(n: usize) -> NeuronId {
    // Hash the random number using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&n.to_be_bytes());
    let hash_result = hasher.finalize();

    // Convert the hash result to hexadecimal string
    let hex_id = hex::encode(hash_result);
    NeuronId::new(&hex_id).unwrap()
}

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
        cached_neuron_stake_e8s: 3000000000000u64,
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

// pub fn create_neuron(
//     id: NeuronId,
//     maturity_multiplier: u64,
//     perms: Vec<NeuronPermission>,
// ) -> Neuron {
//     Neuron {
//         id: Some(id),
//         permissions: perms,
//         cached_neuron_stake_e8s: 3000000000000u64,
//         neuron_fees_e8s: 0u64,
//         created_timestamp_seconds: 1620329630,
//         aging_since_timestamp_seconds: 1620329630,
//         followees: BTreeMap::new(),
//         maturity_e8s_equivalent: 1 * maturity_multiplier,
//         voting_power_percentage_multiplier: 1,
//         source_nns_neuron_id: None,
//         staked_maturity_e8s_equivalent: Some(10),
//         auto_stake_maturity: Some(false),
//         vesting_period_seconds: Some(100000),
//         disburse_maturity_in_progress: vec![],
//         dissolve_state: Some(
//             sns_governance_canister::types::neuron::DissolveState::WhenDissolvedTimestampSeconds(
//                 100000000000,
//             ),
//         ),
//     }
// }

use sns_governance_canister::types::NeuronPermission;
pub fn create_neuron_permissions(user_hotkey: Option<&Principal>) -> Vec<NeuronPermission> {
    let mut perms = Vec::new();

    if let Some(hotkey) = user_hotkey {
        // Add the hotkey permissions
        perms.push(NeuronPermission {
            principal: Some(hotkey.clone()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });
    } else {
        // If no user_hotkey, add anonymous permissions
        perms.push(NeuronPermission {
            principal: Some(Principal::anonymous()),
            permission_type: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        });
    }

    perms
}
