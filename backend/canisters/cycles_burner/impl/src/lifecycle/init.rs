use canister_tracing_macros::trace;
use cycles_burner_canister::init::InitArgs;
use ic_cdk_macros::init;
use tracing::info;

#[init]
#[trace]
fn init() {
    crate::jobs::start();
    info!("Initialization complete");
}
