use std::collections::{BTreeMap, HashMap};

use crate::cycles_manager_suite::setup::setup_burner::setup_burner_canister;
use candid::{encode_one, Principal};
use ic_stable_structures::vec;
use pocket_ic::PocketIc;
use sha2::{Digest, Sha256};
use sns_governance_canister::types::{
    governance::SnsMetadata, DefaultFollowees, Governance, NervousSystemParameters, Neuron,
    NeuronId, NeuronPermission, NeuronPermissionList, VotingRewardsParameters,
};

use crate::wasms;

use candid::CandidType;
use candid::Deserialize;

#[derive(Deserialize, CandidType)]
pub struct Args {
    pub dapp_canister_ids: Vec<Principal>,
    pub testflight: bool,
    pub latest_ledger_archive_poll_timestamp_seconds: Option<u64>,
    pub archive_canister_ids: Vec<Principal>,
    pub governance_canister_id: Option<Principal>,
    pub index_canister_id: Option<Principal>,
    pub swap_canister_id: Option<Principal>,
    pub ledger_canister_id: Option<Principal>,
}

pub fn setup_root_canister(pic: &mut PocketIc, controller: &Principal) -> Principal {
    // let mut sns_init_args = generate_sns_init_args(neuron_data);
    let sns_subnet_id = pic.topology().get_sns().unwrap();

    let sns_root_canister_id =
        pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet_id);
    pic.add_cycles(sns_root_canister_id, 200_000_000_000_000);
    pic.set_controllers(
        sns_root_canister_id,
        Some(controller.clone()),
        vec![controller.clone(), sns_root_canister_id.clone()],
    )
    .unwrap();
    pic.tick();

    let governance_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    let index_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    let swap_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 3]);
    let ledger_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 4]);

    // let burner_canister_id = setup_burner_canister(pic, controller);
    // println!("Burner canister: {}", burner_canister_id);

    let root_init_args = Args {
        dapp_canister_ids: vec![],
        testflight: true,
        latest_ledger_archive_poll_timestamp_seconds: None,
        archive_canister_ids: vec![],
        governance_canister_id: Some(*controller),
        index_canister_id: Some(index_canister_id),
        swap_canister_id: Some(swap_canister_id),
        ledger_canister_id: Some(ledger_canister_id),
    };

    pic.tick();
    let sns_root_canister_wasm = wasms::SNS_ROOT.clone();
    pic.install_canister(
        sns_root_canister_id,
        sns_root_canister_wasm,
        encode_one(root_init_args).unwrap(),
        Some(controller.clone()),
    );
    sns_root_canister_id
}
