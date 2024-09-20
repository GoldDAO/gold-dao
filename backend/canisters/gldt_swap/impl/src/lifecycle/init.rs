use candid::Nat;
use ic_cdk_macros::init;
use tracing::info;
use utils::env::CanisterEnv;

use crate::state::{ Data, RuntimeState };

use super::init_canister;
pub use gldt_swap_api_canister::init::*;

#[init]
fn init(args: InitArgs) {
    canister_logger::init(args.test_mode);

    let env = CanisterEnv::new(args.test_mode);
    let mut data = Data::default();

    data.gldt_ledger_id = args.gldt_ledger_id;
    data.gldnft_canisters = args.gldnft_canisters
        .into_iter()
        .map(|(canister_id, config)| (canister_id, config, None))
        .collect();
    data.ogy_ledger_id = args.ogy_ledger_id;
    data.authorized_principals = args.authorized_principals;
    data.version = args.version;

    if args.test_mode {
        data.max_canister_archive_threshold = Nat::from(18 * 1024 * (1024 as u128)); // 18MB
    }

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
