use candid::Nat;
use gldt_stake_common::stake_position::MAX_STAKE_POSITION_SIZE;
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

            let mut data = Data {
                gldt_ledger_id: init_args.gldt_ledger_id,
                goldao_ledger_id: init_args.goldao_ledger_id,
                authorized_principals: init_args.authorized_principals,
                goldao_sns_rewards_canister_id: init_args.gld_sns_rewards_canister_id,
                goldao_sns_governance_canister_id: init_args.gld_sns_governance_canister_id,
                ..Default::default()
            };
            data.stake_system.reward_types = init_args.reward_types.clone();

            init_args.reward_types.iter().for_each(|(token_symbol, _)| {
                data.reward_system
                    .add_to_reward_history(token_symbol, Nat::from(0u64));
            });

            if init_args.test_mode {
                info!("INIT :: settingg max threshold to 32mb");
                data.archive_system.max_canister_archive_threshold = 32 * 1024 * 1024_u128;
            }

            info!("INIT  :: MAX position size {MAX_STAKE_POSITION_SIZE}");

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
