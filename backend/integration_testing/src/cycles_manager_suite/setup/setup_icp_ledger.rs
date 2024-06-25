use candid::{encode_one, Principal};
use ic_ledger_types::Tokens;
use pocket_ic::PocketIc;
use std::collections::HashMap;
use std::collections::HashSet;
use types::CanisterId;

use crate::wasms;

use candid::CandidType;
use candid::Deserialize;

#[derive(Deserialize, CandidType)]
pub struct Args {
    pub minting_account: String,
    pub initial_values: HashMap<String, Tokens>,
    pub send_whitelist: HashSet<CanisterId>,
    pub transfer_fee: Option<Tokens>,
}

pub fn setup_icp_ledger(
    pic: &mut PocketIc,
    controller: Principal,
    cycles_minting_init_args: Args,
) -> Principal {
    let sns_subnet_id = pic.topology().get_sns().unwrap();

    let icp_ledger_canister_id =
        pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet_id);
    pic.add_cycles(icp_ledger_canister_id, 200_000_000_000_000);
    pic.set_controllers(
        icp_ledger_canister_id,
        Some(controller.clone()),
        vec![controller.clone(), icp_ledger_canister_id.clone()],
    )
    .unwrap();
    pic.tick();

    pic.tick();
    let icp_ledger_canister_wasm = wasms::ICP_LEDGER.clone();
    pic.install_canister(
        icp_ledger_canister_id,
        icp_ledger_canister_wasm,
        encode_one(cycles_minting_init_args).unwrap(),
        Some(controller.clone()),
    );
    icp_ledger_canister_id
}
