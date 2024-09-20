use ic_cdk_macros::init;
use tracing::info;
use utils::env::CanisterEnv;

use crate::state::{ Data, RuntimeState };

use super::init_canister;
pub use gldt_swap_api_archive::init_archive::*;

#[init]
fn init(args: InitArgArchive) {
    canister_logger::init(args.test_mode);

    let env = CanisterEnv::new(args.test_mode);
    let mut data = Data::default();

    data.authorized_principals = args.authorized_principals;
    data.version = args.version;

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
