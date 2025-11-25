use crate::client::pocket::create_canister_with_id;
use crate::wasms::GLDT_SWAP_INDEX;
use candid::encode_one;
use candid::Principal;
use gldt_swap_index_api_canister::Args;
use pocket_ic::PocketIc;

pub fn setup_index_canister(
    pic: &PocketIc,
    gldt_swap_index_canister_id: Principal,
    args: Args,
    controller: Principal,
) -> Principal {
    let gldt_swap_index_canister_id =
        create_canister_with_id(pic, controller, &gldt_swap_index_canister_id.to_text());
    let index_wasm = GLDT_SWAP_INDEX.clone();
    pic.add_cycles(gldt_swap_index_canister_id, 100_000_000_000_000_000_000);

    pic.set_controllers(
        gldt_swap_index_canister_id,
        Some(controller.clone()),
        vec![controller.clone()],
    )
    .unwrap();
    pic.tick();

    pic.install_canister(
        gldt_swap_index_canister_id,
        index_wasm,
        encode_one(args).unwrap(),
        Some(controller.clone()),
    );
    pic.tick();

    gldt_swap_index_canister_id
}

pub fn upgrade_index_canister(
    pic: &mut PocketIc,
    gldt_swap_index_canister_id: Principal,
    args: Args,
    controller: Principal,
) {
    let index_wasm = GLDT_SWAP_INDEX.clone();
    pic.add_cycles(gldt_swap_index_canister_id, 100_000_000_000_000_000_000);

    pic.set_controllers(
        gldt_swap_index_canister_id,
        Some(controller.clone()),
        vec![controller.clone()],
    )
    .unwrap();
    pic.tick();

    pic.upgrade_canister(
        gldt_swap_index_canister_id,
        index_wasm,
        encode_one(args).unwrap(),
        Some(controller.clone()),
    )
    .unwrap();
}
