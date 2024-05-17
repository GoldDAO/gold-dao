use candid::{encode_one, CandidType, Principal};
use pocket_ic::PocketIc;
use serde::{Deserialize, Serialize};
use types::BuildVersion;

use crate::wasms;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Empty {}

pub fn setup_burner_canister(pic: &mut PocketIc, controller: &Principal) -> Principal {
    let sns_subnet = pic.topology().get_sns().unwrap();
    let burner_canister = pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet);

    let root_canister_id = Principal::from_text("lqy7q-dh777-77777-aaaaq-cai").unwrap();

    let burner_wasm = wasms::BURNER.clone();
    pic.add_cycles(burner_canister, 200_000_000_000_000);
    pic.set_controllers(
        burner_canister,
        Some(controller.clone()),
        vec![controller.clone(), root_canister_id],
    )
    .unwrap();
    pic.tick();

    let sns_root_canister_id = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 9]);
    let cycles_dispenser_init_args = cycles_manager_canister::init::InitArgs {
        test_mode: true,
        authorized_principals: vec![root_canister_id], //*controller,
        canisters: vec![],
        sns_root_canister: Some(sns_root_canister_id),
        max_top_up_amount: 0,
        min_interval: 5 * 60 * 1000, // 5 minutes
        min_cycles_balance: 10000000000000000000,
        wasm_version: BuildVersion::min(),
    };

    pic.install_canister(
        burner_canister,
        burner_wasm,
        encode_one(cycles_dispenser_init_args).unwrap(),
        Some(root_canister_id.clone()),
    );
    burner_canister
}
