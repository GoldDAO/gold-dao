use ic_cdk_macros::init;
use tracing::info;
use types::BuildVersion;
use utils::env::CanisterEnv;

use crate::state::{Data, RuntimeState};

use super::init_canister;
pub use gldt_stake_api_archive::lifecycle::Args;

#[init]
fn init(args: Args) {
    match args {
        Args::Init(init_args) => {
            canister_logger::init(init_args.test_mode);

            let env = CanisterEnv::new(
                init_args.test_mode,
                BuildVersion::min(),
                init_args.commit_hash,
            );
            let mut data = Data::default();

            data.authorized_principals = init_args.authorized_principals;

            let runtime_state = RuntimeState::new(env, data);

            init_canister(runtime_state);

            info!("Init complete.");
        }
        Args::Upgrade(_) => {
            panic!(
                "Cannot initialize the canister with an Upgrade argument. Please provide an Init argument."
            );
        }
    }
}
