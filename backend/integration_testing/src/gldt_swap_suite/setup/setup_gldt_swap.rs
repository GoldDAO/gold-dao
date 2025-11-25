use crate::client::pocket::create_canister_with_id;
use crate::wasms::GLDT_SWAP;
use candid::encode_one;
use candid::Principal;
use pocket_ic::PocketIc;

pub fn setup_gldt_swap_canister(
    pic: &PocketIc,
    controller: Principal,
    gldt_swap_canister_id: Principal,
    args: gldt_swap_api_canister::Args,
) -> Principal {
    let gldt_swap_canister_id =
        create_canister_with_id(pic, controller, &gldt_swap_canister_id.to_text());
    let gldt_swap_wasm = GLDT_SWAP.clone();
    pic.add_cycles(gldt_swap_canister_id, 1_000_000_000_000_000);

    pic.set_controllers(
        gldt_swap_canister_id,
        Some(controller.clone()),
        vec![controller],
    )
    .unwrap();
    pic.tick();

    pic.install_canister(
        gldt_swap_canister_id,
        gldt_swap_wasm,
        encode_one(args).unwrap(),
        Some(controller.clone()),
    );

    gldt_swap_canister_id
}
