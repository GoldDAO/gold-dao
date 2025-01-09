use ic_cdk_macros::init;
use tracing::info;
use utils::env::CanisterEnv;

pub use gldt_stake_api_canister::Args;

use crate::state::{Data, RuntimeState};

use super::init_canister;

#[init]
fn init(args: Args) {
    match args {
        Args::Init(init_args) => {
            canister_logger::init(init_args.test_mode);

            if init_args.test_mode {
                info!("INIT :: in test mode.");
            }

            let env = CanisterEnv::new(
                init_args.test_mode,
                init_args.version,
                init_args.commit_hash,
            );
            let mut data = Data::default();

            data.gldt_ledger_id = init_args.gldt_ledger_id;
            data.gldgov_ledger_id = init_args.gldgov_ledger_id;
            data.authorized_principals = init_args.authorized_principals;
            data.gld_sns_rewards_canister_id = init_args.gld_sns_rewards_canister_id;
            data.gld_sns_governance_canister_id = init_args.gld_sns_governance_canister_id;
            data.stake_system.reward_types = init_args.reward_types;

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
