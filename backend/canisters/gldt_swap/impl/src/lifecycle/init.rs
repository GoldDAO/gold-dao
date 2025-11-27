use crate::model::swap_configs::SwapConfigs;
use crate::state::init_icrc3;
use crate::state::start_default_archive_job;
use ic_cdk_macros::init;
use tracing::info;
use utils::env::CanisterEnv;

pub use gldt_swap_api_canister::Args;

use crate::state::{Data, RuntimeState};

use super::init_canister;

#[init]
fn init(args: Args) {
    match args {
        Args::Init(init_args) => {
            bity_ic_canister_logger::init(init_args.test_mode);

            if init_args.test_mode {
                info!("INIT :: in test mode.");
            }

            let env = CanisterEnv::new(
                init_args.test_mode,
                init_args.version,
                init_args.commit_hash,
            );
            let data = Data {
                authorized_principals: init_args.authorized_principals,
                swap_configs: SwapConfigs::from_vec(init_args.swap_configs),
                buyback_burn_canister: init_args.buyback_burn_canister,
                old_archive_canister: init_args.gldt_swap_old_archive,
                ..Default::default()
            };

            let runtime_state = RuntimeState::new(env, data);

            init_canister(runtime_state);
            init_icrc3(init_args.icrc3_config);
            start_default_archive_job();

            // NOTE: run only in real canister
            #[cfg(not(feature = "inttest"))]
            crate::migrations::jobs::migrate_old_swaps::start_job();

            info!("Init complete.")
        }
        Args::Upgrade(_) => {
            panic!(
                "Cannot initialize the canister with an Upgrade argument. Please provide an Init argument."
            );
        }
    }
}
