pub mod init;
pub mod post_upgrade;
pub mod pre_upgrade;

use crate::state::{init_state, RuntimeState};

pub fn init_canister(runtime_state: RuntimeState) {
    crate::jobs::start();
    init_state(runtime_state);
}
