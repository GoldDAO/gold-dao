use super::init_canister;
use crate::state::init_icrc3;
use crate::state::start_default_archive_job;
use crate::state::{Data, RuntimeState};
use candid::Nat;
pub use gldt_stake_api_canister::Args;
use ic_cdk_macros::init;
use tracing::info;
use types::TokenSymbol;
use utils::env::CanisterEnv;
use utils::numeric::Percentage;

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

            let mut data = Data {
                gldt_ledger_id: init_args.gldt_ledger_id,
                goldao_ledger_id: init_args.goldao_ledger_id,
                authorized_principals: init_args.authorized_principals,
                goldao_sns_rewards_canister_id: init_args.gld_sns_rewards_canister_id,
                goldao_sns_governance_canister_id: init_args.gld_sns_governance_canister_id,
                ..Default::default()
            };
            data.stake_system.apy_limit = match init_args.apy_limit {
                Some(value) => Some(Percentage::new(value).expect(
                    "Invalid APY limit in init args. It should be an integer between 0 and 100.",
                )),
                None => None,
            };
            data.stake_system.reward_types = init_args
                .allowed_reward_tokens
                .iter()
                .map(|token_symbol| {
                    TokenSymbol::parse(token_symbol)
                        .expect("Invalid token symbol in allowed rewards")
                })
                .collect();

            data.stake_system
                .reward_types
                .iter()
                .for_each(|token_symbol| {
                    data.allocated_rewards_pool
                        .add_to_reward_history(token_symbol, Nat::from(0u64));
                });

            let runtime_state = RuntimeState::new(env, data);

            init_canister(runtime_state);
            init_icrc3(init_args.icrc3_config);
            start_default_archive_job();

            info!("Init complete.")
        }
        Args::Upgrade(_) => {
            panic!(
                "Cannot initialize the canister with an Upgrade argument. Please provide an Init argument."
            );
        }
    }
}
