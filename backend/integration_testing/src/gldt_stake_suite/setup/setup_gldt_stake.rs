use crate::wasms::GLDT_STAKE;
use candid::encode_one;
use candid::Principal;
use pocket_ic::PocketIc;

pub fn setup_gldt_stake_canister(
    pic: &mut PocketIc,
    canister_id: Principal,
    args: gldt_stake_api_canister::Args,
    controller: Principal,
) -> Principal {
    let sns_neuron_controller_wasm = GLDT_STAKE.clone();
    pic.add_cycles(canister_id, 10_000_000_000_000_000);

    pic.set_controllers(
        canister_id,
        Some(controller.clone()),
        vec![controller.clone()],
    )
    .unwrap();

    pic.tick();

    pic.install_canister(
        canister_id,
        sns_neuron_controller_wasm,
        encode_one(args).unwrap(),
        Some(controller.clone()),
    );

    canister_id
}
