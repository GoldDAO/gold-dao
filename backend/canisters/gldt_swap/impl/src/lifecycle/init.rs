use candid::Nat;
use gldt_swap_common::swap::trace;
use ic_cdk_macros::init;
use tracing::{ debug, info };
use types::BuildVersion;
use utils::env::CanisterEnv;

pub use gldt_swap_api_canister::Args;

use crate::state::{ Data, RuntimeState };

use super::init_canister;

#[init]
fn init(args: Args) {
    match args {
        Args::Init(init_args) => {
            canister_logger::init(init_args.test_mode);

            let env = CanisterEnv::new(
                init_args.test_mode,
                init_args.version,
                init_args.commit_hash
            );
            let mut data = Data::default();

            data.gldt_ledger_id = init_args.gldt_ledger_id;
            data.gldnft_canisters = init_args.gldnft_canisters
                .into_iter()
                .map(|(canister_id, config)| (canister_id, config, None))
                .collect();
            data.ogy_ledger_id = init_args.ogy_ledger_id;
            data.authorized_principals = init_args.authorized_principals;

            // on staging - set a slighly higher threshold - based on a swap size of 2000 we expect around 4000~ swaps per page size ( per 8mb )
            if init_args.test_mode {
                debug!("INIT :: settingg max threshold to 64mb");
                data.max_canister_archive_threshold = 32 * 1024 * (1024 as u128); // 64M
                data.archive_buffer = 100;
            }

            let runtime_state = RuntimeState::new(env, data);

            init_canister(runtime_state);

            info!("Init complete.")
        }
        Args::Upgrade(_) => {
            panic!(
                "Cannot initialize the canister with an Upgrade argument. Please provide an Init argument."
            );
        }
    }
}
