use crate::lifecycle::init_canister;
use crate::Data;
use canister_tracing_macros::trace;
pub use cycles_manager_api_canister::Args;
use ic_cdk_macros::init;
use tracing::info;
use types::BuildVersion;
use utils::env::CanisterEnv;
use utils::env::Environment;

#[init]
#[trace]
fn init(args: Args) {
    match args {
        Args::Init(init_args) => {
            canister_logger::init(init_args.test_mode);
            let env = CanisterEnv::new(
                init_args.test_mode,
                BuildVersion::min(),
                init_args.commit_hash,
            );

            let data = Data::new(
                init_args.authorized_principals,
                init_args.canisters,
                init_args.sns_root_canister,
                init_args.max_top_up_amount,
                init_args.min_cycles_balance,
                init_args.icp_burn_amount,
                init_args.icp_ledger_canister,
                init_args.cycles_minting_canister,
                env.now(),
            );

            let state = crate::State::new(env, data);
            init_canister(state);

            info!("Initialization complete");
        }
        Args::Upgrade(_) => {
            panic!(
        "Cannot initialize the canister with an Upgrade argument. Please provide an Init argument."
    );
        }
    }
}
