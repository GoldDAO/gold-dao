use crate::wasms::BUYBACK_BURN;
use candid::encode_one;
use candid::Principal;
use pocket_ic::PocketIc;

pub fn setup_buyback_burn_canister(
    pic: &mut PocketIc,
    buyback_burn_id: Principal,
    args: buyback_burn_api::Args,
    controller: Principal,
) -> Principal {
    let buyback_burn_wasm = BUYBACK_BURN.clone();
    pic.add_cycles(buyback_burn_id, 1_000_000_000_000_000);

    pic.set_controllers(
        buyback_burn_id,
        Some(controller.clone()),
        vec![controller.clone()],
    )
    .unwrap();
    pic.tick();

    pic.install_canister(
        buyback_burn_id,
        buyback_burn_wasm,
        encode_one(args).unwrap(),
        Some(controller.clone()),
    );

    buyback_burn_id
}

use bity_ic_types::BuildVersion;
use buyback_burn_api::post_upgrade::UpgradeArgs;
use buyback_burn_api::Args;
use pocket_ic::RejectResponse;
pub fn upgrade_buyback_burn_canister(
    pic: &PocketIc,
    canister_id: Principal,
    controller: &Principal,
) -> std::result::Result<(), RejectResponse> {
    let buyback_burn_wasm = crate::wasms::BUYBACK_BURN.clone();

    let upgrade_args = Args::Upgrade(UpgradeArgs {
        version: BuildVersion::min(),
        commit_hash: "Test".to_string(),
    });

    let encoded_args = encode_one(upgrade_args).expect("Failed to encode upgrade args");

    pic.upgrade_canister(
        canister_id,
        buyback_burn_wasm,
        encoded_args,
        Some(controller.clone()),
    )
}
