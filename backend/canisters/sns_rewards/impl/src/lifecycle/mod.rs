pub mod init;
mod post_upgrade;
mod pre_upgrade;

pub use init::*;

use crate::state::{init_state, RuntimeState};

pub fn init_canister_init(runtime_state: RuntimeState) {
    init_state(runtime_state);
    crate::jobs::start_init();
}

pub fn init_canister_upgrade(runtime_state: RuntimeState) {
    init_state(runtime_state);
    crate::jobs::start_upgrade();
}
