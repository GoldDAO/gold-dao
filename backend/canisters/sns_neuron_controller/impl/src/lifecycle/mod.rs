pub mod init;
pub mod post_upgrade;
pub mod pre_upgrade;

use crate::state::{ init_state, RuntimeState };

pub use init::*;

pub fn init_canister(runtime_state: RuntimeState) {
    init_state(runtime_state);
    crate::jobs::start();
}
