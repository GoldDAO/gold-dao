pub mod init;
mod post_upgrade;
mod pre_upgrade;

pub use init::*;

use crate::state::{ init_state, RuntimeState };

pub fn init_canister(runtime_state: RuntimeState) {
    crate::jobs::start();
    init_state(runtime_state);
}
