// use crate::lifecycle::init_canister;
// use crate::Data;
// pub use buyback_burn_canister::init::InitArgs;
// use canister_tracing_macros::trace;
// use ic_cdk_macros::init;
// use tracing::info;
// use utils::env::CanisterEnv;

// #[init]
// #[trace]
// fn init(args: InitArgs) {
//     canister_logger::init(true);

//     let env = CanisterEnv::new(true);

//     let data = Data::new(
//         args.authorized_principals,
//         args.sns_governance_canister,
//         args.min_burn_amount,
//     );

//     let state = crate::State::new(env, data);
//     init_canister(state);

//     info!("Initialization complete");
// }
