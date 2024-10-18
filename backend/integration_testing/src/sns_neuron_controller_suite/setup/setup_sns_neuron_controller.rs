use crate::wasms::SNS_NEURON_CONTROLLER;
use candid::encode_one;
use candid::Principal;
use pocket_ic::PocketIc;

pub fn setup_sns_neuron_controller_canister(
    pic: &mut PocketIc,
    sns_neuron_controller_id: Principal,
    args: sns_neuron_controller_api_canister::Args,
    controller: Principal,
) -> Principal {
    let sns_neuron_controller_wasm = SNS_NEURON_CONTROLLER.clone();
    pic.add_cycles(sns_neuron_controller_id, 1_000_000_000_000_000);

    pic.set_controllers(
        sns_neuron_controller_id,
        Some(controller.clone()),
        vec![controller.clone()],
    )
    .unwrap();
    pic.tick();

    pic.install_canister(
        sns_neuron_controller_id,
        sns_neuron_controller_wasm,
        encode_one(args).unwrap(),
        Some(controller.clone()),
    );

    sns_neuron_controller_id
}
