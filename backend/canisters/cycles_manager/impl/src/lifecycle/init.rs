use crate::lifecycle::{init_env, init_state};
use crate::Data;
use canister_tracing_macros::trace;
use cycles_manager_canister::init::InitArgs;
use ic_cdk_macros::init;
use tracing::info;
use utils::env::Environment;

#[init]
#[trace]
fn init(args: InitArgs) {
    canister_logger::init(args.test_mode);

    let env = init_env([0; 32]);

    let data = Data::new(
        args.test_mode,
        args.authorized_principals,
        args.canisters,
        args.sns_root_canister,
        args.max_top_up_amount,
        args.min_interval,
        args.min_cycles_balance,
        env.now(),
    );

    init_state(env, data, args.wasm_version);

    info!(version = %args.wasm_version, "Initialization complete");
}
