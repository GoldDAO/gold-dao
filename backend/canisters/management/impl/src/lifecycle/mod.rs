pub mod init;
mod post_upgrade;
mod pre_upgrade;

use crate::state::{ init_state, RuntimeState };

pub fn init_canister(runtime_state: RuntimeState) {
    init_state(runtime_state);
}
