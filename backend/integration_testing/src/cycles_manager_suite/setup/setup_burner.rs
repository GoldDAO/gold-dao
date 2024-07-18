use candid::{encode_one, CandidType, Principal};
use pocket_ic::PocketIc;
use serde::{Deserialize, Serialize};

use crate::wasms;
#[derive(Debug, CandidType, Serialize, Deserialize)]
pub struct InitArgs {
    /// Interval between timers in seconds.
    pub interval_between_timers_in_seconds: u128,
    /// Amount of burned cycles per timer.
    pub burn_amount: u128,
}

pub fn setup_burner_canister(
    pic: &mut PocketIc,
    controller: &Principal,
    burner_canister_init_args: InitArgs,
    sns_root_canister_id: Principal,
) -> Principal {
    let sns_subnet = pic.topology().get_sns().unwrap();
    let burner_canister = pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet);

    let burner_wasm = wasms::BURNER.clone();
    pic.add_cycles(burner_canister, 20_000_000_000_000);
    pic.set_controllers(
        burner_canister,
        Some(controller.clone()),
        vec![controller.clone(), sns_root_canister_id],
    )
    .unwrap();
    pic.tick();

    pic.install_canister(
        burner_canister,
        burner_wasm,
        encode_one(burner_canister_init_args).unwrap(),
        Some(sns_root_canister_id.clone()),
    );
    burner_canister
}
