use ic_cdk_macros::init;

use crate::state::{ init_state, RuntimeState };

#[init]
fn init() {
    let runtime_state = RuntimeState::default();

    init_state(runtime_state)
}
