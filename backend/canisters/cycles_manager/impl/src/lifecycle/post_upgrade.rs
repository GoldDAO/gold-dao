use crate::lifecycle::init_state;
use crate::memory::get_upgrades_memory;
use crate::Data;
use canister_logger::LogEntry;
use canister_tracing_macros::trace;
use ic_cdk_macros::post_upgrade;
use stable_memory::get_reader;
use tracing::info;

// #[post_upgrade]
// #[trace]
// fn post_upgrade() {
//     let memory = get_upgrades_memory();
//     let reader = get_reader(&memory);

//     let (data, logs, traces): (Data, Vec<LogEntry>, Vec<LogEntry>) =
//         serializer::deserialize(reader).unwrap();

//     canister_logger::init_with_logs(data.test_mode, logs, traces);

//     let env = init_env(data.rng_seed);
//     init_state(env, data);

//     info!("Post-upgrade complete");
// }
