use candid::{encode_one, Principal};
use pocket_ic::PocketIc;

use crate::wasms;

use candid::CandidType;
use candid::Deserialize;

#[derive(Deserialize, CandidType)]
pub struct Args {
    pub ledger_canister_id: Principal,
    pub governance_canister_id: Principal,
    pub minting_account_id: Option<String>,
    pub last_purged_notification: Option<u64>,
}

pub fn setup_cycles_minting(
    pic: &mut PocketIc,
    controller: &Principal,
    cycles_minting_init_args: Args,
) -> Principal {
    // let mut sns_init_args = generate_sns_init_args(neuron_data);
    let sns_subnet_id = pic.topology().get_sns().unwrap();

    let cycles_minting_canister_id =
        pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet_id);
    pic.add_cycles(cycles_minting_canister_id, 200_000_000_000_000);
    pic.set_controllers(
        cycles_minting_canister_id,
        Some(controller.clone()),
        vec![controller.clone(), cycles_minting_canister_id.clone()],
    )
    .unwrap();
    pic.tick();

    pic.tick();
    let cycles_minting_canister_wasm = wasms::CYCLES_MINTING_CANISTER.clone();
    pic.install_canister(
        cycles_minting_canister_id,
        cycles_minting_canister_wasm,
        encode_one(cycles_minting_init_args).unwrap(),
        Some(controller.clone()),
    );
    cycles_minting_canister_id
}
