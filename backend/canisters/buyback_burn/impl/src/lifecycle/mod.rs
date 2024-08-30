use crate::RuntimeState;

pub mod init;
mod post_upgrade;
mod pre_upgrade;

pub use init::*;

pub fn init_canister(state: RuntimeState) {
    crate::state::init_state(state);
    crate::jobs::start();
}
